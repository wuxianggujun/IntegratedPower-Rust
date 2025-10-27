use crate::config::manager::ConfigManager;
use crate::error::AppError;
use crate::history::manager::HistoryManager;
use crate::processor::manager::ProcessorManager;

/// 主应用程序结构
pub struct Application {
    config_manager: ConfigManager,
    processor_manager: ProcessorManager,
    history_manager: HistoryManager,
}

impl Application {
    /// 创建新的应用程序实例
    pub fn new() -> Result<Self, AppError> {
        tracing::info!("初始化 IntegratedPower 应用程序");

        // 加载配置
        let config_manager = ConfigManager::load()?;

        // 创建处理器管理器
        let processor_manager = ProcessorManager::new();

        // 创建历史记录管理器
        let history_manager = HistoryManager::new(config_manager.config().max_history_entries)?;

        Ok(Self {
            config_manager,
            processor_manager,
            history_manager,
        })
    }

    /// 运行应用程序
    pub fn run(&mut self) -> Result<(), AppError> {
        tracing::info!("启动应用程序");

        // 注册示例处理器
        use crate::processor::examples::{DataCleaningProcessor, DataStatisticsProcessor};
        use std::sync::Arc;
        
        self.processor_manager.register_processor(Arc::new(DataCleaningProcessor::new()));
        self.processor_manager.register_processor(Arc::new(DataStatisticsProcessor::new()));

        tracing::info!("已注册 {} 个处理器", self.processor_manager.processor_count());

        // 创建主控制器
        // 将管理器安全地移动到控制器中，避免对 Default 的要求
        let processor_manager = std::mem::replace(&mut self.processor_manager, ProcessorManager::new());
        let config_manager = std::mem::replace(&mut self.config_manager, ConfigManager::load()?);
        let max_entries = self.history_manager.max_entries();
        let history_manager = std::mem::replace(&mut self.history_manager, HistoryManager::new(max_entries)?);

        let controller = crate::controller::main_controller::MainController::new(
            processor_manager,
            config_manager,
            history_manager,
        );

        // 启动 Qt GUI
        use crate::ui::qt_app::QtApp;
        let qt_app = QtApp::new(controller);
        qt_app.run()?;

        Ok(())
    }
}
