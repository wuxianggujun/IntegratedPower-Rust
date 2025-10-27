use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// 处理结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    /// 总文件数
    pub total_files: usize,
    /// 成功处理的文件数
    pub successful: usize,
    /// 失败的文件数
    pub failed: usize,
    /// 错误列表
    pub errors: Vec<ProcessingError>,
    /// 处理耗时
    #[serde(with = "duration_serde")]
    pub duration: Duration,
}

impl ProcessingResult {
    /// 创建新的处理结果
    pub fn new(total_files: usize) -> Self {
        Self {
            total_files,
            successful: 0,
            failed: 0,
            errors: Vec::new(),
            duration: Duration::default(),
        }
    }

    /// 添加成功记录
    pub fn add_success(&mut self) {
        self.successful += 1;
    }

    /// 添加失败记录
    pub fn add_failure(&mut self, error: ProcessingError) {
        self.failed += 1;
        self.errors.push(error);
    }

    /// 设置处理耗时
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    /// 检查是否有错误
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// 获取成功率
    pub fn success_rate(&self) -> f32 {
        if self.total_files > 0 {
            (self.successful as f32 / self.total_files as f32) * 100.0
        } else {
            0.0
        }
    }
}

/// 处理错误信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingError {
    /// 出错的文件路径
    pub file: PathBuf,
    /// 错误消息
    pub error_message: String,
}

impl ProcessingError {
    /// 创建新的处理错误
    pub fn new(file: PathBuf, error_message: String) -> Self {
        Self {
            file,
            error_message,
        }
    }
}

/// 处理统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStats {
    /// 已处理文件数
    pub files_processed: usize,
    /// 成功的文件数
    pub files_succeeded: usize,
    /// 失败的文件数
    pub files_failed: usize,
    /// 总耗时
    #[serde(with = "duration_serde")]
    pub total_duration: Duration,
}

impl ProcessingStats {
    /// 从处理结果创建统计信息
    pub fn from_result(result: &ProcessingResult) -> Self {
        Self {
            files_processed: result.total_files,
            files_succeeded: result.successful,
            files_failed: result.failed,
            total_duration: result.duration,
        }
    }

    /// 创建新的统计信息
    pub fn new() -> Self {
        Self {
            files_processed: 0,
            files_succeeded: 0,
            files_failed: 0,
            total_duration: Duration::default(),
        }
    }
}

impl Default for ProcessingStats {
    fn default() -> Self {
        Self::new()
    }
}

// Duration 序列化辅助模块
mod duration_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}
