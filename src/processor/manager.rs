use crate::error::{AppError, Result};
use crate::processor::trait_def::{DataProcessor, ProcessorInfo};
use std::collections::HashMap;
use std::sync::Arc;

/// 处理器管理器
pub struct ProcessorManager {
    processors: HashMap<String, Arc<dyn DataProcessor>>,
}

impl ProcessorManager {
    /// 创建新的处理器管理器
    pub fn new() -> Self {
        Self {
            processors: HashMap::new(),
        }
    }

    /// 注册处理器
    pub fn register_processor(&mut self, processor: Arc<dyn DataProcessor>) {
        let id = processor.id().to_string();
        tracing::info!("注册处理器: {} ({})", processor.name(), id);
        self.processors.insert(id, processor);
    }

    /// 获取处理器
    pub fn get_processor(&self, id: &str) -> Option<Arc<dyn DataProcessor>> {
        self.processors.get(id).cloned()
    }

    /// 列出所有处理器
    pub fn list_processors(&self) -> Vec<ProcessorInfo> {
        self.processors
            .values()
            .map(|p| ProcessorInfo::from_processor(p.as_ref()))
            .collect()
    }

    /// 列出可用的处理器
    pub fn list_available_processors(&self) -> Vec<ProcessorInfo> {
        self.processors
            .values()
            .filter(|p| p.is_available())
            .map(|p| ProcessorInfo::from_processor(p.as_ref()))
            .collect()
    }

    /// 检查处理器是否存在
    pub fn has_processor(&self, id: &str) -> bool {
        self.processors.contains_key(id)
    }

    /// 获取处理器数量
    pub fn processor_count(&self) -> usize {
        self.processors.len()
    }

    /// 移除处理器
    pub fn unregister_processor(&mut self, id: &str) -> Result<()> {
        if self.processors.remove(id).is_some() {
            tracing::info!("移除处理器: {}", id);
            Ok(())
        } else {
            Err(AppError::ProcessorNotFound(id.to_string()))
        }
    }

    /// 清空所有处理器
    pub fn clear(&mut self) {
        tracing::info!("清空所有处理器");
        self.processors.clear();
    }
}

impl Default for ProcessorManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::trait_def::DataProcessor;
    use async_trait::async_trait;
    use polars::prelude::*;

    struct TestProcessor {
        id: String,
        name: String,
    }

    #[async_trait]
    impl DataProcessor for TestProcessor {
        fn id(&self) -> &str {
            &self.id
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            "Test processor"
        }

        async fn process(&self, df: DataFrame) -> Result<DataFrame> {
            Ok(df)
        }
    }

    #[test]
    fn test_register_processor() {
        let mut manager = ProcessorManager::new();
        let processor = Arc::new(TestProcessor {
            id: "test1".to_string(),
            name: "Test 1".to_string(),
        });

        manager.register_processor(processor);
        assert_eq!(manager.processor_count(), 1);
        assert!(manager.has_processor("test1"));
    }

    #[test]
    fn test_get_processor() {
        let mut manager = ProcessorManager::new();
        let processor = Arc::new(TestProcessor {
            id: "test1".to_string(),
            name: "Test 1".to_string(),
        });

        manager.register_processor(processor);

        let retrieved = manager.get_processor("test1");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id(), "test1");
    }

    #[test]
    fn test_list_processors() {
        let mut manager = ProcessorManager::new();

        manager.register_processor(Arc::new(TestProcessor {
            id: "test1".to_string(),
            name: "Test 1".to_string(),
        }));

        manager.register_processor(Arc::new(TestProcessor {
            id: "test2".to_string(),
            name: "Test 2".to_string(),
        }));

        let list = manager.list_processors();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_unregister_processor() {
        let mut manager = ProcessorManager::new();
        let processor = Arc::new(TestProcessor {
            id: "test1".to_string(),
            name: "Test 1".to_string(),
        });

        manager.register_processor(processor);
        assert_eq!(manager.processor_count(), 1);

        manager.unregister_processor("test1").unwrap();
        assert_eq!(manager.processor_count(), 0);
    }
}
