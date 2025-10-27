use crate::processor::trait_def::ProcessorInfo;

/// 处理器信息（用于 UI 显示）
#[derive(Debug, Clone)]
pub struct ProcessorEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub version: String,
}

/// 处理器管理器
pub struct ProcessorManager {
    processors: Vec<ProcessorEntry>,
}

impl ProcessorManager {
    /// 创建新的处理器管理器
    pub fn new() -> Self {
        let mut manager = Self {
            processors: Vec::new(),
        };
        
        // 注册示例处理器
        manager.register_example_processors();
        
        manager
    }

    /// 注册示例处理器
    fn register_example_processors(&mut self) {
        // 示例处理器 1
        self.processors.push(ProcessorEntry {
            id: "data_cleaning".to_string(),
            name: "数据清洗".to_string(),
            description: "清洗和过滤数据，移除空值和重复项".to_string(),
            icon: Some("🧹".to_string()),
            version: "1.0.0".to_string(),
        });

        // 示例处理器 2
        self.processors.push(ProcessorEntry {
            id: "data_statistics".to_string(),
            name: "数据统计".to_string(),
            description: "计算数据的统计信息和汇总".to_string(),
            icon: Some("📊".to_string()),
            version: "1.0.0".to_string(),
        });
    }

    /// 获取处理器信息
    pub fn get_processor(&self, id: &str) -> Option<&ProcessorEntry> {
        self.processors.iter().find(|p| p.id == id)
    }

    /// 列出所有处理器
    pub fn list_processors(&self) -> Vec<ProcessorInfo> {
        self.processors
            .iter()
            .map(|p| ProcessorInfo {
                id: p.id.clone(),
                name: p.name.clone(),
                description: p.description.clone(),
                icon: p.icon.clone(),
                version: p.version.clone(),
                available: true,
            })
            .collect()
    }

    /// 检查处理器是否存在
    pub fn has_processor(&self, id: &str) -> bool {
        self.processors.iter().any(|p| p.id == id)
    }

    /// 获取处理器数量
    pub fn processor_count(&self) -> usize {
        self.processors.len()
    }

}

impl Default for ProcessorManager {
    fn default() -> Self {
        Self::new()
    }
}
