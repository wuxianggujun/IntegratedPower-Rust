// Row Type Identifier - Main API
use crate::engine::FileTypeProfile;
use crate::models::{
    IdentificationStatistics, RowData, RowIdentificationResult, RowType, WorksheetData,
};

/// 行类型识别器，提供行类型识别的主要接口
pub struct RowTypeIdentifier {
    /// 文件类型配置
    profile: FileTypeProfile,
    /// 是否启用调试模式
    debug_mode: bool,
}

impl RowTypeIdentifier {
    /// 创建新的识别器
    ///
    /// # Arguments
    ///
    /// * `profile` - 文件类型配置
    pub fn new(profile: FileTypeProfile) -> Self {
        Self {
            profile,
            debug_mode: false,
        }
    }

    /// 启用调试模式
    ///
    /// # Arguments
    ///
    /// * `debug` - 是否启用调试模式
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug_mode = debug;
        self
    }

    /// 识别单行的类型
    ///
    /// 该方法遍历所有行类型定义，按优先级顺序评估规则，
    /// 返回第一个匹配的行类型。
    ///
    /// # Arguments
    ///
    /// * `row_data` - 行数据
    ///
    /// # Returns
    ///
    /// 识别结果，包含行类型、匹配的规则名称和置信度
    pub fn identify_row(&self, row_data: &RowData) -> RowIdentificationResult {
        if self.debug_mode {
            println!("Identifying row {}", row_data.row_index);
        }

        // 遍历所有行类型定义（已按优先级排序）
        for definition in &self.profile.row_type_definitions {
            if self.debug_mode {
                println!(
                    "  Evaluating rule '{}' for row type {:?} (priority {})",
                    definition.rule.name(),
                    definition.row_type,
                    definition.priority
                );
            }

            // 评估规则
            if definition.rule.matches(row_data) {
                if self.debug_mode {
                    println!("    ✓ Rule matched!");
                }

                return RowIdentificationResult {
                    row_index: row_data.row_index,
                    row_type: definition.row_type.clone(),
                    matched_rule: definition.rule.name().to_string(),
                    confidence: definition.rule.confidence(),
                };
            } else if self.debug_mode {
                println!("    ✗ Rule did not match");
            }
        }

        // 没有规则匹配，返回Unknown类型
        if self.debug_mode {
            println!("  No rules matched, returning Unknown");
        }

        RowIdentificationResult {
            row_index: row_data.row_index,
            row_type: RowType::Unknown,
            matched_rule: "none".to_string(),
            confidence: 0.0,
        }
    }

    /// 批量识别所有行的类型
    ///
    /// # Arguments
    ///
    /// * `worksheet_data` - 工作表数据
    ///
    /// # Returns
    ///
    /// 所有行的识别结果列表
    pub fn identify_all_rows(&self, worksheet_data: &WorksheetData) -> Vec<RowIdentificationResult> {
        if self.debug_mode {
            println!(
                "Identifying all rows in worksheet '{}' ({} rows)",
                worksheet_data.name,
                worksheet_data.row_count()
            );
        }

        worksheet_data
            .rows
            .iter()
            .map(|row| self.identify_row(row))
            .collect()
    }

    /// 获取识别统计信息
    ///
    /// # Arguments
    ///
    /// * `results` - 识别结果列表
    ///
    /// # Returns
    ///
    /// 统计信息，包含各类型行数、未知行数和成功率
    pub fn get_statistics(&self, results: &[RowIdentificationResult]) -> IdentificationStatistics {
        IdentificationStatistics::from_results(results)
    }

    /// 检查是否有多个相同优先级的规则匹配同一行
    ///
    /// 该方法用于调试和验证配置，会记录警告信息
    ///
    /// # Arguments
    ///
    /// * `row_data` - 行数据
    ///
    /// # Returns
    ///
    /// 所有匹配的行类型定义的索引列表
    fn find_all_matches(&self, row_data: &RowData) -> Vec<usize> {
        self.profile
            .row_type_definitions
            .iter()
            .enumerate()
            .filter(|(_, def)| def.rule.matches(row_data))
            .map(|(idx, _)| idx)
            .collect()
    }

    /// 检测并记录优先级冲突
    ///
    /// 当多个相同优先级的规则匹配同一行时，记录警告
    ///
    /// # Arguments
    ///
    /// * `row_data` - 行数据
    pub fn check_priority_conflicts(&self, row_data: &RowData) {
        let matches = self.find_all_matches(row_data);

        if matches.len() > 1 {
            // 检查是否有相同优先级的匹配
            let priorities: Vec<u8> = matches
                .iter()
                .map(|&idx| self.profile.row_type_definitions[idx].priority)
                .collect();

            for i in 0..priorities.len() - 1 {
                for j in i + 1..priorities.len() {
                    if priorities[i] == priorities[j] {
                        let def1 = &self.profile.row_type_definitions[matches[i]];
                        let def2 = &self.profile.row_type_definitions[matches[j]];

                        eprintln!(
                            "Warning: Multiple rules with same priority {} matched row {}: '{}' ({:?}) and '{}' ({:?})",
                            priorities[i],
                            row_data.row_index,
                            def1.rule.name(),
                            def1.row_type,
                            def2.rule.name(),
                            def2.row_type
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CellData, CellStyle, RgbColor};

    #[test]
    fn test_identify_row_project_number() {
        let profile = FileTypeProfile::cargo_analysis();
        let identifier = RowTypeIdentifier::new(profile);

        let row = RowData {
            row_index: 11,
            cells: vec![CellData {
                column_index: 0,
                content: "G00E-500009085-00011".to_string(),
                style: CellStyle {
                    background_color: Some(RgbColor::new(198, 239, 206)),
                    ..Default::default()
                },
                merge_info: Some(crate::models::MergeInfo {
                    start_col: 0,
                    end_col: 10,
                    start_row: 11,
                    end_row: 11,
                }),
            }],
        };

        let result = identifier.identify_row(&row);
        assert_eq!(result.row_type, RowType::ProjectNumber);
        assert_eq!(result.row_index, 11);
    }

    #[test]
    fn test_identify_row_header() {
        let profile = FileTypeProfile::cargo_analysis();
        let identifier = RowTypeIdentifier::new(profile);

        let row = RowData {
            row_index: 13,
            cells: vec![CellData {
                column_index: 0,
                content: "序号".to_string(),
                style: CellStyle {
                    background_color: Some(RgbColor::new(217, 217, 217)),
                    ..Default::default()
                },
                merge_info: None,
            }],
        };

        let result = identifier.identify_row(&row);
        assert_eq!(result.row_type, RowType::Header);
    }

    #[test]
    fn test_identify_row_data() {
        let profile = FileTypeProfile::cargo_analysis();
        let identifier = RowTypeIdentifier::new(profile);

        let row = RowData {
            row_index: 14,
            cells: vec![CellData {
                column_index: 0,
                content: "1".to_string(),
                style: CellStyle {
                    background_color: None, // 无背景色
                    ..Default::default()
                },
                merge_info: None,
            }],
        };

        let result = identifier.identify_row(&row);
        assert_eq!(result.row_type, RowType::Data);
    }

    #[test]
    fn test_identify_row_unknown() {
        let profile = FileTypeProfile::cargo_analysis();
        let identifier = RowTypeIdentifier::new(profile);

        let row = RowData {
            row_index: 0,
            cells: vec![CellData {
                column_index: 0,
                content: "Test".to_string(),
                style: CellStyle {
                    background_color: Some(RgbColor::new(100, 100, 100)), // 未定义的颜色
                    ..Default::default()
                },
                merge_info: None,
            }],
        };

        let result = identifier.identify_row(&row);
        assert_eq!(result.row_type, RowType::Unknown);
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn test_identify_all_rows() {
        let profile = FileTypeProfile::cargo_analysis();
        let identifier = RowTypeIdentifier::new(profile);

        let worksheet = WorksheetData {
            name: "Test".to_string(),
            rows: vec![
                RowData {
                    row_index: 0,
                    cells: vec![CellData {
                        column_index: 0,
                        content: "序号".to_string(),
                        style: CellStyle {
                            background_color: Some(RgbColor::new(217, 217, 217)),
                            ..Default::default()
                        },
                        merge_info: None,
                    }],
                },
                RowData {
                    row_index: 1,
                    cells: vec![CellData {
                        column_index: 0,
                        content: "1".to_string(),
                        style: CellStyle::default(),
                        merge_info: None,
                    }],
                },
            ],
        };

        let results = identifier.identify_all_rows(&worksheet);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].row_type, RowType::Header);
        assert_eq!(results[1].row_type, RowType::Data);
    }

    #[test]
    fn test_get_statistics() {
        let profile = FileTypeProfile::cargo_analysis();
        let identifier = RowTypeIdentifier::new(profile);

        let results = vec![
            RowIdentificationResult {
                row_index: 0,
                row_type: RowType::Header,
                matched_rule: "test".to_string(),
                confidence: 1.0,
            },
            RowIdentificationResult {
                row_index: 1,
                row_type: RowType::Data,
                matched_rule: "test".to_string(),
                confidence: 1.0,
            },
            RowIdentificationResult {
                row_index: 2,
                row_type: RowType::Data,
                matched_rule: "test".to_string(),
                confidence: 1.0,
            },
            RowIdentificationResult {
                row_index: 3,
                row_type: RowType::Unknown,
                matched_rule: "none".to_string(),
                confidence: 0.0,
            },
        ];

        let stats = identifier.get_statistics(&results);
        assert_eq!(stats.total_rows, 4);
        assert_eq!(stats.unknown_count, 1);
        assert_eq!(stats.success_rate, 0.75);
    }

    #[test]
    fn test_debug_mode() {
        let profile = FileTypeProfile::cargo_analysis();
        let identifier = RowTypeIdentifier::new(profile).with_debug(true);

        let row = RowData {
            row_index: 0,
            cells: vec![CellData {
                column_index: 0,
                content: "Test".to_string(),
                style: CellStyle::default(),
                merge_info: None,
            }],
        };

        // 调试模式应该输出日志（这里只是确保不会崩溃）
        let _result = identifier.identify_row(&row);
    }
}
