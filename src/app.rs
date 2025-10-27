use crate::config::ConfigManager;
use crate::history::HistoryManager;
use crate::models::{AppView, ProcessingProgress, ProcessingState};
use crate::processor::ProcessorManager;
use std::path::PathBuf;
use std::sync::mpsc;

pub struct IntegratedPowerApp {
    // 管理器
    pub config_manager: ConfigManager,
    pub processor_manager: ProcessorManager,
    pub history_manager: HistoryManager,

    // UI 状态
    pub current_view: AppView,
    pub selected_processor: Option<String>,
    pub input_dir: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,

    // 处理状态
    pub processing_state: ProcessingState,
    pub progress: ProcessingProgress,

    // UI 组件状态
    pub search_query: String,
    pub error_message: Option<String>,

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

        // 从配置中恢复目录
        let input_dir = config_manager.get_config().default_input_dir.clone();
        let output_dir = config_manager.get_config().default_output_dir.clone();

        Self {
            config_manager,
            processor_manager,
            history_manager,
            current_view: AppView::Home,
            selected_processor: None,
            input_dir,
            output_dir,
            processing_state: ProcessingState::Idle,
            progress: ProcessingProgress::default(),
            search_query: String::new(),
            error_message: None,
            progress_rx: None,
        }
    }

    fn setup_custom_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        // 添加中文字体支持
        #[cfg(target_os = "windows")]
        {
            if let Ok(font_data) = std::fs::read("C:\\Windows\\Fonts\\msyh.ttc") {
                fonts.font_data.insert(
                    "microsoft_yahei".to_owned(),
                    egui::FontData::from_owned(font_data),
                );
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "microsoft_yahei".to_owned());
            }
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

        // 处理后台任务
        self.poll_processing_tasks(ctx);
    }
}
