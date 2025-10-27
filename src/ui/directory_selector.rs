use crate::error::{AppError, Result};
use std::path::PathBuf;

/// 目录选择器
pub struct DirectorySelector {
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
}

impl DirectorySelector {
    /// 创建新的目录选择器
    pub fn new() -> Self {
        Self {
            input_path: None,
            output_path: None,
        }
    }

    /// 设置输入目录
    pub fn set_input_directory(&mut self, path: PathBuf) -> Result<()> {
        self.validate_input_directory(&path)?;
        self.input_path = Some(path);
        Ok(())
    }

    /// 设置输出目录
    pub fn set_output_directory(&mut self, path: PathBuf) -> Result<()> {
        self.validate_output_directory(&path)?;
        self.output_path = Some(path);
        Ok(())
    }

    /// 获取输入目录
    pub fn input_directory(&self) -> Option<&PathBuf> {
        self.input_path.as_ref()
    }

    /// 获取输出目录
    pub fn output_directory(&self) -> Option<&PathBuf> {
        self.output_path.as_ref()
    }

    /// 清除输入目录
    pub fn clear_input_directory(&mut self) {
        self.input_path = None;
    }

    /// 清除输出目录
    pub fn clear_output_directory(&mut self) {
        self.output_path = None;
    }

    /// 清除所有目录
    pub fn clear_all(&mut self) {
        self.input_path = None;
        self.output_path = None;
    }

    /// 验证输入目录
    fn validate_input_directory(&self, path: &PathBuf) -> Result<()> {
        // 检查目录是否存在
        if !path.exists() {
            return Err(AppError::DirectoryNotFound(path.clone()));
        }

        // 检查是否为目录
        if !path.is_dir() {
            return Err(AppError::InvalidDirectory(path.clone()));
        }

        // 检查是否可读
        if path.read_dir().is_err() {
            return Err(AppError::DirectoryNotReadable(path.clone()));
        }

        Ok(())
    }

    /// 验证输出目录
    fn validate_output_directory(&self, path: &PathBuf) -> Result<()> {
        // 如果目录不存在，尝试创建
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }

        // 检查是否为目录
        if !path.is_dir() {
            return Err(AppError::InvalidDirectory(path.clone()));
        }

        // 检查是否可写
        let test_file = path.join(".write_test");
        if std::fs::write(&test_file, b"test").is_err() {
            return Err(AppError::DirectoryNotWritable(path.clone()));
        }
        let _ = std::fs::remove_file(test_file);

        Ok(())
    }

    /// 验证所有目录
    pub fn validate_directories(&self) -> Result<()> {
        // 验证输入目录
        if let Some(ref input_path) = self.input_path {
            self.validate_input_directory(input_path)?;
        } else {
            return Err(AppError::config_error("未选择输入目录"));
        }

        // 验证输出目录
        if let Some(ref output_path) = self.output_path {
            self.validate_output_directory(output_path)?;
        } else {
            return Err(AppError::config_error("未选择输出目录"));
        }

        Ok(())
    }

    /// 检查是否已选择所有必需的目录
    pub fn is_ready(&self) -> bool {
        self.input_path.is_some() && self.output_path.is_some()
    }

    /// 获取输入目录的显示文本
    pub fn input_directory_display(&self) -> String {
        self.input_path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "未选择".to_string())
    }

    /// 获取输出目录的显示文本
    pub fn output_directory_display(&self) -> String {
        self.output_path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "未选择".to_string())
    }

    /// 检查输入目录中的文件数量
    pub fn count_input_files(&self) -> Result<usize> {
        if let Some(ref input_path) = self.input_path {
            let mut count = 0;
            for entry in std::fs::read_dir(input_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext.eq_ignore_ascii_case("xlsx") {
                            count += 1;
                        }
                    }
                }
            }
            Ok(count)
        } else {
            Ok(0)
        }
    }

    /// 检查输出目录的可用空间（字节）
    pub fn check_output_space(&self) -> Result<u64> {
        if let Some(ref _output_path) = self.output_path {
            // 使用 fs2 crate 获取磁盘空间信息
            // 这里简化实现，返回一个大值
            // 实际应用中应该使用系统 API 获取真实的可用空间
            Ok(u64::MAX)
        } else {
            Ok(0)
        }
    }
}

impl Default for DirectorySelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_directory_selector_creation() {
        let selector = DirectorySelector::new();
        assert!(selector.input_directory().is_none());
        assert!(selector.output_directory().is_none());
        assert!(!selector.is_ready());
    }

    #[test]
    fn test_set_input_directory() {
        let mut selector = DirectorySelector::new();
        let temp_dir = tempdir().unwrap();

        selector
            .set_input_directory(temp_dir.path().to_path_buf())
            .unwrap();

        assert!(selector.input_directory().is_some());
        assert_eq!(
            selector.input_directory().unwrap(),
            &temp_dir.path().to_path_buf()
        );
    }

    #[test]
    fn test_set_output_directory() {
        let mut selector = DirectorySelector::new();
        let temp_dir = tempdir().unwrap();

        selector
            .set_output_directory(temp_dir.path().to_path_buf())
            .unwrap();

        assert!(selector.output_directory().is_some());
    }

    #[test]
    fn test_validate_nonexistent_input() {
        let mut selector = DirectorySelector::new();
        let nonexistent = PathBuf::from("/nonexistent/path");

        let result = selector.set_input_directory(nonexistent);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_ready() {
        let mut selector = DirectorySelector::new();
        let input_dir = tempdir().unwrap();
        let output_dir = tempdir().unwrap();

        assert!(!selector.is_ready());

        selector
            .set_input_directory(input_dir.path().to_path_buf())
            .unwrap();
        assert!(!selector.is_ready());

        selector
            .set_output_directory(output_dir.path().to_path_buf())
            .unwrap();
        assert!(selector.is_ready());
    }

    #[test]
    fn test_clear_directories() {
        let mut selector = DirectorySelector::new();
        let temp_dir = tempdir().unwrap();

        selector
            .set_input_directory(temp_dir.path().to_path_buf())
            .unwrap();
        selector
            .set_output_directory(temp_dir.path().to_path_buf())
            .unwrap();

        assert!(selector.is_ready());

        selector.clear_all();
        assert!(!selector.is_ready());
    }

    #[test]
    fn test_count_input_files() {
        let mut selector = DirectorySelector::new();
        let temp_dir = tempdir().unwrap();

        // 创建测试文件
        std::fs::write(temp_dir.path().join("test1.xlsx"), b"").unwrap();
        std::fs::write(temp_dir.path().join("test2.xlsx"), b"").unwrap();
        std::fs::write(temp_dir.path().join("test.txt"), b"").unwrap();

        selector
            .set_input_directory(temp_dir.path().to_path_buf())
            .unwrap();

        let count = selector.count_input_files().unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_display_text() {
        let selector = DirectorySelector::new();
        assert_eq!(selector.input_directory_display(), "未选择");
        assert_eq!(selector.output_directory_display(), "未选择");
    }
}
