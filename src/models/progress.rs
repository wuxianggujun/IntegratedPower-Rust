use serde::{Deserialize, Serialize};

/// 处理进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingProgress {
    /// 总文件数
    pub total_files: usize,
    /// 已处理文件数
    pub processed_files: usize,
    /// 当前正在处理的文件名
    pub current_file: String,
    /// 进度百分比 (0.0 - 100.0)
    pub percentage: f32,
}

impl ProcessingProgress {
    /// 创建新的进度信息
    pub fn new(total_files: usize) -> Self {
        Self {
            total_files,
            processed_files: 0,
            current_file: String::new(),
            percentage: 0.0,
        }
    }

    /// 更新进度
    pub fn update(&mut self, processed_files: usize, current_file: String) {
        self.processed_files = processed_files;
        self.current_file = current_file;
        self.percentage = if self.total_files > 0 {
            (self.processed_files as f32 / self.total_files as f32) * 100.0
        } else {
            0.0
        };
    }

    /// 检查是否完成
    pub fn is_complete(&self) -> bool {
        self.processed_files >= self.total_files
    }
}

impl Default for ProcessingProgress {
    fn default() -> Self {
        Self {
            total_files: 0,
            processed_files: 0,
            current_file: String::new(),
            percentage: 0.0,
        }
    }
}
