use crate::error::Result;
use async_trait::async_trait;
use polars::prelude::*;
use std::path::Path;

/// 数据处理器 trait
#[async_trait]
pub trait DataProcessor: Send + Sync {
    /// 获取处理器 ID
    fn id(&self) -> &str;

    /// 获取处理器名称
    fn name(&self) -> &str;

    /// 获取处理器描述
    fn description(&self) -> &str;

    /// 处理 DataFrame
    async fn process(&self, df: DataFrame) -> Result<DataFrame>;

    /// 验证输入文件
    fn validate_input(&self, _file: &Path) -> Result<()> {
        // 默认实现：不进行额外验证
        Ok(())
    }

    /// 获取处理器图标（可选）
    fn icon(&self) -> Option<&str> {
        None
    }

    /// 获取处理器版本
    fn version(&self) -> &str {
        "1.0.0"
    }

    /// 检查处理器是否可用
    fn is_available(&self) -> bool {
        true
    }
}

/// 处理器信息
#[derive(Debug, Clone)]
pub struct ProcessorInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub version: String,
    pub available: bool,
}

impl ProcessorInfo {
    /// 从处理器创建信息
    pub fn from_processor(processor: &dyn DataProcessor) -> Self {
        Self {
            id: processor.id().to_string(),
            name: processor.name().to_string(),
            description: processor.description().to_string(),
            icon: processor.icon().map(|s| s.to_string()),
            version: processor.version().to_string(),
            available: processor.is_available(),
        }
    }
}
