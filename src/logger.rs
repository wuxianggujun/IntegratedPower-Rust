// 日志系统
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
        }
    }

    pub fn color(&self) -> egui::Color32 {
        match self {
            LogLevel::Debug => egui::Color32::from_rgb(128, 128, 128),
            LogLevel::Info => egui::Color32::from_rgb(33, 150, 243),
            LogLevel::Warning => egui::Color32::from_rgb(255, 152, 0),
            LogLevel::Error => egui::Color32::from_rgb(244, 67, 54),
        }
    }
}

/// 日志条目
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: String) -> Self {
        Self {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            level,
            message,
        }
    }

    pub fn format(&self) -> String {
        format!("[{}] [{}] {}", self.timestamp, self.level.as_str(), self.message)
    }
}

/// 日志管理器
pub struct Logger {
    entries: Arc<Mutex<Vec<LogEntry>>>,
    max_entries: usize,
    log_file: Option<PathBuf>,
}

impl Logger {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
            max_entries,
            log_file: Self::get_log_file_path().ok(),
        }
    }

    fn get_log_file_path() -> Result<PathBuf, std::io::Error> {
        let log_dir = dirs::data_dir()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "无法获取数据目录"))?
            .join("IntegratedPower")
            .join("logs");

        std::fs::create_dir_all(&log_dir)?;

        let log_file = log_dir.join(format!(
            "app_{}.log",
            Local::now().format("%Y%m%d")
        ));

        Ok(log_file)
    }

    pub fn log(&self, level: LogLevel, message: impl Into<String>) {
        let entry = LogEntry::new(level, message.into());
        
        // 添加到内存
        if let Ok(mut entries) = self.entries.lock() {
            entries.push(entry.clone());
            
            // 限制条目数量
            if entries.len() > self.max_entries {
                entries.remove(0);
            }
        }

        // 写入文件
        if let Some(log_file) = &self.log_file {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file)
            {
                let _ = writeln!(file, "{}", entry.format());
            }
        }

        // 同时输出到控制台
        println!("{}", entry.format());
    }

    pub fn debug(&self, message: impl Into<String>) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&self, message: impl Into<String>) {
        self.log(LogLevel::Info, message);
    }

    pub fn warning(&self, message: impl Into<String>) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&self, message: impl Into<String>) {
        self.log(LogLevel::Error, message);
    }

    pub fn get_entries(&self) -> Vec<LogEntry> {
        self.entries.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        if let Ok(mut entries) = self.entries.lock() {
            entries.clear();
        }
    }

    pub fn get_log_file_path_str(&self) -> Option<String> {
        self.log_file.as_ref().map(|p| p.display().to_string())
    }
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Self {
            entries: Arc::clone(&self.entries),
            max_entries: self.max_entries,
            log_file: self.log_file.clone(),
        }
    }
}

// 全局日志实例
lazy_static::lazy_static! {
    pub static ref LOGGER: Logger = Logger::new(1000);
}

// 便捷宏
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logger::LOGGER.debug(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logger::LOGGER.info(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        $crate::logger::LOGGER.warning(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logger::LOGGER.error(format!($($arg)*))
    };
}
