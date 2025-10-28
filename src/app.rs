use crate::config::ConfigManager;
use crate::history::HistoryManager;
use crate::models::{AppView, ProcessingProgress, ProcessingState, ProcessorConfigs};
use crate::processor::ProcessorManager;
use std::sync::mpsc;

pub struct IntegratedPowerApp {
    // 管理器
    pub config_manager: ConfigManager,
    pub processor_manager: ProcessorManager,
    pub history_manager: HistoryManager,

    // UI 状态
    pub current_view: AppView,
    pub selected_processor: Option<String>,

    // 处理状态
    pub processing_state: ProcessingState,
    pub progress: ProcessingProgress,

    // UI 组件状态
    pub search_query: String,
    pub error_message: Option<String>,

    // 处理器配置
    pub processor_configs: ProcessorConfigs,

    // 日志查看器
    pub log_viewer: crate::ui::LogViewer,

    // 进度接收通道
    pub progress_rx: Option<mpsc::Receiver<ProcessingProgress>>,
}

impl IntegratedPowerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 配置中文字体支持
        Self::setup_custom_fonts(&cc.egui_ctx);

        // 加载配置
        let config_manager = ConfigManager::load().unwrap_or_else(|e| {
            eprintln!("加载配置失败: {}, 使用默认配置", e);
            ConfigManager::default()
        });

        // 加载历史记录
        let max_entries = config_manager.get_config().max_history_entries;
        let history_manager = HistoryManager::load(max_entries).unwrap_or_else(|e| {
            eprintln!("加载历史记录失败: {}, 使用空历史", e);
            HistoryManager::new(max_entries)
        });

        // 创建处理器管理器
        let processor_manager = ProcessorManager::new();

        // 加载处理器配置
        let processor_configs = Self::load_processor_configs().unwrap_or_default();

        // 记录应用启动
        crate::log_info!("IntegratedPower 应用启动");

        Self {
            config_manager,
            processor_manager,
            history_manager,
            current_view: AppView::Home,
            selected_processor: None,
            processing_state: ProcessingState::Idle,
            progress: ProcessingProgress::default(),
            search_query: String::new(),
            error_message: None,
            processor_configs,
            log_viewer: crate::ui::LogViewer::default(),
            progress_rx: None,
        }
    }

    fn setup_custom_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        // 1) 优先尝试加载项目资源字体（思源黑体）
        let mut loaded_key: Option<String> = None;
        let resource_candidates: Vec<std::path::PathBuf> = {
            let mut v = Vec::new();
            // 运行目录相对路径
            v.push(std::path::PathBuf::from("resources/fonts/SourceHanSansSC-Regular.otf"));
            // 可执行文件所在目录相对路径
            if let Ok(exe) = std::env::current_exe() {
                if let Some(dir) = exe.parent() {
                    v.push(dir.join("resources/fonts/SourceHanSansSC-Regular.otf"));
                }
            }
            v
        };

        for p in resource_candidates {
            if let Ok(font_data) = std::fs::read(&p) {
                let key = "source_han_sans_sc".to_owned();
                fonts.font_data.insert(key.clone(), egui::FontData::from_owned(font_data));
                loaded_key = Some(key);
                break;
            }
        }

        // 2) 若资源字体不可用，在 Windows 上回退到常见系统中文字体
        #[cfg(target_os = "windows")]
        if loaded_key.is_none() {
            let candidates = [
                "C:\\Windows\\Fonts\\simhei.ttf",
                "C:\\Windows\\Fonts\\msyh.ttf",
                "C:\\Windows\\Fonts\\msyh.ttc",
            ];
            for path in candidates.iter() {
                if let Ok(font_data) = std::fs::read(path) {
                    let key = "win_cn_font".to_owned();
                    fonts.font_data.insert(key.clone(), egui::FontData::from_owned(font_data));
                    loaded_key = Some(key);
                    break;
                }
            }
        }

        // 3) 若已加载到中文字体，则注入到比例与等宽字体族，保证 .monospace 等场景不缺字
        if let Some(key) = loaded_key {
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, key.clone());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, key);
        }

        ctx.set_fonts(fonts);
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        use crate::config::Theme;

        match self.config_manager.get_config().theme {
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            Theme::System => {
                match dark_light::detect() {
                    dark_light::Mode::Dark => ctx.set_visuals(egui::Visuals::dark()),
                    dark_light::Mode::Light | dark_light::Mode::Default => {
                        ctx.set_visuals(egui::Visuals::light())
                    }
                }
            }
        }
    }

    pub fn toggle_theme(&mut self) {
        use crate::config::Theme;

        let current_theme = self.config_manager.get_config().theme;
        let new_theme = match current_theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::System,
            Theme::System => Theme::Light,
        };

        let mut config = self.config_manager.get_config().clone();
        config.theme = new_theme;

        if let Err(e) = self.config_manager.update_config(config) {
            self.error_message = Some(format!("保存主题设置失败: {}", e));
        }
    }

    // 保存处理器配置
    pub fn save_processor_configs(&self) -> anyhow::Result<()> {
        let config_path = Self::get_processor_configs_path()?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let json = serde_json::to_string_pretty(&self.processor_configs)?;
        std::fs::write(&config_path, json)?;
        
        Ok(())
    }

    // 加载处理器配置
    fn load_processor_configs() -> anyhow::Result<ProcessorConfigs> {
        let config_path = Self::get_processor_configs_path()?;
        
        if !config_path.exists() {
            return Ok(ProcessorConfigs::default());
        }
        
        let content = std::fs::read_to_string(&config_path)?;
        let configs: ProcessorConfigs = serde_json::from_str(&content)?;
        
        Ok(configs)
    }

    // 获取处理器配置文件路径
    fn get_processor_configs_path() -> anyhow::Result<std::path::PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("无法获取配置目录"))?;
        
        let app_config_dir = config_dir.join("IntegratedPower");
        Ok(app_config_dir.join("processor_configs.json"))
    }

    fn poll_processing_tasks(&mut self, ctx: &egui::Context) {
        if let Some(rx) = &self.progress_rx {
            if let Ok(progress) = rx.try_recv() {
                self.progress = progress;
                ctx.request_repaint();
            }
        }
    }

    fn show_error(&mut self, ctx: &egui::Context) {
        let mut should_close = false;
        
        if let Some(error) = &self.error_message {
            let error_text = error.clone();
            egui::Window::new("错误")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(&error_text);
                    if ui.button("确定").clicked() {
                        should_close = true;
                    }
                });
        }
        
        if should_close {
            self.error_message = None;
        }
    }
}

impl eframe::App for IntegratedPowerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 应用主题
        self.apply_theme(ctx);

        // 渲染 UI - 委托给 ui 模块
        crate::ui::render_top_panel(self, ctx);
        crate::ui::render_bottom_panel(self, ctx);
        crate::ui::render_central_panel(self, ctx);

        // 显示错误对话框
        self.show_error(ctx);

        // 显示日志查看器
        self.log_viewer.render(ctx);

        // 处理后台任务
        self.poll_processing_tasks(ctx);
    }
}
