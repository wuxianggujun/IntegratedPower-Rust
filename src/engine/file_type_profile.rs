// File Type Profile Configuration
use crate::engine::{
    ColorRule, CompositeLogic, CompositeRule, MergeRequirement, MergeStateRule,
    RecognitionRule, TextPattern, TextPatternRule,
};
use crate::models::{RgbColor, RowType};

/// 行类型的完整定义，包含识别规则和元数据
pub struct RowTypeDefinition {
    /// 行类型
    pub row_type: RowType,
    /// 显示名称
    pub display_name: String,
    /// 描述
    pub description: String,
    /// 识别规则
    pub rule: Box<dyn RecognitionRule>,
    /// 优先级 (0-9, 9最高)
    pub priority: u8,
}

impl RowTypeDefinition {
    /// 创建新的行类型定义
    pub fn new(
        row_type: RowType,
        display_name: String,
        description: String,
        rule: Box<dyn RecognitionRule>,
        priority: u8,
    ) -> Self {
        Self {
            row_type,
            display_name,
            description,
            rule,
            priority,
        }
    }
}

/// 文件类型配置，定义特定文件类型的所有行类型和识别规则
pub struct FileTypeProfile {
    /// 配置名称
    pub name: String,
    /// 配置描述
    pub description: String,
    /// 行类型定义列表 (按优先级排序)
    pub row_type_definitions: Vec<RowTypeDefinition>,
}

impl FileTypeProfile {
    /// 创建新的文件类型配置
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            row_type_definitions: Vec::new(),
        }
    }

    /// 添加行类型定义
    pub fn add_row_type(&mut self, definition: RowTypeDefinition) {
        self.row_type_definitions.push(definition);
        // 按优先级排序（高优先级在前）
        self.row_type_definitions
            .sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), String> {
        if self.row_type_definitions.is_empty() {
            return Err(format!(
                "File type profile '{}' has no row type definitions",
                self.name
            ));
        }
        Ok(())
    }

    /// 创建货物分析表配置
    ///
    /// 该配置定义了货物分析表的5种行类型：
    /// 1. 项目编号行 - 浅绿色背景 RGB(198, 239, 206)
    /// 2. 柜号行 - 灰蓝色背景 RGB(127, 150, 152) 或 B列包含"柜号:"
    /// 3. 表头行 - 浅灰色背景 RGB(217, 217, 217)
    /// 4. 合计行 - 浅粉色背景 RGB(255, 204, 253)，区分"单台合计"和"小计"
    /// 5. 数据行 - 白色背景或无背景色
    pub fn cargo_analysis() -> Self {
        let mut profile = Self::new(
            "cargo_analysis".to_string(),
            "货物分析表配置".to_string(),
        );

        // 1. 项目编号行 (优先级9)
        // 先匹配颜色，再匹配合并状态
        profile.add_row_type(RowTypeDefinition::new(
            RowType::ProjectNumber,
            "项目编号行".to_string(),
            "浅绿色背景，包含项目编号".to_string(),
            Box::new(CompositeRule::new(
                "project_number_rule".to_string(),
                vec![
                    Box::new(ColorRule::new(
                        "green_background".to_string(),
                        RgbColor::new(198, 239, 206),
                        None,
                    )),
                    Box::new(MergeStateRule::new(
                        "merged_across_columns".to_string(),
                        MergeRequirement::MergedAcross {
                            start_col: 0,
                            end_col: 10,
                        },
                    )),
                ],
                CompositeLogic::And,
            )),
            9,
        ));

        // 2. 柜号行 (优先级8)
        // 先匹配颜色，如果颜色不匹配则匹配文本
        profile.add_row_type(RowTypeDefinition::new(
            RowType::CabinetNumber,
            "柜号行".to_string(),
            "灰蓝色背景或B列包含'柜号:'".to_string(),
            Box::new(CompositeRule::new(
                "cabinet_number_rule".to_string(),
                vec![
                    Box::new(ColorRule::new(
                        "gray_blue_background".to_string(),
                        RgbColor::new(127, 150, 152),
                        None,
                    )),
                    Box::new(TextPatternRule::new(
                        "cabinet_text".to_string(),
                        1, // B列
                        TextPattern::Contains("柜号:".to_string()),
                        false,
                    )),
                ],
                CompositeLogic::Or,
            )),
            8,
        ));

        // 3. 表头行 (优先级7)
        profile.add_row_type(RowTypeDefinition::new(
            RowType::Header,
            "表头行".to_string(),
            "浅灰色背景".to_string(),
            Box::new(ColorRule::new(
                "gray_background".to_string(),
                RgbColor::new(217, 217, 217),
                None,
            )),
            7,
        ));

        // 4. 单台合计行 (优先级6)
        // 先匹配颜色，再匹配文本
        profile.add_row_type(RowTypeDefinition::new(
            RowType::UnitTotal,
            "单台合计行".to_string(),
            "浅粉色背景，包含'单台合计'".to_string(),
            Box::new(CompositeRule::new(
                "unit_total_rule".to_string(),
                vec![
                    Box::new(ColorRule::new(
                        "pink_background".to_string(),
                        RgbColor::new(255, 204, 253),
                        None,
                    )),
                    Box::new(TextPatternRule::new(
                        "unit_total_text".to_string(),
                        0, // A列
                        TextPattern::Contains("单台合计".to_string()),
                        false,
                    )),
                ],
                CompositeLogic::And,
            )),
            6,
        ));

        // 5. 小计行 (优先级6)
        // 先匹配颜色，再匹配文本
        profile.add_row_type(RowTypeDefinition::new(
            RowType::SubTotal,
            "小计行".to_string(),
            "浅粉色背景，包含'小计'".to_string(),
            Box::new(CompositeRule::new(
                "subtotal_rule".to_string(),
                vec![
                    Box::new(ColorRule::new(
                        "pink_background".to_string(),
                        RgbColor::new(255, 204, 253),
                        None,
                    )),
                    Box::new(TextPatternRule::new(
                        "subtotal_text".to_string(),
                        0, // A列
                        TextPattern::Contains("小计".to_string()),
                        false,
                    )),
                ],
                CompositeLogic::And,
            )),
            6,
        ));

        // 6. 总计行 (优先级5)
        // 先匹配颜色，再匹配文本
        profile.add_row_type(RowTypeDefinition::new(
            RowType::GrandTotal,
            "总计行".to_string(),
            "浅粉色背景，包含'总计'".to_string(),
            Box::new(CompositeRule::new(
                "grand_total_rule".to_string(),
                vec![
                    Box::new(ColorRule::new(
                        "pink_background".to_string(),
                        RgbColor::new(255, 204, 253),
                        None,
                    )),
                    Box::new(TextPatternRule::new(
                        "grand_total_text".to_string(),
                        0, // A列
                        TextPattern::Contains("总计".to_string()),
                        false,
                    )),
                ],
                CompositeLogic::And,
            )),
            5,
        ));

        // 7. 数据行 (优先级1，最低)
        profile.add_row_type(RowTypeDefinition::new(
            RowType::Data,
            "数据行".to_string(),
            "无背景色或白色背景".to_string(),
            Box::new(ColorRule::new(
                "white_or_no_background".to_string(),
                RgbColor::white(),
                None,
            )),
            1,
        ));

        profile
    }

    /// 创建辅材配置（占位实现，后续扩展）
    pub fn auxiliary_material() -> Self {
        let mut profile = Self::new(
            "auxiliary_material".to_string(),
            "辅材配置（待实现）".to_string(),
        );

        // 占位：添加一个默认的数据行类型
        profile.add_row_type(RowTypeDefinition::new(
            RowType::Data,
            "数据行".to_string(),
            "默认数据行".to_string(),
            Box::new(ColorRule::new(
                "default_rule".to_string(),
                RgbColor::white(),
                None,
            )),
            1,
        ));

        profile
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_analysis_profile_creation() {
        let profile = FileTypeProfile::cargo_analysis();

        assert_eq!(profile.name, "cargo_analysis");
        assert_eq!(profile.description, "货物分析表配置");

        // 应该有7种行类型
        assert_eq!(profile.row_type_definitions.len(), 7);

        // 验证配置
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn test_profile_priority_sorting() {
        let profile = FileTypeProfile::cargo_analysis();

        // 验证按优先级排序（高优先级在前）
        let priorities: Vec<u8> = profile
            .row_type_definitions
            .iter()
            .map(|def| def.priority)
            .collect();

        // 检查是否降序排列
        for i in 0..priorities.len() - 1 {
            assert!(priorities[i] >= priorities[i + 1]);
        }
    }

    #[test]
    fn test_profile_validation() {
        let empty_profile = FileTypeProfile::new("test".to_string(), "test".to_string());

        // 空配置应该验证失败
        assert!(empty_profile.validate().is_err());

        // 有定义的配置应该验证成功
        let cargo_profile = FileTypeProfile::cargo_analysis();
        assert!(cargo_profile.validate().is_ok());
    }

    #[test]
    fn test_auxiliary_material_profile() {
        let profile = FileTypeProfile::auxiliary_material();

        assert_eq!(profile.name, "auxiliary_material");
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn test_add_row_type() {
        let mut profile = FileTypeProfile::new("test".to_string(), "test".to_string());

        profile.add_row_type(RowTypeDefinition::new(
            RowType::Header,
            "Header".to_string(),
            "Test header".to_string(),
            Box::new(ColorRule::new(
                "test".to_string(),
                RgbColor::white(),
                None,
            )),
            5,
        ));

        profile.add_row_type(RowTypeDefinition::new(
            RowType::Data,
            "Data".to_string(),
            "Test data".to_string(),
            Box::new(ColorRule::new(
                "test".to_string(),
                RgbColor::white(),
                None,
            )),
            3,
        ));

        assert_eq!(profile.row_type_definitions.len(), 2);

        // 验证排序：优先级5应该在优先级3之前
        assert_eq!(profile.row_type_definitions[0].priority, 5);
        assert_eq!(profile.row_type_definitions[1].priority, 3);
    }
}
