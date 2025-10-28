use crate::error::{AppError, Result};
use crate::models::{ProcessingProgress, ProcessingStats};
use polars::prelude::*;
use std::path::Path;
use tokio::task;

/// 数据处理引擎
pub struct DataEngine;

impl DataEngine {
    /// 读取 Excel 文件为 DataFrame（使用 umya-spreadsheet）
    pub fn read_excel(path: &Path) -> Result<DataFrame> {
        tracing::debug!("读取 Excel 文件: {}", path.display());

        let book = umya_spreadsheet::reader::xlsx::read(path)
            .map_err(|e| AppError::excel_error(format!("无法打开文件: {}", e)))?;

        // 取第一个工作表
        let sheets = book.get_sheet_collection();
        if sheets.is_empty() {
            return Err(AppError::excel_error("Excel 文件中没有工作表"));
        }
        let ws = &sheets[0];

        // 计算范围（dimension）
        let (height, width) = Self::worksheet_size(ws);
        if height == 0 || width == 0 {
            return Err(AppError::excel_error("工作表为空"));
        }

        // 读取表头（第一行）
        let headers: Vec<String> = (1..=width)
            .map(|col| ws.get_value((col, 1)))
            .enumerate()
            .map(|(i, v)| if v.is_empty() { format!("Column_{}", i) } else { v })
            .collect();

        // 读取数据
        let mut columns: Vec<Column> = Vec::new();
        for (col_idx, header) in headers.iter().enumerate() {
            let mut values: Vec<String> = Vec::new();
            for row in 2..=height {
                let v = ws.get_value(((col_idx + 1) as u32, row));
                values.push(v);
            }
            let series = Series::new(header.as_str().into(), values);
            columns.push(series.into_column());
        }

        DataFrame::new(columns)
            .map_err(|e| AppError::polars_error(format!("创建 DataFrame 失败: {}", e)))
    }

    fn worksheet_size(ws: &umya_spreadsheet::Worksheet) -> (u32, u32) {
        let rows = ws.get_highest_row();
        let cols = ws.get_highest_column();
        (rows, cols)
    }

    /// 写入 DataFrame 到 Excel 文件
    pub fn write_excel(df: &DataFrame, path: &Path) -> Result<()> {
        tracing::debug!("写入 Excel 文件: {}", path.display());

        // 将 DataFrame 转换为 CSV 格式（临时方案）
        // 注意：Polars 原生不支持直接写入 Excel，这里使用 CSV 作为中间格式
        // 在实际应用中，可以使用 rust_xlsxwriter 等库来写入真正的 Excel 文件

        let mut df_mut = df.clone();
        let mut file = std::fs::File::create(path)?;
        CsvWriter::new(&mut file)
            .finish(&mut df_mut)
            .map_err(|e| AppError::polars_error(format!("写入文件失败: {}", e)))?;

        Ok(())
    }

    /// 批量处理文件
    pub async fn process_batch<F, P>(
        input_dir: &Path,
        output_dir: &Path,
        processor: P,
        progress_callback: F,
    ) -> Result<ProcessingStats>
    where
        F: Fn(ProcessingProgress) + Send + Sync + 'static,
        P: Fn(DataFrame) -> Result<DataFrame> + Send + Sync + 'static + Clone,
    {
        tracing::info!(
            "开始批量处理: 输入={}, 输出={}",
            input_dir.display(),
            output_dir.display()
        );

        // 验证目录
        if !input_dir.exists() {
            return Err(AppError::DirectoryNotFound(input_dir.to_path_buf()));
        }

        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)?;
        }

        // 扫描输入目录中的所有 xlsx 文件
        let files = Self::scan_xlsx_files(input_dir)?;
        let total_files = files.len();

        if total_files == 0 {
            tracing::warn!("输入目录中没有找到 xlsx 文件");
            return Ok(ProcessingStats::new());
        }

        tracing::info!("找到 {} 个文件待处理", total_files);

        let mut stats = ProcessingStats::new();
        let start_time = std::time::Instant::now();

        // 处理每个文件
        for (idx, file_path) in files.iter().enumerate() {
            let file_name = file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            // 更新进度
            let mut progress = ProcessingProgress::new(total_files);
            progress.update(idx, file_name.to_string());
            progress_callback(progress.clone());

            // 处理文件
            let output_path = output_dir.join(file_name);
            let processor_clone = processor.clone();

            match Self::process_single_file(file_path, &output_path, processor_clone).await {
                Ok(_) => {
                    stats.files_succeeded += 1;
                    tracing::info!("成功处理: {}", file_name);
                }
                Err(e) => {
                    stats.files_failed += 1;
                    tracing::error!("处理失败 {}: {}", file_name, e);
                }
            }

            stats.files_processed += 1;
        }

        stats.total_duration = start_time.elapsed();

        // 最终进度更新
        let mut final_progress = ProcessingProgress::new(total_files);
        final_progress.update(total_files, "完成".to_string());
        progress_callback(final_progress);

        tracing::info!(
            "批量处理完成: 成功={}, 失败={}, 耗时={:?}",
            stats.files_succeeded,
            stats.files_failed,
            stats.total_duration
        );

        Ok(stats)
    }

    /// 处理单个文件
    async fn process_single_file<P>(
        input_path: &Path,
        output_path: &Path,
        processor: P,
    ) -> Result<()>
    where
        P: Fn(DataFrame) -> Result<DataFrame> + Send + Sync + 'static,
    {
        let input_path = input_path.to_path_buf();
        let output_path = output_path.to_path_buf();

        // 在独立任务中处理文件
        task::spawn_blocking(move || {
            // 读取文件
            let df = Self::read_excel(&input_path)?;

            // 应用处理器
            let processed_df = processor(df)?;

            // 写入结果
            Self::write_excel(&processed_df, &output_path)?;

            Ok::<(), AppError>(())
        })
        .await
        .map_err(|e| AppError::processing_error(format!("任务执行失败: {}", e)))??;

        Ok(())
    }

    /// 扫描目录中的所有 xlsx 文件
    fn scan_xlsx_files(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
        let mut files = Vec::new();

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.eq_ignore_ascii_case("xlsx") {
                        files.push(path);
                    }
                }
            }
        }

        files.sort();
        Ok(files)
    }

    /// 并行批量处理文件
    pub async fn process_batch_parallel<F, P>(
        input_dir: &Path,
        output_dir: &Path,
        processor: P,
        progress_callback: F,
        max_parallel: usize,
    ) -> Result<ProcessingStats>
    where
        F: Fn(ProcessingProgress) + Send + Sync + 'static + Clone,
        P: Fn(DataFrame) -> Result<DataFrame> + Send + Sync + 'static + Clone,
    {
        tracing::info!(
            "开始并行批量处理: 输入={}, 输出={}, 并行数={}",
            input_dir.display(),
            output_dir.display(),
            max_parallel
        );

        // 验证目录
        if !input_dir.exists() {
            return Err(AppError::DirectoryNotFound(input_dir.to_path_buf()));
        }

        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)?;
        }

        // 扫描文件
        let files = Self::scan_xlsx_files(input_dir)?;
        let total_files = files.len();

        if total_files == 0 {
            tracing::warn!("输入目录中没有找到 xlsx 文件");
            return Ok(ProcessingStats::new());
        }

        tracing::info!("找到 {} 个文件待处理", total_files);

        let mut stats = ProcessingStats::new();
        let start_time = std::time::Instant::now();

        // 使用信号量限制并发数
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(max_parallel));
        let processed_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let success_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let failure_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let mut tasks = Vec::new();

        for file_path in files {
            let file_name = file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            let output_path = output_dir.join(&file_name);
            let processor = processor.clone();
            let progress_callback = progress_callback.clone();
            let semaphore = semaphore.clone();
            let processed_count = processed_count.clone();
            let success_count = success_count.clone();
            let failure_count = failure_count.clone();

            let task = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();

                let result = Self::process_single_file(&file_path, &output_path, processor).await;

                let processed = processed_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;

                match result {
                    Ok(_) => {
                        success_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        tracing::info!("成功处理: {}", file_name);
                    }
                    Err(e) => {
                        failure_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        tracing::error!("处理失败 {}: {}", file_name, e);
                    }
                }

                // 更新进度
                let mut progress = ProcessingProgress::new(total_files);
                progress.update(processed, file_name);
                progress_callback(progress);
            });

            tasks.push(task);
        }

        // 等待所有任务完成
        for task in tasks {
            let _ = task.await;
        }

        stats.files_processed = total_files;
        stats.files_succeeded = success_count.load(std::sync::atomic::Ordering::SeqCst);
        stats.files_failed = failure_count.load(std::sync::atomic::Ordering::SeqCst);
        stats.total_duration = start_time.elapsed();

        tracing::info!(
            "并行批量处理完成: 成功={}, 失败={}, 耗时={:?}",
            stats.files_succeeded,
            stats.files_failed,
            stats.total_duration
        );

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_scan_xlsx_files() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // 创建测试文件
        std::fs::write(dir_path.join("test1.xlsx"), b"").unwrap();
        std::fs::write(dir_path.join("test2.xlsx"), b"").unwrap();
        std::fs::write(dir_path.join("test.txt"), b"").unwrap();

        let files = DataEngine::scan_xlsx_files(dir_path).unwrap();
        assert_eq!(files.len(), 2);
    }

    #[tokio::test]
    async fn test_process_batch_empty_dir() {
        let input_dir = tempdir().unwrap();
        let output_dir = tempdir().unwrap();

        let stats = DataEngine::process_batch(
            input_dir.path(),
            output_dir.path(),
            |df| Ok(df),
            |_| {},
        )
        .await
        .unwrap();

        assert_eq!(stats.files_processed, 0);
    }
}
