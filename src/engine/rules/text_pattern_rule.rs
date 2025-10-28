// Text Pattern-based Recognition Rule
use crate::engine::RecognitionRule;
use crate::models::RowData;
use regex::Regex;

/// 文本模式类型
#[derive(Debug, Clone)]
pub enum TextPattern {
    /// 精确匹配
    Exact(String),
    /// 包含子串
    Contains(String),
    /// 正则表达式
    Regex(String),
}

/// 基于文本模式的识别规则
/// 
/// 该规则通过检查指定列的单元格内容来判断行类型。
/// 支持精确匹配、包含匹配和正则表达式匹配三种模式。
#[derive(Debug, Clone)]
pub struct TextPatternRule {
    /// 规则名称
    pub name: String,
    /// 要检查的列索引
    pub column_index: usize,
    /// 文本模式
    pub pattern: TextPattern,
    /// 是否区分大小写
    pub case_sensitive: bool,
}

impl TextPatternRule {
    /// 创建新的文本模式规则
    /// 
    /// # Arguments
    /// 
    /// * `name` - 规则名称
    /// * `column_index` - 要检查的列索引
    /// * `pattern` - 文本模式
    /// * `case_sensitive` - 是否区分大小写
    pub fn new(
        name: String,
        column_index: usize,
        pattern: TextPattern,
        case_sensitive: bool,
    ) -> Self {
        Self {
            name,
            column_index,
            pattern,
            case_sensitive,
        }
    }
}

impl RecognitionRule for TextPatternRule {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn matches(&self, row_data: &RowData) -> bool {
        // 获取指定列的单元格
        let cell = match row_data.get_cell(self.column_index) {
            Some(c) => c,
            None => return false,
        };
        
        // 获取单元格内容并trim
        let content = cell.content.trim();
        
        // 根据大小写敏感设置处理内容
        let content_to_match = if self.case_sensitive {
            content.to_string()
        } else {
            content.to_lowercase()
        };
        
        // 根据模式类型进行匹配
        match &self.pattern {
            TextPattern::Exact(target) => {
                if self.case_sensitive {
                    content == target
                } else {
                    content_to_match == target.to_lowercase()
                }
            }
            TextPattern::Contains(substring) => {
                if self.case_sensitive {
                    content.contains(substring)
                } else {
                    content_to_match.contains(&substring.to_lowercase())
                }
            }
            TextPattern::Regex(pattern) => {
                // 尝试编译正则表达式
                let regex_result = if self.case_sensitive {
                    Regex::new(pattern)
                } else {
                    Regex::new(&format!("(?i){}", pattern))
                };
                
                match regex_result {
                    Ok(regex) => regex.is_match(content),
                    Err(e) => {
                        // 记录错误但不崩溃
                        eprintln!("Invalid regex pattern '{}' in rule '{}': {}", pattern, self.name, e);
                        false
                    }
                }
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
    use crate::models::{CellData, CellStyle};

    #[test]
    fn test_exact_match_case_sensitive() {
        let rule = TextPatternRule::new(
            "exact_rule".to_string(),
            1,
            TextPattern::Exact("柜号:".to_string()),
            true,
        );
        
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "A".to_string(),
                    style: CellStyle::default(),
                    merge_info: None,
                },
                CellData {
                    column_index: 1,
                    content: "柜号:".to_string(),
                    style: CellStyle::default(),
                    merge_info: None,
                },
            ],
        };
        
        assert!(rule.matches(&row));
        
        // 大小写不同应该不匹配
        let row2 = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "A".to_string(),
                    style: CellStyle::default(),
                    merge_info: None,
                },
                CellData {
                    column_index: 1,
                    content: "柜号：".to_string(), // 不同的冒号
                    style: CellStyle::default(),
                    merge_info: None,
                },
            ],
        };
        
        assert!(!rule.matches(&row2));
    }
    
    #[test]
    fn test_contains_case_insensitive() {
        let rule = TextPatternRule::new(
            "contains_rule".to_string(),
            0,
            TextPattern::Contains("小计".to_string()),
            false,
        );
        
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "  小计: 100  ".to_string(), // 带空格
                    style: CellStyle::default(),
                    merge_info: None,
                },
            ],
        };
        
        assert!(rule.matches(&row));
    }
    
    #[test]
    fn test_regex_pattern() {
        let rule = TextPatternRule::new(
            "regex_rule".to_string(),
            0,
            TextPattern::Regex(r"^G\d{3}E-\d+-\d+$".to_string()),
            true,
        );
        
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "G00E-500009085-00011".to_string(),
                    style: CellStyle::default(),
                    merge_info: None,
                },
            ],
        };
        
        assert!(rule.matches(&row));
        
        let row2 = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "Invalid".to_string(),
                    style: CellStyle::default(),
                    merge_info: None,
                },
            ],
        };
        
        assert!(!rule.matches(&row2));
    }
    
    #[test]
    fn test_invalid_regex_returns_false() {
        let rule = TextPatternRule::new(
            "bad_regex_rule".to_string(),
            0,
            TextPattern::Regex("[invalid(".to_string()), // 无效的正则
            true,
        );
        
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "Test".to_string(),
                    style: CellStyle::default(),
                    merge_info: None,
                },
            ],
        };
        
        // 无效的正则应该返回false而不是崩溃
        assert!(!rule.matches(&row));
    }
}
