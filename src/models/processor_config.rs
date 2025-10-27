// 处理器配置模型
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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
            options: HashMap::new(),
        }
    }
}

impl ProcessorConfig {
    pub fn new(processor_id: &str) -> Self {
        let mut config = Self::default();
        
        // 根据处理器类型设置默认文件名
        config.output_filename = match processor_id {
            "export_cargo_analysis" => "货物分析表.xlsx".to_string(),
            "auxiliary_material" => "辅材处理结果.xlsx".to_string(),
            _ => "output.xlsx".to_string(),
        };
        
        config
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
