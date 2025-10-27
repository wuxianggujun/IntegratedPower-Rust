use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 默认输入目录
    pub default_input_dir: Option<PathBuf>,
    /// 默认输出目录
    pub default_output_dir: Option<PathBuf>,
    /// 界面主题
    pub theme: Theme,
    /// 最大历史记录条目数
    pub max_history_entries: usize,
    /// 是否启用并行处理
    pub parallel_processing: bool,
    /// 最大并行任务数
    pub max_parallel_tasks: usize,
}

/// 主题类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    /// 浅色主题
    Light,
    /// 深色主题
    Dark,
    /// 系统主题
    System,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_input_dir: None,
            default_output_dir: None,
            theme: Theme::System,
            max_history_entries: 100,
            parallel_processing: true,
            max_parallel_tasks: num_cpus::get().max(2).min(8),
        }
    }
}

impl AppConfig {
    /// 验证配置
    pub fn validate(&self) -> Result<()> {
        // 验证历史记录条目数
        if self.max_history_entries == 0 {
            return Err(AppError::config_error("最大历史记录条目数必须大于 0"));
        }

        // 验证并行任务数
        if self.parallel_processing && self.max_parallel_tasks == 0 {
            return Err(AppError::config_error("最大并行任务数必须大于 0"));
        }

        // 验证默认目录（如果设置）
        if let Some(ref dir) = self.default_input_dir {
            if !dir.exists() {
                tracing::warn!("默认输入目录不存在: {}", dir.display());
            }
        }

        if let Some(ref dir) = self.default_output_dir {
            if !dir.exists() {
                tracing::warn!("默认输出目录不存在: {}", dir.display());
            }
        }

        Ok(())
    }
}

/// 配置管理器
pub struct ConfigManager {
    config: AppConfig,
    config_path: PathBuf,
}

impl Default for ConfigManager {
    fn default() -> Self {
        let config_path = Self::get_config_path().unwrap_or_else(|_| PathBuf::from("config.toml"));
        Self {
            config: AppConfig::default(),
            config_path,
        }
    }
}

impl ConfigManager {
    /// 加载配置
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        let config = if config_path.exists() {
            tracing::info!("从文件加载配置: {}", config_path.display());
            let content = fs::read_to_string(&config_path)?;
            let config: AppConfig = toml::from_str(&content)?;
            config.validate()?;
            config
        } else {
            tracing::info!("配置文件不存在，使用默认配置");
            let config = AppConfig::default();
            
            // 创建默认配置文件
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            let content = toml::to_string_pretty(&config)
                .map_err(|e| AppError::config_error(format!("序列化配置失败: {}", e)))?;
            fs::write(&config_path, content)?;
            
            config
        };

        Ok(Self {
            config,
            config_path,
        })
    }

    /// 保存配置
    pub fn save(&self) -> Result<()> {
        tracing::info!("保存配置到: {}", self.config_path.display());
        
        self.config.validate()?;
        
        let content = toml::to_string_pretty(&self.config)
            .map_err(|e| AppError::config_error(format!("序列化配置失败: {}", e)))?;
        
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(&self.config_path, content)?;
        
        Ok(())
    }

    /// 更新配置
    pub fn update_config(&mut self, config: AppConfig) -> Result<()> {
        config.validate()?;
        self.config = config;
        self.save()?;
        tracing::info!("配置已更新");
        Ok(())
    }

    /// 重置为默认配置
    pub fn reset_to_default(&mut self) -> Result<()> {
        tracing::info!("重置配置为默认值");
        self.config = AppConfig::default();
        self.save()?;
        Ok(())
    }

    /// 获取配置
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// 获取配置（别名方法）
    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    /// 获取可变配置
    pub fn config_mut(&mut self) -> &mut AppConfig {
        &mut self.config
    }

    /// 设置默认输入目录
    pub fn set_default_input_dir(&mut self, dir: Option<PathBuf>) -> Result<()> {
        self.config.default_input_dir = dir;
        self.save()
    }

    /// 设置默认输出目录
    pub fn set_default_output_dir(&mut self, dir: Option<PathBuf>) -> Result<()> {
        self.config.default_output_dir = dir;
        self.save()
    }

    /// 设置主题
    pub fn set_theme(&mut self, theme: Theme) -> Result<()> {
        self.config.theme = theme;
        self.save()
    }

    /// 设置最大历史记录条目数
    pub fn set_max_history_entries(&mut self, count: usize) -> Result<()> {
        if count == 0 {
            return Err(AppError::config_error("最大历史记录条目数必须大于 0"));
        }
        self.config.max_history_entries = count;
        self.save()
    }

    /// 设置并行处理选项
    pub fn set_parallel_processing(&mut self, enabled: bool, max_tasks: usize) -> Result<()> {
        if enabled && max_tasks == 0 {
            return Err(AppError::config_error("最大并行任务数必须大于 0"));
        }
        self.config.parallel_processing = enabled;
        self.config.max_parallel_tasks = max_tasks;
        self.save()
    }

    /// 获取配置文件路径
    fn get_config_path() -> Result<PathBuf> {
        // 使用用户配置目录
        let config_dir = if cfg!(target_os = "windows") {
            // Windows: %APPDATA%\IntegratedPower
            dirs::config_dir()
                .ok_or_else(|| AppError::config_error("无法获取配置目录"))?
                .join("IntegratedPower")
        } else if cfg!(target_os = "macos") {
            // macOS: ~/Library/Application Support/IntegratedPower
            dirs::config_dir()
                .ok_or_else(|| AppError::config_error("无法获取配置目录"))?
                .join("IntegratedPower")
        } else {
            // Linux: ~/.config/IntegratedPower
            dirs::config_dir()
                .ok_or_else(|| AppError::config_error("无法获取配置目录"))?
                .join("IntegratedPower")
        };

        Ok(config_dir.join("config.toml"))
    }

    /// 获取配置文件路径（公开方法）
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.theme, Theme::Light);
        assert_eq!(config.max_history_entries, 100);
        assert!(config.parallel_processing);
        assert!(config.max_parallel_tasks > 0);
    }

    #[test]
    fn test_config_validation() {
        let mut config = AppConfig::default();
        assert!(config.validate().is_ok());

        config.max_history_entries = 0;
        assert!(config.validate().is_err());

        config.max_history_entries = 100;
        config.max_parallel_tasks = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_theme_serialization() {
        let config = AppConfig {
            theme: Theme::Dark,
            ..Default::default()
        };

        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("Dark"));

        let deserialized: AppConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized.theme, Theme::Dark);
    }
}
