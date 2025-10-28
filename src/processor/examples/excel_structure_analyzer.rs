use crate::error::Result;
use crate::processor::trait_def::DataProcessor;
use async_trait::async_trait;
use polars::prelude::*;
use std::path::Path;
use calamine::{open_workbook_auto, Reader};

/// Excel结构分析器
///
/// 功能：
/// - 分析 Excel 文件中单个 Sheet 的表格结构
/// - 直接输出完整的表格内容到日志
pub struct ExcelStructureAnalyzer;

impl ExcelStructureAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// 分析 Excel 文件结构并输出到日志
    ///
    /// selected_sheet: 指定要分析的 Sheet 名称；
    /// - Some(name): 仅分析指定的 Sheet
    /// - None: 若文件存在多个 Sheet，则默认分析第一个
    pub fn analyze_excel_structure(&self, file_path: &Path, selected_sheet: Option<&str>) -> Result<()> {
        crate::log_info!("开始分析 Excel 文件: {}", file_path.display());

        // 打开 Excel 文件
        let mut workbook = open_workbook_auto(file_path)
            .map_err(|e| crate::error::AppError::calamine_error(format!("无法打开 Excel 文件: {}", e)))?;

        // 获取所有 sheet 名称
        let sheet_names = workbook.sheet_names().to_vec();
        if sheet_names.is_empty() {
            crate::log_info!("Excel 文件中没有 Sheet");
            return Ok(());
        }

        // 选择要分析的 sheet
        let sheet_to_analyze = selected_sheet
            .filter(|s| sheet_names.iter().any(|n| n == s))
            .map(|s| s.to_string())
            .unwrap_or_else(|| sheet_names[0].clone());

        crate::log_info!("将分析 Sheet: {}", sheet_to_analyze);
        self.analyze_sheet(&mut workbook, &sheet_to_analyze)?;

        crate::log_info!("Excel 文件结构分析完成");
        Ok(())
    }

    /// 分析单个 Sheet 的结构
    fn analyze_sheet(&self, workbook: &mut calamine::Sheets<std::io::BufReader<std::fs::File>>, sheet_name: &str) -> Result<()> {
        crate::log_info!("=== 分析 Sheet: {} ===", sheet_name);

        // 获取 sheet 范围
        let range = workbook
            .worksheet_range(sheet_name)
            .map_err(|e| crate::error::AppError::calamine_error(format!("无法读取 Sheet '{}': {}", sheet_name, e)))?;

        let height = range.height();
        let width = range.width();
        crate::log_info!("Sheet 尺寸: {} 行 × {} 列", height, width);

        // 直接输出完整表格内容
        self.dump_full_sheet(&range)?;

        crate::log_info!("=== Sheet {} 分析完成 ===", sheet_name);
        Ok(())
    }

    /// 输出完整表格内容（逐行逐列）
    fn dump_full_sheet(&self, range: &calamine::Range<calamine::Data>) -> Result<()> {
        let height = range.height();
        let width = range.width();
        if height == 0 || width == 0 {
            crate::log_info!("工作表为空");
            return Ok(());
        }

        crate::log_info!("表格完整内容 ({} 行 × {} 列):", height, width);

        for r in 0..height {
            let mut row_vals: Vec<String> = Vec::with_capacity(width);
            for c in 0..width {
                let val = range
                    .get_value((r as u32, c as u32))
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| String::new());
                row_vals.push(val);
            }
            crate::log_info!("第{}行: {:?}", r + 1, row_vals);
        }

        Ok(())
    }
}

#[async_trait]
impl DataProcessor for ExcelStructureAnalyzer {
    fn id(&self) -> &str {
        "excel_structure_analyzer"
    }

    fn name(&self) -> &str {
        "Excel结构分析器"
    }

    fn description(&self) -> &str {
        "分析 Excel 文件的单个 Sheet 结构"
    }

    fn icon(&self) -> Option<&str> {
        Some("🔍")
    }

    async fn process(&self, _df: DataFrame) -> Result<DataFrame> {
        // 这个处理器主要处理文件而不是 DataFrame
        // 实际处理逻辑在 analyze_excel_structure 方法中
        crate::log_info!("Excel 结构分析器已启动，请使用文件选择功能选择 Excel 文件");
        Ok(_df)
    }

    /// 验证输入文件
    fn validate_input(&self, file: &Path) -> Result<()> {
        if !file.exists() {
            return Err(crate::error::AppError::processing_error("文件不存在".to_string()));
        }

        if let Some(extension) = file.extension() {
            if extension != "xlsx" && extension != "xls" {
                return Err(crate::error::AppError::processing_error("不是有效的 Excel 文件".to_string()));
            }
        } else {
            return Err(crate::error::AppError::processing_error("文件没有扩展名".to_string()));
        }

        Ok(())
    }
}

impl Default for ExcelStructureAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
