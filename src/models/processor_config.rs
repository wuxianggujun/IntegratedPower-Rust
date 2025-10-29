// 处理器配置模型
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
// 使用 umya-spreadsheet 读取 sheet 列表

/// 处理器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorConfig {
    /// 输入路径（文件或文件夹）
    pub input_path: Option<PathBuf>,
    /// 输入类型
    pub input_type: InputType,
    /// 输出目录
    pub output_dir: Option<PathBuf>,
    /// 输出文件名
    pub output_filename: String,
    /// 选中的 sheet 名称（None 表示处理所有 sheet）
    pub selected_sheet: Option<String>,
    /// 可用的 sheet 列表（从文件中读取）
    pub available_sheets: Vec<String>,
    /// 功能特定选项
    pub options: HashMap<String, ConfigValue>,
}

/// 输入类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputType {
    File,
    Folder,
}

/// 配置值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigValue {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            input_path: None,
            input_type: InputType::Folder,
            output_dir: None,
            output_filename: "output.xlsx".to_string(),
            selected_sheet: None,
            available_sheets: Vec::new(),
            options: HashMap::new(),
        }
    }
}

impl ProcessorConfig {
    pub fn new(processor_id: &str) -> Self {
        let mut config = Self::default();
        
        // 根据处理器类型设置默认配置
        match processor_id {
            "cargo_analysis" => {
                config.output_filename = "货物分析表.xlsx".to_string();
                config.input_type = InputType::File;
                // 设置默认 sheet 名称
                config.selected_sheet = Some("货物数据".to_string());
                // 设置默认选项
                config.set_bool("include_statistics".to_string(), true);
                config.set_bool("generate_charts".to_string(), true);
                config.set_bool("export_logs".to_string(), false);
            }
            "auxiliary_material" => {
                config.output_filename = "辅材处理结果.xlsx".to_string();
                config.input_type = InputType::File;
                // 设置默认 sheet 名称
                config.selected_sheet = Some("辅材清单".to_string());
                // 设置默认选项
                config.set_bool("auto_classify".to_string(), true);
                config.set_bool("remove_duplicates".to_string(), true);
                config.set_bool("generate_summary".to_string(), false);
            }
            "excel_structure_analyzer" => {
                config.output_filename = "分析结果.txt".to_string();
                config.input_type = InputType::File;
                // Excel分析器不需要输出目录，结果直接输出到日志
                config.output_dir = None;
                // 设置默认选项
                config.set_bool("analyze_structure".to_string(), true);
                config.set_bool("analyze_colors".to_string(), false);
                config.set_bool("detailed_output".to_string(), true);
            }
            _ => {
                config.output_filename = "output.xlsx".to_string();
            }
        }
        
        config
    }
    
    /// 从文件加载可用的 sheet 列表
    pub fn load_sheets_from_file(&mut self) -> Result<(), String> {
        if let Some(path) = &self.input_path {
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("xlsx") {
                // 使用 umya-spreadsheet 读取 sheet 列表
                match umya_spreadsheet::reader::xlsx::read(path) {
                    Ok(book) => {
                        // 获取工作表名称列表
                        let names: Vec<String> = book
                            .get_sheet_collection()
                            .iter()
                            .map(|ws| ws.get_name().to_string())
                            .collect();
                        self.available_sheets = names;
                        
                        // 如果当前没有选中的 sheet，选择第一个
                        if self.selected_sheet.is_none() && !self.available_sheets.is_empty() {
                            self.selected_sheet = Some(self.available_sheets[0].clone());
                        }
                        
                        Ok(())
                    }
                    Err(e) => Err(format!("无法读取 Excel 文件: {}", e)),
                }
            } else {
                Err("不是有效的 Excel 文件".to_string())
            }
        } else {
            Err("未选择输入文件".to_string())
        }
    }
    
    pub fn get_bool(&self, key: &str) -> bool {
        match self.options.get(key) {
            Some(ConfigValue::Bool(v)) => *v,
            _ => false,
        }
    }
    
    pub fn set_bool(&mut self, key: String, value: bool) {
        self.options.insert(key, ConfigValue::Bool(value));
    }
    
    pub fn get_string(&self, key: &str) -> String {
        match self.options.get(key) {
            Some(ConfigValue::String(v)) => v.clone(),
            _ => String::new(),
        }
    }
    
    pub fn set_string(&mut self, key: String, value: String) {
        self.options.insert(key, ConfigValue::String(value));
    }
}

/// 所有处理器的配置集合
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorConfigs {
    pub configs: HashMap<String, ProcessorConfig>,
}

impl ProcessorConfigs {
    pub fn get_or_create(&mut self, processor_id: &str) -> &mut ProcessorConfig {
        self.configs
            .entry(processor_id.to_string())
            .or_insert_with(|| ProcessorConfig::new(processor_id))
    }
    
    pub fn get(&self, processor_id: &str) -> Option<&ProcessorConfig> {
        self.configs.get(processor_id)
    }
}
