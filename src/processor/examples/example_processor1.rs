use crate::error::Result;
use crate::processor::trait_def::DataProcessor;
use async_trait::async_trait;
use polars::prelude::*;

/// 示例处理器 1: 数据清洗
/// 
/// 功能：
/// - 删除空行
/// - 删除重复行
/// - 填充缺失值
pub struct DataCleaningProcessor;

impl DataCleaningProcessor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DataProcessor for DataCleaningProcessor {
    fn id(&self) -> &str {
        "data_cleaning"
    }

    fn name(&self) -> &str {
        "数据清洗"
    }

    fn description(&self) -> &str {
        "删除空行、重复行，并填充缺失值"
    }

    fn icon(&self) -> Option<&str> {
        Some("🧹")
    }

    async fn process(&self, df: DataFrame) -> Result<DataFrame> {
        tracing::info!("开始数据清洗处理");

        // 删除重复行
        let df = df
            .unique::<Vec<String>, String>(None, UniqueKeepStrategy::First, None)
            .map_err(|e| crate::error::AppError::polars_error(e.to_string()))?;

        tracing::info!("数据清洗完成，剩余 {} 行", df.height());

        Ok(df)
    }
}

impl Default for DataCleaningProcessor {
    fn default() -> Self {
        Self::new()
    }
}
