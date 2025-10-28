// Merge State-based Recognition Rule
use crate::engine::RecognitionRule;
use crate::models::RowData;

/// 合并范围要求
#[derive(Debug, Clone)]
pub enum MergeRequirement {
    /// 必须合并，跨越指定列范围
    MergedAcross { start_col: usize, end_col: usize },
    /// 必须合并，至少跨越指定列数
    MergedMinColumns(usize),
    /// 不能合并
    NotMerged,
}

/// 基于单元格合并状态的识别规则
/// 
/// 该规则通过检查单元格的合并状态来判断行类型。
/// 可以检查是否合并、合并的列范围等。
#[derive(Debug, Clone)]
pub struct MergeStateRule {
    /// 规则名称
    pub name: String,
    /// 合并范围要求
    pub merge_requirement: MergeRequirement,
}

impl MergeStateRule {
    /// 创建新的合并状态规则
    /// 
    /// # Arguments
    /// 
    /// * `name` - 规则名称
    /// * `merge_requirement` - 合并范围要求
    pub fn new(name: String, merge_requirement: MergeRequirement) -> Self {
        Self {
            name,
            merge_requirement,
        }
    }
}

impl RecognitionRule for MergeStateRule {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn matches(&self, row_data: &RowData) -> bool {
        match &self.merge_requirement {
            MergeRequirement::MergedAcross { start_col, end_col } => {
                // 检查是否有单元格合并且跨越指定的列范围
                row_data.cells.iter().any(|cell| {
                    if let Some(merge_info) = &cell.merge_info {
                        merge_info.start_col == *start_col && merge_info.end_col == *end_col
                    } else {
                        false
                    }
                })
            }
            MergeRequirement::MergedMinColumns(min_cols) => {
                // 检查是否有单元格合并且至少跨越指定列数
                row_data.cells.iter().any(|cell| {
                    if let Some(merge_info) = &cell.merge_info {
                        merge_info.column_span() >= *min_cols
                    } else {
                        false
                    }
                })
            }
            MergeRequirement::NotMerged => {
                // 检查所有单元格都没有合并
                row_data.cells.iter().all(|cell| cell.merge_info.is_none())
            }
        }
    }
    
    fn clone_box(&self) -> Box<dyn RecognitionRule> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CellData, CellStyle, MergeInfo};

    #[test]
    fn test_merged_across_exact_range() {
        let rule = MergeStateRule::new(
            "merged_rule".to_string(),
            MergeRequirement::MergedAcross {
                start_col: 0,
                end_col: 10,
            },
        );
        
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "Merged Cell".to_string(),
                    style: CellStyle::default(),
                    merge_info: Some(MergeInfo {
                        start_col: 0,
                        end_col: 10,
                        start_row: 0,
                        end_row: 0,
                    }),
                },
            ],
        };
        
        assert!(rule.matches(&row));
        
        // 不同的合并范围应该不匹配
        let row2 = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "Merged Cell".to_string(),
                    style: CellStyle::default(),
                    merge_info: Some(MergeInfo {
                        start_col: 0,
                        end_col: 5, // 不同的结束列
                        start_row: 0,
                        end_row: 0,
                    }),
                },
            ],
        };
        
        assert!(!rule.matches(&row2));
    }
    
    #[test]
    fn test_merged_min_columns() {
        let rule = MergeStateRule::new(
            "min_cols_rule".to_string(),
            MergeRequirement::MergedMinColumns(5),
        );
        
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "Merged Cell".to_string(),
                    style: CellStyle::default(),
                    merge_info: Some(MergeInfo {
                        start_col: 0,
                        end_col: 6, // 跨越7列
                        start_row: 0,
                        end_row: 0,
                    }),
                },
            ],
        };
        
        assert!(rule.matches(&row));
        
        // 少于最小列数应该不匹配
        let row2 = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "Merged Cell".to_string(),
                    style: CellStyle::default(),
                    merge_info: Some(MergeInfo {
                        start_col: 0,
                        end_col: 3, // 跨越4列
                        start_row: 0,
                        end_row: 0,
                    }),
                },
            ],
        };
        
        assert!(!rule.matches(&row2));
    }
    
    #[test]
    fn test_not_merged() {
        let rule = MergeStateRule::new(
            "not_merged_rule".to_string(),
            MergeRequirement::NotMerged,
        );
        
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "Cell 1".to_string(),
                    style: CellStyle::default(),
                    merge_info: None,
                },
                CellData {
                    column_index: 1,
                    content: "Cell 2".to_string(),
                    style: CellStyle::default(),
                    merge_info: None,
                },
            ],
        };
        
        assert!(rule.matches(&row));
        
        // 有合并的单元格应该不匹配
        let row2 = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "Merged Cell".to_string(),
                    style: CellStyle::default(),
                    merge_info: Some(MergeInfo {
                        start_col: 0,
                        end_col: 1,
                        start_row: 0,
                        end_row: 0,
                    }),
                },
            ],
        };
        
        assert!(!rule.matches(&row2));
    }
    
    #[test]
    fn test_partial_merge() {
        let rule = MergeStateRule::new(
            "merged_rule".to_string(),
            MergeRequirement::MergedMinColumns(3),
        );
        
        // 部分单元格合并的情况
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "Normal Cell".to_string(),
                    style: CellStyle::default(),
                    merge_info: None,
                },
                CellData {
                    column_index: 1,
                    content: "Merged Cell".to_string(),
                    style: CellStyle::default(),
                    merge_info: Some(MergeInfo {
                        start_col: 1,
                        end_col: 5, // 跨越5列
                        start_row: 0,
                        end_row: 0,
                    }),
                },
            ],
        };
        
        assert!(rule.matches(&row));
    }
}
