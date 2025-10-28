// Excel Data Extraction Layer
use crate::engine::{IdentificationError, IdentificationResult};
use crate::models::{CellData, CellStyle, MergeInfo, RgbColor, RowData, WorksheetData};
use std::path::Path;

/// Excel数据提取器
pub struct ExcelExtractor;

impl ExcelExtractor {
    /// 从Excel文件读取工作表数据
    pub fn read_worksheet(
        path: &Path,
        sheet_index: usize,
    ) -> IdentificationResult<WorksheetData> {
        tracing::debug!("Reading Excel worksheet from: {}", path.display());

        let book = umya_spreadsheet::reader::xlsx::read(path).map_err(|e| {
            let msg = format!("无法打开Excel文件: {}", e);
            tracing::error!("{}", msg);
            IdentificationError::file_read_error(msg)
        })?;

        let sheets = book.get_sheet_collection();
        if sheet_index >= sheets.len() {
            let msg = format!(
                "工作表索引 {} 超出范围，文件只有 {} 个工作表",
                sheet_index,
                sheets.len()
            );
            tracing::error!("{}", msg);
            return Err(IdentificationError::worksheet_not_found(msg));
        }

        let worksheet = &sheets[sheet_index];
        let sheet_name = worksheet.get_name().to_string();

        tracing::debug!("Reading worksheet: {}", sheet_name);

        let max_row = worksheet.get_highest_row();
        let max_col = worksheet.get_highest_column();

        tracing::debug!(
            "Worksheet dimensions: {} rows x {} columns",
            max_row,
            max_col
        );

        if max_row == 0 || max_col == 0 {
            tracing::warn!("Worksheet is empty");
            return Ok(WorksheetData {
                name: sheet_name,
                rows: Vec::new(),
            });
        }

        let mut rows = Vec::new();
        for row_idx in 1..=max_row {
            let row_data = Self::extract_row(worksheet, row_idx, max_col);
            rows.push(row_data);
        }

        tracing::info!(
            "Successfully extracted {} rows from worksheet '{}'",
            rows.len(),
            sheet_name
        );

        Ok(WorksheetData {
            name: sheet_name,
            rows,
        })
    }

    fn extract_row(
        worksheet: &umya_spreadsheet::Worksheet,
        row_idx: u32,
        max_col: u32,
    ) -> RowData {
        let mut cells = Vec::new();

        for col_idx in 1..=max_col {
            let cell_data = Self::extract_cell(worksheet, col_idx, row_idx);
            cells.push(cell_data);
        }

        RowData {
            row_index: (row_idx - 1) as usize,
            cells,
        }
    }

    fn extract_cell(
        worksheet: &umya_spreadsheet::Worksheet,
        col_idx: u32,
        row_idx: u32,
    ) -> CellData {
        let content = worksheet.get_value((col_idx, row_idx));
        let style = Self::extract_cell_style(worksheet, col_idx, row_idx);
        let merge_info = Self::extract_merge_info(worksheet, col_idx, row_idx);

        CellData {
            column_index: (col_idx - 1) as usize,
            content,
            style,
            merge_info,
        }
    }

    fn extract_cell_style(
        worksheet: &umya_spreadsheet::Worksheet,
        col_idx: u32,
        row_idx: u32,
    ) -> CellStyle {
        let cell = worksheet.get_cell((col_idx, row_idx));

        let mut style = CellStyle::default();

        if let Some(cell) = cell {
            if let Some(fill) = cell.get_style().get_fill() {
                if let Some(pattern_fill) = fill.get_pattern_fill() {
                    if let Some(fg_color) = pattern_fill.get_foreground_color() {
                        if let Some(rgb) = Self::extract_rgb_color(fg_color) {
                            style.background_color = Some(rgb);
                        }
                    }
                }
            }

            if let Some(font) = cell.get_style().get_font() {
                if let Some(rgb) = Self::extract_rgb_color(font.get_color()) {
                    style.font_color = Some(rgb);
                }

                style.bold = *font.get_bold();
                style.italic = *font.get_italic();
            }
        }

        style
    }

    fn extract_rgb_color(color: &umya_spreadsheet::Color) -> Option<RgbColor> {
        let rgb_str = color.get_argb();
        if rgb_str.len() >= 8 {
            match (
                u8::from_str_radix(&rgb_str[2..4], 16),
                u8::from_str_radix(&rgb_str[4..6], 16),
                u8::from_str_radix(&rgb_str[6..8], 16),
            ) {
                (Ok(r), Ok(g), Ok(b)) => {
                    return Some(RgbColor::new(r, g, b));
                }
                _ => {
                    tracing::warn!("Failed to parse RGB color from: {}", rgb_str);
                }
            }
        } else if !rgb_str.is_empty() {
            tracing::warn!("Invalid ARGB format (too short): {}", rgb_str);
        }

        None
    }

    fn extract_merge_info(
        worksheet: &umya_spreadsheet::Worksheet,
        col_idx: u32,
        row_idx: u32,
    ) -> Option<MergeInfo> {
        for merge_cell in worksheet.get_merge_cells() {
            // 获取合并范围的坐标
            let start_col = *merge_cell.get_coordinate_start_col()?.get_num();
            let start_row = *merge_cell.get_coordinate_start_row()?.get_num();
            let end_col = *merge_cell.get_coordinate_end_col()?.get_num();
            let end_row = *merge_cell.get_coordinate_end_row()?.get_num();

            if col_idx >= start_col
                && col_idx <= end_col
                && row_idx >= start_row
                && row_idx <= end_row
            {
                return Some(MergeInfo {
                    start_col: (start_col - 1) as usize,
                    end_col: (end_col - 1) as usize,
                    start_row: (start_row - 1) as usize,
                    end_row: (end_row - 1) as usize,
                });
            }
        }

        None
    }
}
