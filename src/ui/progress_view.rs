use crate::models::{ProcessingProgress, ProcessingStats};

/// 进度显示视图
pub struct ProgressView {
    is_visible: bool,
    progress: Option<ProcessingProgress>,
    stats: Option<ProcessingStats>,
    can_cancel: bool,
}

impl ProgressView {
    /// 创建新的进度视图
    pub fn new() -> Self {
        Self {
            is_visible: false,
            progress: None,
            stats: None,
            can_cancel: true,
        }
    }

    /// 显示进度视图
    pub fn show(&mut self) {
        self.is_visible = true;
    }

    /// 隐藏进度视图
    pub fn hide(&mut self) {
        self.is_visible = false;
        self.progress = None;
        self.stats = None;
    }

    /// 检查是否可见
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    /// 更新进度
    pub fn update_progress(&mut self, progress: ProcessingProgress) {
        self.progress = Some(progress);
        self.is_visible = true;
    }

    /// 获取当前进度
    pub fn current_progress(&self) -> Option<&ProcessingProgress> {
        self.progress.as_ref()
    }

    /// 获取进度百分比
    pub fn percentage(&self) -> f32 {
        self.progress
            .as_ref()
            .map(|p| p.percentage)
            .unwrap_or(0.0)
    }

    /// 获取当前文件名
    pub fn current_file(&self) -> Option<&str> {
        self.progress
            .as_ref()
            .map(|p| p.current_file.as_str())
    }

    /// 获取已处理文件数
    pub fn processed_count(&self) -> usize {
        self.progress
            .as_ref()
            .map(|p| p.processed_files)
            .unwrap_or(0)
    }

    /// 获取总文件数
    pub fn total_count(&self) -> usize {
        self.progress
            .as_ref()
            .map(|p| p.total_files)
            .unwrap_or(0)
    }

    /// 显示完成统计
    pub fn show_completion(&mut self, stats: ProcessingStats) {
        self.stats = Some(stats);
        self.progress = None;
    }

    /// 获取完成统计
    pub fn completion_stats(&self) -> Option<&ProcessingStats> {
        self.stats.as_ref()
    }

    /// 检查是否完成
    pub fn is_complete(&self) -> bool {
        self.stats.is_some()
    }

    /// 设置是否可以取消
    pub fn set_can_cancel(&mut self, can_cancel: bool) {
        self.can_cancel = can_cancel;
    }

    /// 检查是否可以取消
    pub fn can_cancel(&self) -> bool {
        self.can_cancel && self.is_visible && !self.is_complete()
    }

    /// 重置进度视图
    pub fn reset(&mut self) {
        self.is_visible = false;
        self.progress = None;
        self.stats = None;
        self.can_cancel = true;
    }

    /// 获取进度文本
    pub fn progress_text(&self) -> String {
        if let Some(progress) = &self.progress {
            format!(
                "已处理: {}/{} 文件 ({:.1}%)",
                progress.processed_files, progress.total_files, progress.percentage
            )
        } else if let Some(stats) = &self.stats {
            format!(
                "完成: 成功 {} | 失败 {} | 总计 {}",
                stats.files_succeeded, stats.files_failed, stats.files_processed
            )
        } else {
            "准备中...".to_string()
        }
    }

    /// 获取状态消息
    pub fn status_message(&self) -> String {
        if let Some(progress) = &self.progress {
            if progress.current_file.is_empty() {
                "正在处理...".to_string()
            } else {
                format!("正在处理: {}", progress.current_file)
            }
        } else if let Some(stats) = &self.stats {
            if stats.files_failed == 0 {
                "✓ 处理完成！所有文件都已成功处理。".to_string()
            } else {
                format!(
                    "⚠ 处理完成，但有 {} 个文件失败。",
                    stats.files_failed
                )
            }
        } else {
            "等待开始...".to_string()
        }
    }

    /// 获取耗时文本
    pub fn duration_text(&self) -> Option<String> {
        self.stats.as_ref().map(|stats| {
            let secs = stats.total_duration.as_secs();
            if secs < 60 {
                format!("耗时: {} 秒", secs)
            } else {
                let mins = secs / 60;
                let secs = secs % 60;
                format!("耗时: {} 分 {} 秒", mins, secs)
            }
        })
    }
}

impl Default for ProgressView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_progress_view_creation() {
        let view = ProgressView::new();
        assert!(!view.is_visible());
        assert!(view.current_progress().is_none());
        assert!(!view.is_complete());
    }

    #[test]
    fn test_show_hide() {
        let mut view = ProgressView::new();
        
        view.show();
        assert!(view.is_visible());
        
        view.hide();
        assert!(!view.is_visible());
    }

    #[test]
    fn test_update_progress() {
        let mut view = ProgressView::new();
        
        let mut progress = ProcessingProgress::new(10);
        progress.update(5, "test.xlsx".to_string());
        
        view.update_progress(progress);
        
        assert!(view.is_visible());
        assert_eq!(view.percentage(), 50.0);
        assert_eq!(view.current_file(), Some("test.xlsx"));
        assert_eq!(view.processed_count(), 5);
        assert_eq!(view.total_count(), 10);
    }

    #[test]
    fn test_show_completion() {
        let mut view = ProgressView::new();
        
        let mut stats = ProcessingStats::new();
        stats.files_processed = 10;
        stats.files_succeeded = 8;
        stats.files_failed = 2;
        stats.total_duration = Duration::from_secs(30);
        
        view.show_completion(stats);
        
        assert!(view.is_complete());
        assert!(view.completion_stats().is_some());
        assert_eq!(view.completion_stats().unwrap().files_succeeded, 8);
    }

    #[test]
    fn test_can_cancel() {
        let mut view = ProgressView::new();
        
        // 默认可以取消
        assert!(view.can_cancel);
        
        // 但不可见时不能取消
        assert!(!view.can_cancel());
        
        // 显示后可以取消
        view.show();
        assert!(view.can_cancel());
        
        // 禁用取消
        view.set_can_cancel(false);
        assert!(!view.can_cancel());
    }

    #[test]
    fn test_progress_text() {
        let mut view = ProgressView::new();
        
        let mut progress = ProcessingProgress::new(10);
        progress.update(5, "test.xlsx".to_string());
        view.update_progress(progress);
        
        let text = view.progress_text();
        assert!(text.contains("5/10"));
        assert!(text.contains("50.0%"));
    }

    #[test]
    fn test_status_message() {
        let mut view = ProgressView::new();
        
        // 初始状态
        assert_eq!(view.status_message(), "等待开始...");
        
        // 处理中
        let mut progress = ProcessingProgress::new(10);
        progress.update(5, "test.xlsx".to_string());
        view.update_progress(progress);
        assert!(view.status_message().contains("test.xlsx"));
        
        // 完成
        let mut stats = ProcessingStats::new();
        stats.files_processed = 10;
        stats.files_succeeded = 10;
        stats.files_failed = 0;
        view.show_completion(stats);
        assert!(view.status_message().contains("完成"));
    }

    #[test]
    fn test_reset() {
        let mut view = ProgressView::new();
        
        let progress = ProcessingProgress::new(10);
        view.update_progress(progress);
        view.set_can_cancel(false);
        
        view.reset();
        
        assert!(!view.is_visible());
        assert!(view.current_progress().is_none());
        assert!(view.can_cancel);
    }
}
