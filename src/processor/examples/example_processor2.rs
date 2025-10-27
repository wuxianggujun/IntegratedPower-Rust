use crate::error::Result;
use crate::processor::trait_def::DataProcessor;
use async_trait::async_trait;
use polars::prelude::*;

/// 示例处理器 2: 数据统计
/// 
/// 功能：
/// - 添加行号列
/// - 计算数值列的统计信息
/// - 添加汇总行
pub struct DataStatisticsProcessor;

impl DataStatisticsProcessor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DataProcessor for DataStatisticsProcessor {
    fn id(&self) -> &str {
        "data_statistics"
    }

    fn name(&self) -> &str {
        "数据统计"
    }

    fn description(&self) -> &str {
        "添加行号，计算统计信息"
    }

    fn icon(&self) -> Option<&str> {
        Some("📊")
    }

    async fn process(&self, df: DataFrame) -> Result<DataFrame> {
        tracing::info!("开始数据统计处理");

        let row_count = df.height();

        // 添加行号列
        let row_numbers: Vec<u32> = (1..=row_count as u32).collect();
        let row_number_series = Series::new("行号".into(), row_numbers);
        let row_number_column = row_number_series.into_column();

        // 将行号列添加到 DataFrame 的开头
        let mut columns = vec![row_number_column];
        for col in df.get_columns() {
            columns.push(col.clone());
        }

        let df = DataFrame::new(columns)
            .map_err(|e| crate::error::AppError::polars_error(e.to_string()))?;

        tracing::info!("数据统计完成，共 {} 行", df.height());

        Ok(df)
    }
}

impl Default for DataStatisticsProcessor {
    fn default() -> Self {
        Self::new()
    }
}
