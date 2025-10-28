// Row Type Identification Error Types
use thiserror::Error;

/// 行类型识别相关错误
#[derive(Debug, Error)]
pub enum IdentificationError {
    /// 无效的颜色数据
    #[error("Invalid color data in row {row}: {details}")]
    InvalidColorData { row: usize, details: String },

    /// 无效的正则表达式模式
    #[error("Invalid regex pattern in rule '{rule}': {details}")]
    InvalidRegexPattern { rule: String, details: String },

    /// 配置中没有行类型定义
    #[error("No row type definitions in profile '{profile}'")]
    EmptyProfile { profile: String },

    /// 无效的配置
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Excel数据访问错误
    #[error("Excel data access error: {0}")]
    ExcelDataError(String),

    /// 文件读取错误
    #[error("File read error: {0}")]
    FileReadError(String),

    /// 工作表不存在
    #[error("Worksheet not found: {0}")]
    WorksheetNotFound(String),

    /// 行索引超出范围
    #[error("Row index {row} out of range (max: {max})")]
    RowIndexOutOfRange { row: usize, max: usize },

    /// 列索引超出范围
    #[error("Column index {col} out of range (max: {max})")]
    ColumnIndexOutOfRange { col: usize, max: usize },
}

impl IdentificationError {
    /// 创建无效颜色数据错误
    pub fn invalid_color_data(row: usize, details: impl Into<String>) -> Self {
        Self::InvalidColorData {
            row,
            details: details.into(),
        }
    }

    /// 创建无效正则表达式错误
    pub fn invalid_regex_pattern(rule: impl Into<String>, details: impl Into<String>) -> Self {
        Self::InvalidRegexPattern {
            rule: rule.into(),
            details: details.into(),
        }
    }

    /// 创建空配置错误
    pub fn empty_profile(profile: impl Into<String>) -> Self {
        Self::EmptyProfile {
            profile: profile.into(),
        }
    }

    /// 创建无效配置错误
    pub fn invalid_configuration(msg: impl Into<String>) -> Self {
        Self::InvalidConfiguration(msg.into())
    }

    /// 创建Excel数据访问错误
    pub fn excel_data_error(msg: impl Into<String>) -> Self {
        Self::ExcelDataError(msg.into())
    }

    /// 创建文件读取错误
    pub fn file_read_error(msg: impl Into<String>) -> Self {
        Self::FileReadError(msg.into())
    }

    /// 创建工作表不存在错误
    pub fn worksheet_not_found(name: impl Into<String>) -> Self {
        Self::WorksheetNotFound(name.into())
    }

    /// 创建行索引超出范围错误
    pub fn row_index_out_of_range(row: usize, max: usize) -> Self {
        Self::RowIndexOutOfRange { row, max }
    }

    /// 创建列索引超出范围错误
    pub fn column_index_out_of_range(col: usize, max: usize) -> Self {
        Self::ColumnIndexOutOfRange { col, max }
    }
}

/// 识别结果类型别名
pub type IdentificationResult<T> = Result<T, IdentificationError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = IdentificationError::invalid_color_data(10, "Invalid RGB value");
        assert!(err.to_string().contains("row 10"));
        assert!(err.to_string().contains("Invalid RGB value"));

        let err = IdentificationError::invalid_regex_pattern("test_rule", "Unclosed bracket");
        assert!(err.to_string().contains("test_rule"));
        assert!(err.to_string().contains("Unclosed bracket"));

        let err = IdentificationError::empty_profile("cargo_analysis");
        assert!(err.to_string().contains("cargo_analysis"));

        let err = IdentificationError::row_index_out_of_range(100, 50);
        assert!(err.to_string().contains("100"));
        assert!(err.to_string().contains("50"));
    }

    #[test]
    fn test_error_display() {
        let err = IdentificationError::InvalidConfiguration("Test error".to_string());
        let display = format!("{}", err);
        assert_eq!(display, "Invalid configuration: Test error");
    }
}
