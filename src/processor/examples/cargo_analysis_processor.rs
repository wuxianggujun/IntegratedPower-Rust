// Cargo Analysis Processor - 货物分析表处理器
use crate::engine::{ExcelExtractor, FileTypeProfile, RowTypeIdentifier};
use crate::error::Result;
use crate::models::RowType;
use crate::processor::DataProcessor;
use async_trait::async_trait;
use polars::prelude::*;
use std::path::Path;

/// 货物分析表处理器
///
/// 该处理器使用行类型识别系统来处理货物分析表Excel文件。
/// 它能够：
/// - 自动识别项目编号行、柜号行、表头行、数据行和合计行
/// - 根据识别结果分组处理不同类型的行
/// - 提取和转换数据到标准格式
pub struct CargoAnalysisProcessor;

impl CargoAnalysisProcessor {
    pub fn new() -> Self {
        Self
    }

    /// 处理Excel文件并识别行类型
    async fn process_excel_file(&self, file_path: &Path) -> Result<DataFrame> {
        tracing::info!("Processing cargo analysis file: {}", file_path.display());

        // 1. 使用ExcelExtractor读取工作表数据
        let worksheet_data = ExcelExtractor::read_worksheet(file_path, 0)
            .map_err(|e| crate::error::AppError::processing_error(e.to_string()))?;

        tracing::info!(
            "Loaded worksheet '{}' with {} rows",
            worksheet_data.name,
            worksheet_data.row_count()
        );

        // 2. 创建行类型识别器
        let profile = FileTypeProfile::cargo_analysis();
        let identifier = RowTypeIdentifier::new(profile);

        // 3. 识别所有行的类型
        let results = identifier.identify_all_rows(&worksheet_data);

        // 4. 获取统计信息
        let stats = identifier.get_statistics(&results);
        tracing::info!(
            "Row identification complete: {} rows, {:.1}% success rate",
            stats.total_rows,
            stats.success_rate * 100.0
        );
        tracing::debug!("Row type distribution: {:?}", stats.row_type_counts);

        // 5. 根据识别结果分组处理
        let mut project_numbers = Vec::new();
        let mut cabinet_numbers = Vec::new();
        let mut data_rows = Vec::new();

        for result in &results {
            let row = worksheet_data.get_row(result.row_index).unwrap();

            match result.row_type {
                RowType::ProjectNumber => {
                    // 提取项目编号
                    if let Some(cell) = row.first_non_empty_cell() {
                        project_numbers.push(cell.content.clone());
                        tracing::debug!("Found project number: {}", cell.content);
                    }
                }
                RowType::CabinetNumber => {
                    // 提取柜号信息
                    if let Some(cell) = row.get_cell(1) {
                        cabinet_numbers.push(cell.content.clone());
                        tracing::debug!("Found cabinet number: {}", cell.content);
                    }
                }
                RowType::Data => {
                    // 收集数据行
                    let row_data: Vec<String> =
                        row.cells.iter().map(|c| c.content.clone()).collect();
                    data_rows.push(row_data);
                }
                RowType::Header => {
                    tracing::debug!("Found header row at index {}", result.row_index);
                }
                RowType::SubTotal | RowType::UnitTotal | RowType::GrandTotal => {
                    tracing::debug!(
                        "Found {:?} row at index {}",
                        result.row_type,
                        result.row_index
                    );
                }
                RowType::Unknown => {
                    tracing::warn!("Unknown row type at index {}", result.row_index);
                }
                _ => {}
            }
        }

        tracing::info!(
            "Extracted: {} project numbers, {} cabinet numbers, {} data rows",
            project_numbers.len(),
            cabinet_numbers.len(),
            data_rows.len()
        );

        // 6. 转换为DataFrame
        // 这里创建一个简单的汇总DataFrame
        let df = self.create_summary_dataframe(project_numbers, cabinet_numbers, data_rows)?;

        Ok(df)
    }

    /// 创建汇总DataFrame
    fn create_summary_dataframe(
        &self,
        project_numbers: Vec<String>,
        cabinet_numbers: Vec<String>,
        data_rows: Vec<Vec<String>>,
    ) -> Result<DataFrame> {
        // 创建一个简单的汇总表
        let mut columns = Vec::new();

        // 项目编号列
        let project_series = Series::new(
            "项目编号".into(),
            vec![project_numbers.join(", ")],
        );
        columns.push(project_series.into_column());

        // 柜号数量列
        let cabinet_count_series = Series::new(
            "柜号数量".into(),
            vec![cabinet_numbers.len() as i32],
        );
        columns.push(cabinet_count_series.into_column());

        // 数据行数量列
        let data_row_count_series = Series::new(
            "数据行数量".into(),
            vec![data_rows.len() as i32],
        );
        columns.push(data_row_count_series.into_column());

        DataFrame::new(columns)
            .map_err(|e| crate::error::AppError::polars_error(e.to_string()))
    }
}

impl Default for CargoAnalysisProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DataProcessor for CargoAnalysisProcessor {
    fn id(&self) -> &str {
        "cargo_analysis"
    }

    fn name(&self) -> &str {
        "货物分析表处理器"
    }

    fn description(&self) -> &str {
        "自动识别和处理货物分析表，提取项目编号、柜号、数据行等信息"
    }

    async fn process(&self, _df: DataFrame) -> Result<DataFrame> {
        // 注意：这个方法接收DataFrame，但我们需要原始Excel文件
        // 在实际使用中，应该通过validate_input或其他方式传递文件路径
        // 这里返回一个占位DataFrame
        tracing::warn!("process() called with DataFrame, but cargo analysis needs Excel file path");

        let columns = vec![
            Series::new("提示".into(), vec!["请使用文件路径处理模式"]).into_column(),
        ];

        DataFrame::new(columns)
            .map_err(|e| crate::error::AppError::polars_error(e.to_string()))
    }

    fn validate_input(&self, file: &Path) -> Result<()> {
        // 验证文件是否为Excel文件
        if let Some(ext) = file.extension() {
            if ext.eq_ignore_ascii_case("xlsx") || ext.eq_ignore_ascii_case("xls") {
                return Ok(());
            }
        }

        Err(crate::error::AppError::processing_error(
            "文件必须是Excel格式 (.xlsx 或 .xls)",
        ))
    }

    fn icon(&self) -> Option<&str> {
        Some("📦")
    }

    fn version(&self) -> &str {
        "1.0.0"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_info() {
        let processor = CargoAnalysisProcessor::new();
        assert_eq!(processor.id(), "cargo_analysis");
        assert_eq!(processor.name(), "货物分析表处理器");
        assert_eq!(processor.icon(), Some("📦"));
    }

    #[test]
    fn test_validate_input() {
        let processor = CargoAnalysisProcessor::new();

        // 有效的Excel文件
        let valid_path = Path::new("test.xlsx");
        assert!(processor.validate_input(valid_path).is_ok());

        // 无效的文件类型
        let invalid_path = Path::new("test.txt");
        assert!(processor.validate_input(invalid_path).is_err());
    }

    #[test]
    fn test_create_summary_dataframe() {
        let processor = CargoAnalysisProcessor::new();

        let project_numbers = vec!["G00E-500009085-00011".to_string()];
        let cabinet_numbers = vec!["1-1".to_string(), "1-2".to_string()];
        let data_rows = vec![
            vec!["1".to_string(), "Item1".to_string()],
            vec!["2".to_string(), "Item2".to_string()],
        ];

        let df = processor
            .create_summary_dataframe(project_numbers, cabinet_numbers, data_rows)
            .unwrap();

        assert_eq!(df.height(), 1);
        assert_eq!(df.width(), 3);
    }
}
