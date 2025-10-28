// Composite Recognition Rule
use crate::engine::RecognitionRule;
use crate::models::RowData;

/// 组合逻辑类型
#[derive(Debug, Clone, Copy)]
pub enum CompositeLogic {
    /// 所有规则都必须匹配 (AND)
    And,
    /// 至少一个规则匹配 (OR)
    Or,
}

/// 组合多个规则的复合规则
/// 
/// 该规则允许将多个识别规则组合在一起，使用AND或OR逻辑。
/// 支持嵌套组合，可以创建复杂的识别条件。
#[derive(Clone)]
pub struct CompositeRule {
    /// 规则名称
    pub name: String,
    /// 子规则列表
    pub rules: Vec<Box<dyn RecognitionRule>>,
    /// 组合逻辑
    pub logic: CompositeLogic,
}

impl CompositeRule {
    /// 创建新的组合规则
    /// 
    /// # Arguments
    /// 
    /// * `name` - 规则名称
    /// * `rules` - 子规则列表
    /// * `logic` - 组合逻辑 (AND或OR)
    pub fn new(
        name: String,
        rules: Vec<Box<dyn RecognitionRule>>,
        logic: CompositeLogic,
    ) -> Self {
        Self { name, rules, logic }
    }
    
    /// 创建AND组合规则
    /// 
    /// # Arguments
    /// 
    /// * `name` - 规则名称
    /// * `rules` - 子规则列表
    pub fn and(name: String, rules: Vec<Box<dyn RecognitionRule>>) -> Self {
        Self::new(name, rules, CompositeLogic::And)
    }
    
    /// 创建OR组合规则
    /// 
    /// # Arguments
    /// 
    /// * `name` - 规则名称
    /// * `rules` - 子规则列表
    pub fn or(name: String, rules: Vec<Box<dyn RecognitionRule>>) -> Self {
        Self::new(name, rules, CompositeLogic::Or)
    }
}

impl std::fmt::Debug for CompositeRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompositeRule")
            .field("name", &self.name)
            .field("logic", &self.logic)
            .field("rules_count", &self.rules.len())
            .finish()
    }
}

impl RecognitionRule for CompositeRule {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn matches(&self, row_data: &RowData) -> bool {
        match self.logic {
            CompositeLogic::And => {
                // 所有规则都必须匹配
                self.rules.iter().all(|rule| rule.matches(row_data))
            }
            CompositeLogic::Or => {
                // 至少一个规则匹配
                self.rules.iter().any(|rule| rule.matches(row_data))
            }
        }
    }
    
    fn confidence(&self) -> f32 {
        if self.rules.is_empty() {
            return 0.0;
        }
        
        match self.logic {
            CompositeLogic::And => {
                // AND逻辑：取所有规则置信度的平均值
                let sum: f32 = self.rules.iter().map(|r| r.confidence()).sum();
                sum / self.rules.len() as f32
            }
            CompositeLogic::Or => {
                // OR逻辑：取最高的置信度
                self.rules
                    .iter()
                    .map(|r| r.confidence())
                    .fold(0.0f32, |a, b| a.max(b))
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
    use crate::engine::rules::{ColorRule, TextPattern, TextPatternRule};
    use crate::models::{CellData, CellStyle, RgbColor};

    #[test]
    fn test_and_logic_all_match() {
        let color_rule = Box::new(ColorRule::new(
            "color".to_string(),
            RgbColor::new(255, 204, 253),
            Some(0),
        ));
        
        let text_rule = Box::new(TextPatternRule::new(
            "text".to_string(),
            0,
            TextPattern::Contains("小计".to_string()),
            false,
        ));
        
        let composite = CompositeRule::and(
            "and_rule".to_string(),
            vec![color_rule, text_rule],
        );
        
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "小计: 100".to_string(),
                    style: CellStyle {
                        background_color: Some(RgbColor::new(255, 204, 253)),
                        ..Default::default()
                    },
                    merge_info: None,
                },
            ],
        };
        
        assert!(composite.matches(&row));
    }
    
    #[test]
    fn test_and_logic_one_fails() {
        let color_rule = Box::new(ColorRule::new(
            "color".to_string(),
            RgbColor::new(255, 204, 253),
            Some(0),
        ));
        
        let text_rule = Box::new(TextPatternRule::new(
            "text".to_string(),
            0,
            TextPattern::Contains("单台合计".to_string()),
            false,
        ));
        
        let composite = CompositeRule::and(
            "and_rule".to_string(),
            vec![color_rule, text_rule],
        );
        
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "小计: 100".to_string(), // 文本不匹配
                    style: CellStyle {
                        background_color: Some(RgbColor::new(255, 204, 253)),
                        ..Default::default()
                    },
                    merge_info: None,
                },
            ],
        };
        
        assert!(!composite.matches(&row));
    }
    
    #[test]
    fn test_or_logic_one_matches() {
        let color_rule = Box::new(ColorRule::new(
            "color".to_string(),
            RgbColor::new(127, 150, 152),
            Some(0),
        ));
        
        let text_rule = Box::new(TextPatternRule::new(
            "text".to_string(),
            1,
            TextPattern::Contains("柜号:".to_string()),
            false,
        ));
        
        let composite = CompositeRule::or(
            "or_rule".to_string(),
            vec![color_rule, text_rule],
        );
        
        // 只有文本匹配
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
                    content: "柜号: 1-1".to_string(),
                    style: CellStyle::default(),
                    merge_info: None,
                },
            ],
        };
        
        assert!(composite.matches(&row));
    }
    
    #[test]
    fn test_or_logic_none_match() {
        let color_rule = Box::new(ColorRule::new(
            "color".to_string(),
            RgbColor::new(127, 150, 152),
            Some(0),
        ));
        
        let text_rule = Box::new(TextPatternRule::new(
            "text".to_string(),
            1,
            TextPattern::Contains("柜号:".to_string()),
            false,
        ));
        
        let composite = CompositeRule::or(
            "or_rule".to_string(),
            vec![color_rule, text_rule],
        );
        
        // 都不匹配
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
                    content: "Other text".to_string(),
                    style: CellStyle::default(),
                    merge_info: None,
                },
            ],
        };
        
        assert!(!composite.matches(&row));
    }
    
    #[test]
    fn test_nested_composite() {
        // 创建嵌套的组合规则: (Color AND Text1) OR Text2
        let inner_and = Box::new(CompositeRule::and(
            "inner_and".to_string(),
            vec![
                Box::new(ColorRule::new(
                    "color".to_string(),
                    RgbColor::new(198, 239, 206),
                    Some(0),
                )),
                Box::new(TextPatternRule::new(
                    "text1".to_string(),
                    0,
                    TextPattern::Contains("G00E".to_string()),
                    false,
                )),
            ],
        ));
        
        let outer_or = CompositeRule::or(
            "outer_or".to_string(),
            vec![
                inner_and,
                Box::new(TextPatternRule::new(
                    "text2".to_string(),
                    0,
                    TextPattern::Contains("项目".to_string()),
                    false,
                )),
            ],
        );
        
        // 测试匹配第二个条件
        let row = RowData {
            row_index: 0,
            cells: vec![
                CellData {
                    column_index: 0,
                    content: "项目编号".to_string(),
                    style: CellStyle::default(),
                    merge_info: None,
                },
            ],
        };
        
        assert!(outer_or.matches(&row));
    }
}
