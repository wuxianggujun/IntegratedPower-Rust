use crate::config::manager::ConfigManager;
use crate::error::{AppError, Result};
use crate::history::manager::{HistoryEntry, HistoryManager};
use crate::models::{AppState, ProcessingProgress, ProcessingResult};
use crate::processor::{ProcessorInfo, ProcessorManager};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

/// 主控制器
pub struct MainController {
    processor_manager: Arc<ProcessorManager>,
    config_manager: Arc<RwLock<ConfigManager>>,
    history_manager: Arc<RwLock<HistoryManager>>,
    current_state: Arc<RwLock<AppState>>,
}

impl MainController {
    /// 创建新的主控制器
    pub fn new(
        processor_manager: ProcessorManager,
        config_manager: ConfigManager,
        history_manager: HistoryManager,
    ) -> Self {
        Self {
            processor_manager: Arc::new(processor_manager),
            config_manager: Arc::new(RwLock::new(config_manager)),
            history_manager: Arc::new(RwLock::new(history_manager)),
            current_state: Arc::new(RwLock::new(AppState::Idle)),
        }
    }

    /// 执行处理器
    pub async fn execute_processor<F>(
        &self,
        processor_id: &str,
        input_dir: PathBuf,
        output_dir: PathBuf,
        progress_callback: F,
    ) -> Result<ProcessingResult>
    where
        F: Fn(ProcessingProgress) + Send + Sync + 'static + Clone,
    {
        // 检查当前状态
        {
            let state = self.current_state.read().await;
            if state.is_processing() {
                return Err(AppError::processing_error("已有处理任务正在运行"));
            }
        }

        // 获取处理器
        let processor = self
            .processor_manager
            .get_processor(processor_id)
            .ok_or_else(|| AppError::ProcessorNotFound(processor_id.to_string()))?;

        let processor_name = processor.name().to_string();

        tracing::info!(
            "开始执行处理器: {} ({})",
            processor_name,
            processor_id
        );

        // 验证目录
        Self::validate_directories(&input_dir, &output_dir)?;

        // 创建取消令牌
        let cancel_token = CancellationToken::new();

        // 更新状态为处理中
        {
            let mut state = self.current_state.write().await;
            *state = AppState::processing(processor_id.to_string(), cancel_token.clone());
        }

        // 获取配置
        let config = self.config_manager.read().await;
        let parallel_processing = config.config().parallel_processing;
        let max_parallel_tasks = config.config().max_parallel_tasks;
        drop(config);

        // 执行处理
        let start_time = std::time::Instant::now();
        let result = if parallel_processing {
            self.execute_parallel(
                processor,
                &input_dir,
                &output_dir,
                progress_callback,
                max_parallel_tasks,
                cancel_token.clone(),
            )
            .await
        } else {
            self.execute_sequential(
                processor,
                &input_dir,
                &output_dir,
                progress_callback,
                cancel_token.clone(),
            )
            .await
        };

        let duration = start_time.elapsed();

        // 更新状态为空闲
        {
            let mut state = self.current_state.write().await;
            *state = AppState::Idle;
        }

        match result {
            Ok(mut processing_result) => {
                processing_result.set_duration(duration);

                // 添加到历史记录
                let entry = HistoryEntry::new(
                    processor_id.to_string(),
                    processor_name,
                    input_dir,
                    output_dir,
                    processing_result.clone(),
                );

                let mut history = self.history_manager.write().await;
                history.add_entry(entry)?;

                tracing::info!("处理完成: {:?}", processing_result);

                Ok(processing_result)
            }
            Err(e) => {
                tracing::error!("处理失败: {}", e);

                // 更新状态为错误
                {
                    let mut state = self.current_state.write().await;
                    *state = AppState::error(e.to_string());
                }

                Err(e)
            }
        }
    }

    /// 串行执行处理
    async fn execute_sequential<F>(
        &self,
        processor: Arc<dyn crate::processor::DataProcessor>,
        input_dir: &PathBuf,
        output_dir: &PathBuf,
        progress_callback: F,
        cancel_token: CancellationToken,
    ) -> Result<ProcessingResult>
    where
        F: Fn(ProcessingProgress) + Send + Sync + 'static,
    {
        use crate::engine::data_engine::DataEngine;

        let processor_clone = processor.clone();
        let input_dir = input_dir.clone();
        let output_dir = output_dir.clone();

        let stats = DataEngine::process_batch(
            &input_dir,
            &output_dir,
            move |df| {
                // 检查是否取消
                if cancel_token.is_cancelled() {
                    return Err(AppError::OperationCancelled);
                }

                // 使用 tokio 运行时执行异步处理
                let rt = tokio::runtime::Handle::current();
                rt.block_on(processor_clone.process(df))
            },
            progress_callback,
        )
        .await?;

        let mut result = ProcessingResult::new(stats.files_processed);
        result.successful = stats.files_succeeded;
        result.failed = stats.files_failed;

        Ok(result)
    }

    /// 并行执行处理
    async fn execute_parallel<F>(
        &self,
        processor: Arc<dyn crate::processor::DataProcessor>,
        input_dir: &PathBuf,
        output_dir: &PathBuf,
        progress_callback: F,
        max_parallel: usize,
        cancel_token: CancellationToken,
    ) -> Result<ProcessingResult>
    where
        F: Fn(ProcessingProgress) + Send + Sync + 'static + Clone,
    {
        use crate::engine::data_engine::DataEngine;

        let processor_clone = processor.clone();
        let input_dir = input_dir.clone();
        let output_dir = output_dir.clone();

        let stats = DataEngine::process_batch_parallel(
            &input_dir,
            &output_dir,
            move |df| {
                // 检查是否取消
                if cancel_token.is_cancelled() {
                    return Err(AppError::OperationCancelled);
                }

                let rt = tokio::runtime::Handle::current();
                rt.block_on(processor_clone.process(df))
            },
            progress_callback,
            max_parallel,
        )
        .await?;

        let mut result = ProcessingResult::new(stats.files_processed);
        result.successful = stats.files_succeeded;
        result.failed = stats.files_failed;

        Ok(result)
    }

    /// 取消处理
    pub async fn cancel_processing(&self) -> Result<()> {
        let state = self.current_state.read().await;

        if let Some(cancel_token) = state.cancel_token() {
            tracing::info!("取消处理任务");
            cancel_token.cancel();
            Ok(())
        } else {
            Err(AppError::processing_error("没有正在运行的处理任务"))
        }
    }

    /// 获取可用的处理器列表
    pub fn get_available_processors(&self) -> Vec<ProcessorInfo> {
        self.processor_manager.list_available_processors()
    }

    /// 获取所有处理器列表
    pub fn get_all_processors(&self) -> Vec<ProcessorInfo> {
        self.processor_manager.list_processors()
    }

    /// 获取当前状态
    pub async fn get_current_state(&self) -> AppState {
        self.current_state.read().await.clone()
    }

    /// 检查是否正在处理
    pub async fn is_processing(&self) -> bool {
        self.current_state.read().await.is_processing()
    }

    /// 获取配置管理器
    pub fn config_manager(&self) -> Arc<RwLock<ConfigManager>> {
        self.config_manager.clone()
    }

    /// 获取历史管理器
    pub fn history_manager(&self) -> Arc<RwLock<HistoryManager>> {
        self.history_manager.clone()
    }

    /// 获取处理器管理器
    pub fn processor_manager(&self) -> Arc<ProcessorManager> {
        self.processor_manager.clone()
    }

    /// 验证目录
    fn validate_directories(input_dir: &PathBuf, output_dir: &PathBuf) -> Result<()> {
        // 验证输入目录
        if !input_dir.exists() {
            return Err(AppError::DirectoryNotFound(input_dir.clone()));
        }

        if !input_dir.is_dir() {
            return Err(AppError::InvalidDirectory(input_dir.clone()));
        }

        // 检查输入目录是否可读
        if input_dir.read_dir().is_err() {
            return Err(AppError::DirectoryNotReadable(input_dir.clone()));
        }

        // 验证输出目录（如果不存在则创建）
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)?;
        }

        if !output_dir.is_dir() {
            return Err(AppError::InvalidDirectory(output_dir.clone()));
        }

        // 检查输出目录是否可写
        let test_file = output_dir.join(".write_test");
        if std::fs::write(&test_file, b"test").is_err() {
            return Err(AppError::DirectoryNotWritable(output_dir.clone()));
        }
        let _ = std::fs::remove_file(test_file);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::examples::{DataCleaningProcessor, DataStatisticsProcessor};

    #[tokio::test]
    async fn test_controller_creation() {
        let processor_manager = ProcessorManager::new();
        let config_manager = ConfigManager::load().unwrap();
        let history_manager = HistoryManager::new(100).unwrap();

        let controller = MainController::new(processor_manager, config_manager, history_manager);

        assert!(!controller.is_processing().await);
    }

    #[tokio::test]
    async fn test_get_processors() {
        let mut processor_manager = ProcessorManager::new();
        processor_manager.register_processor(Arc::new(DataCleaningProcessor::new()));
        processor_manager.register_processor(Arc::new(DataStatisticsProcessor::new()));

        let config_manager = ConfigManager::load().unwrap();
        let history_manager = HistoryManager::new(100).unwrap();

        let controller = MainController::new(processor_manager, config_manager, history_manager);

        let processors = controller.get_all_processors();
        assert_eq!(processors.len(), 2);
    }

    #[test]
    fn test_validate_directories() {
        use tempfile::tempdir;

        let input_dir = tempdir().unwrap();
        let output_dir = tempdir().unwrap();

        let result = MainController::validate_directories(
            &input_dir.path().to_path_buf(),
            &output_dir.path().to_path_buf(),
        );

        assert!(result.is_ok());
    }
}
