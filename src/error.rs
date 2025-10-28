use std::path::PathBuf;
use thiserror::Error;

/// 应用程序错误类型
#[derive(Debug, Error)]
pub enum AppError {
    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("文件 I/O 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("数据处理错误: {0}")]
    ProcessingError(String),

    #[error("无效的目录: {0}")]
    InvalidDirectory(PathBuf),

    #[error("处理器未找到: {0}")]
    ProcessorNotFound(String),

    #[error("Excel 文件错误: {0}")]
    ExcelError(String),

    #[error("TOML 反序列化错误: {0}")]
    TomlDeserializeError(#[from] toml::de::Error),

    #[error("TOML 序列化错误: {0}")]
    TomlSerializeError(#[from] toml::ser::Error),

    #[error("历史记录错误: {0}")]
    HistoryError(String),

    #[error("目录不存在: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("目录不可读: {0}")]
    DirectoryNotReadable(PathBuf),

    #[error("目录不可写: {0}")]
    DirectoryNotWritable(PathBuf),

    #[error("磁盘空间不足")]
    InsufficientDiskSpace,

    #[error("操作已取消")]
    OperationCancelled,

    #[error("Polars 错误: {0}")]
    PolarsError(String),
}

/// Result 类型别名
pub type Result<T> = std::result::Result<T, AppError>;

impl AppError {
    /// 创建配置错误
    pub fn config_error(msg: impl Into<String>) -> Self {
        Self::ConfigError(msg.into())
    }

    /// 创建处理错误
    pub fn processing_error(msg: impl Into<String>) -> Self {
        Self::ProcessingError(msg.into())
    }

    /// 创建 Excel 错误
    pub fn excel_error(msg: impl Into<String>) -> Self {
        Self::ExcelError(msg.into())
    }

    /// 创建历史记录错误
    pub fn history_error(msg: impl Into<String>) -> Self {
        Self::HistoryError(msg.into())
    }

    /// 创建 Polars 错误
    pub fn polars_error(msg: impl Into<String>) -> Self {
        Self::PolarsError(msg.into())
    }

    /// 检查是否为 I/O 错误
    pub fn is_io_error(&self) -> bool {
        matches!(self, Self::IoError(_))
    }

    /// 检查是否为配置错误
    pub fn is_config_error(&self) -> bool {
        matches!(self, Self::ConfigError(_))
    }

    /// 检查是否为处理错误
    pub fn is_processing_error(&self) -> bool {
        matches!(self, Self::ProcessingError(_))
    }

    /// 获取用户友好的错误消息
    pub fn user_message(&self) -> String {
        match self {
            Self::ConfigError(msg) => format!("配置错误: {}", msg),
            Self::IoError(e) => format!("文件操作失败: {}", e),
            Self::ProcessingError(msg) => format!("处理失败: {}", msg),
            Self::InvalidDirectory(path) => format!("无效的目录: {}", path.display()),
            Self::ProcessorNotFound(id) => format!("未找到处理器: {}", id),
            Self::ExcelError(msg) => format!("Excel 文件错误: {}", msg),
            Self::DirectoryNotFound(path) => format!("目录不存在: {}", path.display()),
            Self::DirectoryNotReadable(path) => format!("目录不可读: {}", path.display()),
            Self::DirectoryNotWritable(path) => format!("目录不可写: {}", path.display()),
            Self::InsufficientDiskSpace => "磁盘空间不足".to_string(),
            Self::OperationCancelled => "操作已取消".to_string(),
            Self::HistoryError(msg) => format!("历史记录错误: {}", msg),
            Self::PolarsError(msg) => format!("数据处理错误: {}", msg),
            Self::TomlDeserializeError(e) => format!("配置文件解析失败: {}", e),
            Self::TomlSerializeError(e) => format!("配置文件保存失败: {}", e),
        }
    }
}
