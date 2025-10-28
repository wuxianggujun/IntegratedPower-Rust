// Color-based Recognition Rule
use crate::engine::RecognitionRule;
use crate::models::{RgbColor, RowData};

/// 基于RGB颜色的识别规则
/// 
/// 该规则通过检查单元格的背景颜色来判断行类型。
/// 颜色匹配是精确的，不使用容差。
#[derive(Debug, Clone)]
pub struct ColorRule {
    /// 规则名称
    pub name: String,
    /// 目标RGB颜色
    pub target_color: RgbColor,
    /// 要检查的列索引 (None表示检查第一个非空单元格)
    pub column_index: Option<usize>,
}

impl ColorRule {
    /// 创建新的颜色规则
    /// 
    /// # Arguments
    /// 
    /// * `name` - 规则名称
    /// * `target_color` - 要匹配的目标颜色
    /// * `column_index` - 要检查的列索引，None表示检查第一个非空单元格
    pub fn new(name: String, target_color: RgbColor, column_index: Option<usize>) -> Self {
        Self {
            name,
            target_color,
            column_index,
        }
    }
}

impl RecognitionRule for ColorRule {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn matches(&self, row_data: &RowData) -> bool {
        // 获取要检查的单元格
        let cell = if let Some(col_idx) = self.column_index {
            row_data.get_cell(col_idx)
        } else {
            row_data.first_non_empty_cell()
        };
        
        // 如果找不到单元格，返回false
        let cell = match cell {
            Some(c) => c,
            None => return false,
        };
        
        // 获取单元格的背景颜色
        let cell_color = cell.style.background_color.unwrap_or(RgbColor::white());
        
        // 精确匹配颜色（不使用容差）
        cell_color == self.target_color
    }
    
    fn clone_box(&self) -> Box<dyn RecognitionRule> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CellData, CellStyle};

    #[test]
    fn test_color_rule_exact_match() {
        let rule = ColorRule::new(
            "test_rule".to_string(),
            RgbColor::new(198, 239, 206),
            Some(0),
        );
        
        let mut row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "Test".to_string(),
                    style: CellStyle {
                        background_color: Some(RgbColor::new(198, 239, 206)),
                        ..Default::default()
                    },
                    merge_info: None,
                },
            ],
        };
        
        assert!(rule.matches(&row));
        
        // 修改颜色，应该不匹配
        row.cells[0].style.background_color = Some(RgbColor::new(198, 239, 207));
        assert!(!rule.matches(&row));
    }
    
    #[test]
    fn test_color_rule_no_background_treated_as_white() {
        let rule = ColorRule::new(
            "white_rule".to_string(),
            RgbColor::white(),
            Some(0),
        );
        
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "Test".to_string(),
                    style: CellStyle {
                        background_color: None, // 无背景色
                        ..Default::default()
                    },
                    merge_info: None,
                },
            ],
        };
        
        assert!(rule.matches(&row));
    }
    
    #[test]
    fn test_color_rule_first_non_empty_cell() {
        let rule = ColorRule::new(
            "test_rule".to_string(),
            RgbColor::new(217, 217, 217),
            None, // 检查第一个非空单元格
        );
        
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "".to_string(), // 空单元格
                    style: CellStyle::default(),
                    merge_info: None,
                },
                CellData {
                    column_index: 1,
                    content: "Test".to_string(), // 第一个非空单元格
                    style: CellStyle {
                        background_color: Some(RgbColor::new(217, 217, 217)),
                        ..Default::default()
                    },
                    merge_info: None,
                },
            ],
        };
        
        assert!(rule.matches(&row));
    }
}
