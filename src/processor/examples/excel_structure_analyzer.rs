use crate::error::Result;
use crate::processor::trait_def::DataProcessor;
use async_trait::async_trait;
use polars::prelude::*;
use std::path::Path;
use umya_spreadsheet as umya;

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
    pub fn analyze_excel_structure(&self, file_path: &Path, selected_sheet: Option<&str>, analyze_colors: bool) -> Result<()> {
        crate::log_info!("开始分析 Excel 文件: {}", file_path.display());

        // 打开 Excel 文件（umya）
        let book = umya::reader::xlsx::read(file_path)
            .map_err(|e| crate::error::AppError::excel_error(format!("无法打开 Excel 文件: {}", e)))?;

        // 获取 sheet 名称列表
        let names: Vec<String> = book
            .get_sheet_collection()
            .iter()
            .map(|ws| ws.get_name().to_string())
            .collect();
        if names.is_empty() {
            crate::log_info!("Excel 文件中没有 Sheet");
            return Ok(());
        }

        // 选择要分析的 sheet
        let sheet_to_analyze = selected_sheet
            .filter(|s| names.iter().any(|n| n == s))
            .map(|s| s.to_string())
            .unwrap_or_else(|| names[0].clone());

        crate::log_info!("将分析 Sheet: {}", sheet_to_analyze);
        self.analyze_sheet(&book, &sheet_to_analyze)?;

        if analyze_colors {
            match self.analyze_sheet_colors_ws(&book, &sheet_to_analyze) {
                Ok(_) => {}
                Err(e) => {
                    crate::log_warning!("颜色分析失败: {}（可关闭颜色分析以跳过）", e);
                }
            }
        } else {
            crate::log_info!("已禁用颜色分析（可在选项中开启）");
        }

        crate::log_info!("Excel 文件结构分析完成");
        Ok(())
    }

    /// 分析单个 Sheet 的结构
    fn analyze_sheet(&self, book: &umya::Spreadsheet, sheet_name: &str) -> Result<()> {
        crate::log_info!("=== 分析 Sheet: {} ===", sheet_name);

        // 获取工作表
        let ws = book
            .get_sheet_collection()
            .iter()
            .find(|ws| ws.get_name() == sheet_name)
            .ok_or_else(|| crate::error::AppError::excel_error(format!("未找到工作表: {}", sheet_name)))?;

        let (height, width) = Self::worksheet_size(ws);
        crate::log_info!("Sheet 尺寸: {} 行 × {} 列", height, width);

        // 直接输出完整表格内容
        self.dump_full_sheet(ws, height, width)?;

        crate::log_info!("=== Sheet {} 分析完成 ===", sheet_name);
        Ok(())
    }

    /// 输出完整表格内容（逐行逐列）
    fn dump_full_sheet(&self, ws: &umya::Worksheet, height: u32, width: u32) -> Result<()> {
        if height == 0 || width == 0 {
            crate::log_info!("工作表为空");
            return Ok(());
        }

        crate::log_info!("表格完整内容 ({} 行 × {} 列):", height, width);

        for r in 1..=height {
            let mut row_vals: Vec<String> = Vec::with_capacity(width as usize);
            for c in 1..=width {
                let v = ws.get_value((c, r));
                row_vals.push(v);
            }
            crate::log_info!("第{}行: {:?}", r, row_vals);
        }

        Ok(())
    }

    fn worksheet_size(ws: &umya::Worksheet) -> (u32, u32) {
        let rows = ws.get_highest_row();
        let cols = ws.get_highest_column();
        (rows, cols)
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

impl ExcelStructureAnalyzer {
    /// 使用 umya-spreadsheet 统计背景色分布
    fn analyze_sheet_colors_ws(&self, book: &umya::Spreadsheet, sheet_name: &str) -> Result<()> {
        use std::collections::HashMap;
        let ws = book
            .get_sheet_collection()
            .iter()
            .find(|ws| ws.get_name() == sheet_name)
            .ok_or_else(|| crate::error::AppError::excel_error(format!("未找到工作表: {}", sheet_name)))?;

        let (height, width) = Self::worksheet_size(ws);
        let mut color_counts: HashMap<String, usize> = HashMap::new();
        let mut row_color_map: HashMap<u32, HashMap<String, usize>> = HashMap::new();
        let theme = book.get_theme();

        for r in 1..=height {
            for c in 1..=width {
                let style = ws.get_style((c, r));
                let mut hex: Option<String> = None;
                if let Some(fill) = style.get_fill() {
                    if let Some(pat) = fill.get_pattern_fill() {
                        if let Some(fg) = pat.get_foreground_color() {
                            let cow = fg.get_argb_with_theme(theme);
                            let argb = cow.as_ref();
                            if !argb.is_empty() {
                                hex = Some(Self::argb_to_rgb_hex(argb));
                            }
                        }
                        if hex.is_none() {
                            if let Some(bg) = pat.get_background_color() {
                                let cow = bg.get_argb_with_theme(theme);
                                let argb = cow.as_ref();
                                if !argb.is_empty() {
                                    hex = Some(Self::argb_to_rgb_hex(argb));
                                }
                            }
                        }
                    }
                }
                if let Some(h) = hex {
                    *color_counts.entry(h.clone()).or_insert(0) += 1;
                    let entry = row_color_map.entry(r).or_default();
                    *entry.entry(h).or_insert(0) += 1;
                }
            }
        }

        // 若未检测到颜色，打印部分单元格样式调试信息，帮助排查（只打前 10 行 × 5 列）
        if color_counts.is_empty() {
            let max_r = height.min(10);
            let max_c = width.min(5);
            for r in 1..=max_r {
                for c in 1..=max_c {
                    let style = ws.get_style((c, r));
                    if let Some(fill) = style.get_fill() {
                        if let Some(pat) = fill.get_pattern_fill() {
                            let fg = pat
                                .get_foreground_color()
                                .map(|cc| cc.get_argb_with_theme(theme).to_string())
                                .unwrap_or_default();
                            let bg = pat
                                .get_background_color()
                                .map(|cc| cc.get_argb_with_theme(theme).to_string())
                                .unwrap_or_default();
                            if !fg.is_empty() || !bg.is_empty() {
                                crate::log_debug!("样式调试 r{},c{} fg_argb={} bg_argb={}", r, c, fg, bg);
                            }
                        }
                    }
                }
            }
        }

        if color_counts.is_empty() {
            crate::log_warning!("未检测到有效的背景颜色（可能为主题色未展开或无填充）");
        } else {
            crate::log_info!("颜色分布（Top 8）：");
            let mut items: Vec<(String, usize)> = color_counts.into_iter().collect();
            items.sort_by(|a, b| b.1.cmp(&a.1));
            for (i, (col, cnt)) in items.into_iter().take(8).enumerate() {
                crate::log_info!("  {}. #{}: {} 个单元格", i + 1, col, cnt);
            }
        }

        let mut dominants: Vec<(u32, String)> = Vec::new();
        for (row, cmap) in row_color_map {
            if let Some((col, cnt)) = cmap.into_iter().max_by_key(|(_, c)| *c) {
                if cnt as u32 > (width / 2).max(1) { dominants.push((row, col)); }
            }
        }
        if !dominants.is_empty() {
            dominants.sort_by_key(|(r, _)| *r);
            crate::log_info!("行主色（>50% 单元格同色）：");
            for (r, hex) in dominants { crate::log_info!("  第{}行: #{}", r, hex); }
        }

        Ok(())
    }

    fn argb_to_rgb_hex(argb: &str) -> String {
        let s = argb.trim_start_matches('#');
        if s.len() == 8 { s[2..].to_uppercase() } else { s.to_uppercase() }
    }
}
