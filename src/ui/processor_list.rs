use crate::processor::ProcessorInfo;

/// 处理器卡片
#[derive(Debug, Clone)]
pub struct ProcessorCard {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub version: String,
    pub available: bool,
    pub selected: bool,
}

impl ProcessorCard {
    /// 从处理器信息创建卡片
    pub fn from_processor_info(info: ProcessorInfo) -> Self {
        Self {
            id: info.id,
            name: info.name,
            description: info.description,
            icon: info.icon.unwrap_or_else(|| "📊".to_string()),
            version: info.version,
            available: info.available,
            selected: false,
        }
    }
}

/// 处理器列表视图
pub struct ProcessorListView {
    processors: Vec<ProcessorCard>,
    search_query: String,
    selected_processor_id: Option<String>,
}

impl ProcessorListView {
    /// 创建新的处理器列表视图
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
            search_query: String::new(),
            selected_processor_id: None,
        }
    }

    /// 设置处理器列表
    pub fn set_processors(&mut self, processor_infos: Vec<ProcessorInfo>) {
        self.processors = processor_infos
            .into_iter()
            .map(ProcessorCard::from_processor_info)
            .collect();

        // 更新选中状态
        self.update_selection();
    }

    /// 获取所有处理器
    pub fn processors(&self) -> &[ProcessorCard] {
        &self.processors
    }

    /// 获取过滤后的处理器列表
    pub fn filtered_processors(&self) -> Vec<&ProcessorCard> {
        if self.search_query.is_empty() {
            self.processors.iter().collect()
        } else {
            let query = self.search_query.to_lowercase();
            self.processors
                .iter()
                .filter(|p| {
                    p.name.to_lowercase().contains(&query)
                        || p.description.to_lowercase().contains(&query)
                        || p.id.to_lowercase().contains(&query)
                })
                .collect()
        }
    }

    /// 获取可用的处理器列表
    pub fn available_processors(&self) -> Vec<&ProcessorCard> {
        self.processors.iter().filter(|p| p.available).collect()
    }

    /// 设置搜索查询
    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    /// 获取搜索查询
    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    /// 清除搜索
    pub fn clear_search(&mut self) {
        self.search_query.clear();
    }

    /// 选择处理器
    pub fn select_processor(&mut self, processor_id: &str) {
        self.selected_processor_id = Some(processor_id.to_string());
        self.update_selection();
    }

    /// 取消选择
    pub fn deselect(&mut self) {
        self.selected_processor_id = None;
        self.update_selection();
    }

    /// 获取选中的处理器 ID
    pub fn selected_processor_id(&self) -> Option<&str> {
        self.selected_processor_id.as_deref()
    }

    /// 获取选中的处理器
    pub fn selected_processor(&self) -> Option<&ProcessorCard> {
        self.selected_processor_id.as_ref().and_then(|id| {
            self.processors.iter().find(|p| &p.id == id)
        })
    }

    /// 检查是否有选中的处理器
    pub fn has_selection(&self) -> bool {
        self.selected_processor_id.is_some()
    }

    /// 更新选中状态
    fn update_selection(&mut self) {
        for processor in &mut self.processors {
            processor.selected = self
                .selected_processor_id
                .as_ref()
                .map(|id| &processor.id == id)
                .unwrap_or(false);
        }
    }

    /// 获取处理器数量
    pub fn processor_count(&self) -> usize {
        self.processors.len()
    }

    /// 获取可用处理器数量
    pub fn available_count(&self) -> usize {
        self.processors.iter().filter(|p| p.available).count()
    }

    /// 按名称排序
    pub fn sort_by_name(&mut self) {
        self.processors.sort_by(|a, b| a.name.cmp(&b.name));
    }

    /// 按 ID 排序
    pub fn sort_by_id(&mut self) {
        self.processors.sort_by(|a, b| a.id.cmp(&b.id));
    }

    /// 刷新处理器列表
    pub fn refresh(&mut self, processor_infos: Vec<ProcessorInfo>) {
        let selected_id = self.selected_processor_id.clone();
        self.set_processors(processor_infos);
        if let Some(id) = selected_id {
            self.select_processor(&id);
        }
    }
}

impl Default for ProcessorListView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_processor_info(id: &str, name: &str) -> ProcessorInfo {
        ProcessorInfo {
            id: id.to_string(),
            name: name.to_string(),
            description: format!("{} description", name),
            icon: Some("📊".to_string()),
            version: "1.0.0".to_string(),
            available: true,
        }
    }

    #[test]
    fn test_processor_list_view_creation() {
        let view = ProcessorListView::new();
        assert_eq!(view.processor_count(), 0);
        assert!(!view.has_selection());
    }

    #[test]
    fn test_set_processors() {
        let mut view = ProcessorListView::new();
        let processors = vec![
            create_test_processor_info("proc1", "Processor 1"),
            create_test_processor_info("proc2", "Processor 2"),
        ];

        view.set_processors(processors);
        assert_eq!(view.processor_count(), 2);
    }

    #[test]
    fn test_select_processor() {
        let mut view = ProcessorListView::new();
        let processors = vec![
            create_test_processor_info("proc1", "Processor 1"),
            create_test_processor_info("proc2", "Processor 2"),
        ];

        view.set_processors(processors);
        view.select_processor("proc1");

        assert!(view.has_selection());
        assert_eq!(view.selected_processor_id(), Some("proc1"));

        let selected = view.selected_processor().unwrap();
        assert_eq!(selected.id, "proc1");
        assert!(selected.selected);
    }

    #[test]
    fn test_search_filter() {
        let mut view = ProcessorListView::new();
        let processors = vec![
            create_test_processor_info("data_cleaning", "Data Cleaning"),
            create_test_processor_info("data_stats", "Data Statistics"),
            create_test_processor_info("export", "Export Data"),
        ];

        view.set_processors(processors);

        // 搜索 "data"
        view.set_search_query("data".to_string());
        let filtered = view.filtered_processors();
        assert_eq!(filtered.len(), 3);

        // 搜索 "cleaning"
        view.set_search_query("cleaning".to_string());
        let filtered = view.filtered_processors();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "data_cleaning");

        // 清除搜索
        view.clear_search();
        let filtered = view.filtered_processors();
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_deselect() {
        let mut view = ProcessorListView::new();
        let processors = vec![create_test_processor_info("proc1", "Processor 1")];

        view.set_processors(processors);
        view.select_processor("proc1");
        assert!(view.has_selection());

        view.deselect();
        assert!(!view.has_selection());
        assert_eq!(view.selected_processor_id(), None);
    }

    #[test]
    fn test_sort_by_name() {
        let mut view = ProcessorListView::new();
        let processors = vec![
            create_test_processor_info("proc3", "Zebra"),
            create_test_processor_info("proc1", "Alpha"),
            create_test_processor_info("proc2", "Beta"),
        ];

        view.set_processors(processors);
        view.sort_by_name();

        let names: Vec<&str> = view.processors().iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, vec!["Alpha", "Beta", "Zebra"]);
    }

    #[test]
    fn test_available_processors() {
        let mut view = ProcessorListView::new();
        let mut processors = vec![
            create_test_processor_info("proc1", "Processor 1"),
            create_test_processor_info("proc2", "Processor 2"),
        ];

        processors[1].available = false;

        view.set_processors(processors);

        let available = view.available_processors();
        assert_eq!(available.len(), 1);
        assert_eq!(available[0].id, "proc1");
    }
}
