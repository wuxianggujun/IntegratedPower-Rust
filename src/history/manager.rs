use crate::error::{AppError, Result};
use crate::models::ProcessingResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 历史记录条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// 唯一标识符
    pub id: String,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 处理器 ID
    pub processor_id: String,
    /// 处理器名称
    pub processor_name: String,
    /// 输入目录
    pub input_dir: PathBuf,
    /// 输出目录
    pub output_dir: PathBuf,
    /// 处理结果
    pub result: ProcessingResult,
}

impl HistoryEntry {
    /// 创建新的历史记录条目
    pub fn new(
        processor_id: String,
        processor_name: String,
        input_dir: PathBuf,
        output_dir: PathBuf,
        result: ProcessingResult,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            processor_id,
            processor_name,
            input_dir,
            output_dir,
            result,
        }
    }

    /// 检查是否成功
    pub fn is_successful(&self) -> bool {
        self.result.failed == 0
    }

    /// 获取成功率
    pub fn success_rate(&self) -> f32 {
        self.result.success_rate()
    }
}

/// 历史记录管理器
pub struct HistoryManager {
    entries: Vec<HistoryEntry>,
    storage_path: PathBuf,
    max_entries: usize,
}

impl HistoryManager {
    /// 创建新的历史记录管理器
    pub fn new(max_entries: usize) -> Self {
        let storage_path = Self::get_storage_path().unwrap_or_else(|_| PathBuf::from("history.json"));

        let entries = if storage_path.exists() {
            Self::load_from_file(&storage_path).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        Self {
            entries,
            storage_path,
            max_entries,
        }
    }

    /// 加载历史记录管理器（别名方法）
    pub fn load(max_entries: usize) -> Result<Self> {
        Ok(Self::new(max_entries))
    }

    /// 添加历史记录条目
    pub fn add_entry(&mut self, entry: HistoryEntry) -> Result<()> {
        tracing::info!(
            "添加历史记录: {} - {}",
            entry.processor_name,
            entry.timestamp
        );

        self.entries.push(entry);

        // 如果超过最大条目数，删除最旧的条目
        if self.entries.len() > self.max_entries {
            let remove_count = self.entries.len() - self.max_entries;
            self.entries.drain(0..remove_count);
            tracing::debug!("删除 {} 条旧历史记录", remove_count);
        }

        self.save()?;
        Ok(())
    }

    /// 获取所有历史记录条目
    pub fn get_entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    /// 获取最近的 N 条历史记录
    pub fn get_recent_entries(&self, count: usize) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .rev()
            .take(count)
            .collect()
    }

    /// 根据 ID 获取历史记录
    pub fn get_entry_by_id(&self, id: &str) -> Option<&HistoryEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// 根据处理器 ID 获取历史记录
    pub fn get_entries_by_processor(&self, processor_id: &str) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.processor_id == processor_id)
            .collect()
    }

    /// 获取成功的历史记录
    pub fn get_successful_entries(&self) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.is_successful())
            .collect()
    }

    /// 获取失败的历史记录
    pub fn get_failed_entries(&self) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| !e.is_successful())
            .collect()
    }

    /// 清除所有历史记录
    pub fn clear_history(&mut self) -> Result<()> {
        tracing::info!("清除所有历史记录");
        self.entries.clear();
        self.save()?;
        Ok(())
    }

    /// 删除指定的历史记录
    pub fn remove_entry(&mut self, id: &str) -> Result<()> {
        if let Some(pos) = self.entries.iter().position(|e| e.id == id) {
            self.entries.remove(pos);
            self.save()?;
            tracing::info!("删除历史记录: {}", id);
            Ok(())
        } else {
            Err(AppError::history_error(format!("历史记录不存在: {}", id)))
        }
    }

    /// 删除旧的历史记录（保留最近的 N 条）
    pub fn trim_to(&mut self, count: usize) -> Result<()> {
        if self.entries.len() > count {
            let remove_count = self.entries.len() - count;
            self.entries.drain(0..remove_count);
            self.save()?;
            tracing::info!("删除 {} 条旧历史记录", remove_count);
        }
        Ok(())
    }

    /// 获取历史记录数量
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// 获取最大条目数
    pub fn max_entries(&self) -> usize {
        self.max_entries
    }

    /// 设置最大条目数
    pub fn set_max_entries(&mut self, max_entries: usize) -> Result<()> {
        self.max_entries = max_entries;
        self.trim_to(max_entries)?;
        Ok(())
    }

    /// 保存历史记录到文件
    pub fn save(&self) -> Result<()> {
        tracing::debug!("保存历史记录到: {}", self.storage_path.display());

        if let Some(parent) = self.storage_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(&self.entries)
            .map_err(|e| AppError::history_error(format!("序列化历史记录失败: {}", e)))?;

        fs::write(&self.storage_path, json)?;

        Ok(())
    }

    /// 从文件加载历史记录
    fn load_from_file(path: &PathBuf) -> Result<Vec<HistoryEntry>> {
        tracing::debug!("从文件加载历史记录: {}", path.display());

        let content = fs::read_to_string(path)?;
        let entries: Vec<HistoryEntry> = serde_json::from_str(&content)
            .map_err(|e| AppError::history_error(format!("反序列化历史记录失败: {}", e)))?;

        tracing::info!("加载了 {} 条历史记录", entries.len());

        Ok(entries)
    }

    /// 获取存储路径
    fn get_storage_path() -> Result<PathBuf> {
        let config_dir = if cfg!(target_os = "windows") {
            dirs::config_dir()
                .ok_or_else(|| AppError::config_error("无法获取配置目录"))?
                .join("IntegratedPower")
        } else if cfg!(target_os = "macos") {
            dirs::config_dir()
                .ok_or_else(|| AppError::config_error("无法获取配置目录"))?
                .join("IntegratedPower")
        } else {
            dirs::config_dir()
                .ok_or_else(|| AppError::config_error("无法获取配置目录"))?
                .join("IntegratedPower")
        };

        Ok(config_dir.join("history.json"))
    }

    /// 获取存储路径（公开方法）
    pub fn storage_path(&self) -> &PathBuf {
        &self.storage_path
    }

    /// 导出历史记录到文件
    pub fn export_to_file(&self, path: &PathBuf) -> Result<()> {
        tracing::info!("导出历史记录到: {}", path.display());

        let json = serde_json::to_string_pretty(&self.entries)
            .map_err(|e| AppError::history_error(format!("序列化历史记录失败: {}", e)))?;

        fs::write(path, json)?;

        Ok(())
    }

    /// 从文件导入历史记录
    pub fn import_from_file(&mut self, path: &PathBuf) -> Result<()> {
        tracing::info!("从文件导入历史记录: {}", path.display());

        let content = fs::read_to_string(path)?;
        let imported_entries: Vec<HistoryEntry> = serde_json::from_str(&content)
            .map_err(|e| AppError::history_error(format!("反序列化历史记录失败: {}", e)))?;

        // 合并导入的条目
        for entry in imported_entries {
            // 检查是否已存在相同 ID 的条目
            if !self.entries.iter().any(|e| e.id == entry.id) {
                self.entries.push(entry);
            }
        }

        // 按时间戳排序
        self.entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // 限制条目数
        self.trim_to(self.max_entries)?;

        self.save()?;

        tracing::info!("导入完成，当前共 {} 条历史记录", self.entries.len());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ProcessingResult;
    use std::time::Duration;
    use tempfile::tempdir;

    fn create_test_entry() -> HistoryEntry {
        let mut result = ProcessingResult::new(10);
        result.successful = 8;
        result.failed = 2;
        result.duration = Duration::from_secs(30);

        HistoryEntry::new(
            "test_processor".to_string(),
            "Test Processor".to_string(),
            PathBuf::from("/input"),
            PathBuf::from("/output"),
            result,
        )
    }

    #[test]
    fn test_add_entry() {
        let dir = tempdir().unwrap();
        let storage_path = dir.path().join("history.json");

        let mut manager = HistoryManager {
            entries: Vec::new(),
            storage_path,
            max_entries: 100,
        };

        let entry = create_test_entry();
        manager.add_entry(entry).unwrap();

        assert_eq!(manager.entry_count(), 1);
    }

    #[test]
    fn test_max_entries_limit() {
        let dir = tempdir().unwrap();
        let storage_path = dir.path().join("history.json");

        let mut manager = HistoryManager {
            entries: Vec::new(),
            storage_path,
            max_entries: 5,
        };

        // 添加 10 条记录
        for _ in 0..10 {
            let entry = create_test_entry();
            manager.add_entry(entry).unwrap();
        }

        // 应该只保留最新的 5 条
        assert_eq!(manager.entry_count(), 5);
    }

    #[test]
    fn test_get_recent_entries() {
        let dir = tempdir().unwrap();
        let storage_path = dir.path().join("history.json");

        let mut manager = HistoryManager {
            entries: Vec::new(),
            storage_path,
            max_entries: 100,
        };

        for _ in 0..10 {
            let entry = create_test_entry();
            manager.add_entry(entry).unwrap();
        }

        let recent = manager.get_recent_entries(3);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_clear_history() {
        let dir = tempdir().unwrap();
        let storage_path = dir.path().join("history.json");

        let mut manager = HistoryManager {
            entries: Vec::new(),
            storage_path,
            max_entries: 100,
        };

        let entry = create_test_entry();
        manager.add_entry(entry).unwrap();
        assert_eq!(manager.entry_count(), 1);

        manager.clear_history().unwrap();
        assert_eq!(manager.entry_count(), 0);
    }

    #[test]
    fn test_get_successful_entries() {
        let dir = tempdir().unwrap();
        let storage_path = dir.path().join("history.json");

        let mut manager = HistoryManager {
            entries: Vec::new(),
            storage_path,
            max_entries: 100,
        };

        // 添加成功的条目
        let mut success_result = ProcessingResult::new(10);
        success_result.successful = 10;
        success_result.failed = 0;

        let success_entry = HistoryEntry::new(
            "test".to_string(),
            "Test".to_string(),
            PathBuf::from("/input"),
            PathBuf::from("/output"),
            success_result,
        );

        manager.add_entry(success_entry).unwrap();

        // 添加失败的条目
        let entry = create_test_entry();
        manager.add_entry(entry).unwrap();

        let successful = manager.get_successful_entries();
        assert_eq!(successful.len(), 1);

        let failed = manager.get_failed_entries();
        assert_eq!(failed.len(), 1);
    }
}
