use crate::error::{AppError, Result};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;

/// 错误日志记录器
pub struct ErrorLogger {
    log_file: PathBuf,
}

impl ErrorLogger {
    /// 创建新的错误日志记录器
    pub fn new(log_file: PathBuf) -> Result<Self> {
        // 确保日志文件的父目录存在
        if let Some(parent) = log_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(Self { log_file })
    }

    /// 记录错误
    pub fn log_error(&self, error: &AppError, context: &str) -> Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!(
            "[{}] [ERROR] {}: {}\n",
            timestamp,
            context,
            error
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)?;

        file.write_all(log_entry.as_bytes())?;
        file.flush()?;

        tracing::error!("{}: {}", context, error);

        Ok(())
    }

    /// 记录警告
    pub fn log_warning(&self, message: &str, context: &str) -> Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!(
            "[{}] [WARN] {}: {}\n",
            timestamp,
            context,
            message
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)?;

        file.write_all(log_entry.as_bytes())?;
        file.flush()?;

        tracing::warn!("{}: {}", context, message);

        Ok(())
    }

    /// 记录信息
    pub fn log_info(&self, message: &str, context: &str) -> Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!(
            "[{}] [INFO] {}: {}\n",
            timestamp,
            context,
            message
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)?;

        file.write_all(log_entry.as_bytes())?;
        file.flush()?;

        tracing::info!("{}: {}", context, message);

        Ok(())
    }

    /// 获取最近的错误日志
    pub fn get_recent_errors(&self, count: usize) -> Result<Vec<String>> {
        let content = std::fs::read_to_string(&self.log_file)?;
        let lines: Vec<String> = content
            .lines()
            .filter(|line| line.contains("[ERROR]"))
            .rev()
            .take(count)
            .map(|s| s.to_string())
            .collect();

        Ok(lines)
    }

    /// 清除日志文件
    pub fn clear_log(&self) -> Result<()> {
        File::create(&self.log_file)?;
        Ok(())
    }

    /// 获取日志文件路径
    pub fn log_file_path(&self) -> &PathBuf {
        &self.log_file
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_error_logger_creation() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("test.log");
        let logger = ErrorLogger::new(log_path.clone()).unwrap();
        assert_eq!(logger.log_file_path(), &log_path);
    }

    #[test]
    fn test_log_error() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("test.log");
        let logger = ErrorLogger::new(log_path.clone()).unwrap();

        let error = AppError::config_error("Test error");
        logger.log_error(&error, "Test context").unwrap();

        let content = std::fs::read_to_string(&log_path).unwrap();
        assert!(content.contains("[ERROR]"));
        assert!(content.contains("Test context"));
        assert!(content.contains("Test error"));
    }

    #[test]
    fn test_get_recent_errors() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("test.log");
        let logger = ErrorLogger::new(log_path.clone()).unwrap();

        // 记录多个错误
        for i in 0..5 {
            let error = AppError::config_error(format!("Error {}", i));
            logger.log_error(&error, "Test").unwrap();
        }

        let recent = logger.get_recent_errors(3).unwrap();
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_clear_log() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("test.log");
        let logger = ErrorLogger::new(log_path.clone()).unwrap();

        let error = AppError::config_error("Test error");
        logger.log_error(&error, "Test").unwrap();

        logger.clear_log().unwrap();

        let content = std::fs::read_to_string(&log_path).unwrap();
        assert!(content.is_empty());
    }
}
