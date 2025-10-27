use crate::config::manager::{ConfigManager, Theme};
use crate::history::HistoryManager;
use crate::models::{AppView, ProcessingProgress, ProcessingState};
use crate::processor::ProcessorManager;
use std::path::PathBuf;
use std::sync::mpsc;

pub struct IntegratedPowerApp {
    // 管理器
    config_manager: ConfigManager,
    processor_manager: ProcessorManager,
    history_manager: HistoryManager,

    // UI 状态
    current_view: AppView,
    selected_processor: Option<String>,
    input_dir: Option<PathBuf>,
    output_dir: Option<PathBuf>,

    // 处理状态
    processing_state: ProcessingState,
    progress: ProcessingProgress,

    // UI 组件状态
    search_query: String,
    error_message: Option<String>,

    // 进度接收通道
    progress_rx: Option<mpsc::Receiver<ProcessingProgress>>,
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
        // 使用系统字体或内嵌字体
        #[cfg(target_os = "windows")]
        {
            // Windows 系统字体路径
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

        #[cfg(target_os = "macos")]
        {
            // macOS 系统字体路径
            if let Ok(font_data) = std::fs::read("/System/Library/Fonts/PingFang.ttc") {
                fonts.font_data.insert(
                    "pingfang".to_owned(),
                    egui::FontData::from_owned(font_data),
                );
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "pingfang".to_owned());
            }
        }

        #[cfg(target_os = "linux")]
        {
            // Linux 系统字体路径
            if let Ok(font_data) = std::fs::read("/usr/share/fonts/truetype/wqy/wqy-microhei.ttc") {
                fonts.font_data.insert(
                    "wqy_microhei".to_owned(),
                    egui::FontData::from_owned(font_data),
                );
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "wqy_microhei".to_owned());
            }
        }

        ctx.set_fonts(fonts);
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        match self.config_manager.get_config().theme {
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            Theme::System => {
                // 使用系统主题
                match dark_light::detect() {
                    dark_light::Mode::Dark => ctx.set_visuals(egui::Visuals::dark()),
                    dark_light::Mode::Light | dark_light::Mode::Default => {
                        ctx.set_visuals(egui::Visuals::light())
                    }
                }
            }
        }
    }

    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::none().fill(ctx.style().visuals.window_fill()).inner_margin(12.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // 应用标题 - 更大更醒目
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("⚡ IntegratedPower")
                            .size(24.0)
                            .strong()
                            .color(ctx.style().visuals.strong_text_color()),
                    );

                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // 导航按钮 - 更大的可点击区域
                    let button_size = egui::vec2(100.0, 32.0);
                    
                    if ui
                        .add_sized(
                            button_size,
                            egui::SelectableLabel::new(
                                self.current_view == AppView::Home,
                                egui::RichText::new("🏠 主页").size(15.0),
                            ),
                        )
                        .clicked()
                    {
                        self.current_view = AppView::Home;
                    }

                    ui.add_space(5.0);

                    if ui
                        .add_sized(
                            button_size,
                            egui::SelectableLabel::new(
                                self.current_view == AppView::Settings,
                                egui::RichText::new("⚙️ 设置").size(15.0),
                            ),
                        )
                        .clicked()
                    {
                        self.current_view = AppView::Settings;
                    }

                    ui.add_space(5.0);

                    if ui
                        .add_sized(
                            button_size,
                            egui::SelectableLabel::new(
                                self.current_view == AppView::History,
                                egui::RichText::new("📜 历史").size(15.0),
                            ),
                        )
                        .clicked()
                    {
                        self.current_view = AppView::History;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // 主题切换按钮 - 更大更明显
                        let theme_icon = match self.config_manager.get_config().theme {
                            crate::config::Theme::Light => "☀️",
                            crate::config::Theme::Dark => "🌙",
                            crate::config::Theme::System => "💻",
                        };
                        
                        if ui
                            .add_sized(
                                egui::vec2(40.0, 32.0),
                                egui::Button::new(egui::RichText::new(theme_icon).size(18.0)),
                            )
                            .on_hover_text("切换主题")
                            .clicked()
                        {
                            self.toggle_theme();
                        }
                        ui.add_space(8.0);
                    });
                });
            });
    }

    fn render_bottom_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .frame(
                egui::Frame::none()
                    .fill(ctx.style().visuals.window_fill())
                    .inner_margin(egui::Margin::symmetric(16.0, 8.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // 状态指示器
                    let (status_text, status_color) = match &self.processing_state {
                        ProcessingState::Idle => ("● 就绪", egui::Color32::from_rgb(76, 175, 80)),
                        ProcessingState::Processing { .. } => {
                            ("● 处理中...", egui::Color32::from_rgb(33, 150, 243))
                        }
                        ProcessingState::Completed(_) => {
                            ("● 完成", egui::Color32::from_rgb(76, 175, 80))
                        }
                        ProcessingState::Error(_) => {
                            ("● 错误", egui::Color32::from_rgb(244, 67, 54))
                        }
                    };

                    ui.label(egui::RichText::new(status_text).color(status_color).size(14.0));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if let Some(input_dir) = &self.input_dir {
                            if let Ok(entries) = std::fs::read_dir(input_dir) {
                                let count = entries
                                    .filter_map(|e| e.ok())
                                    .filter(|e| {
                                        e.path()
                                            .extension()
                                            .and_then(|s| s.to_str())
                                            .map(|s| s == "xlsx")
                                            .unwrap_or(false)
                                    })
                                    .count();
                                
                                ui.label(
                                    egui::RichText::new(format!("📊 {} 个文件待处理", count))
                                        .size(14.0)
                                        .color(ctx.style().visuals.weak_text_color()),
                                );
                            }
                        } else {
                            ui.label(
                                egui::RichText::new("📁 未选择输入目录")
                                    .size(14.0)
                                    .color(ctx.style().visuals.weak_text_color()),
                            );
                        }
                    });
                });
            });
    }

    fn render_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                AppView::Home => self.render_home_view(ui),
                AppView::Processing => self.render_processing_view(ui),
                AppView::Settings => self.render_settings_view(ui),
                AppView::History => self.render_history_view(ui),
            }
        });
    }

    fn render_home_view(&mut self, ui: &mut egui::Ui) {
        ui.heading("选择处理功能");
        ui.add_space(10.0);

        // 搜索栏
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.add(
                egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("搜索处理功能...")
                    .desired_width(300.0),
            );
        });

        ui.add_space(10.0);

        // 功能卡片网格
        egui::ScrollArea::vertical().show(ui, |ui| {
            let processors = self.get_filtered_processors();
            
            // 使用网格布局显示卡片
            let available_width = ui.available_width();
            let card_width = 250.0;
            let spacing = 10.0;
            let columns = ((available_width + spacing) / (card_width + spacing)).floor().max(1.0) as usize;

            egui::Grid::new("processor_grid")
                .num_columns(columns)
                .spacing([spacing, spacing])
                .show(ui, |ui| {
                    for (idx, processor) in processors.iter().enumerate() {
                        self.render_processor_card(ui, processor);
                        
                        if (idx + 1) % columns == 0 {
                            ui.end_row();
                        }
                    }
                });
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        // 目录选择
        self.render_directory_selector(ui);

        ui.add_space(20.0);

        // 开始按钮
        ui.horizontal(|ui| {
            let can_start = self.selected_processor.is_some() 
                && self.input_dir.is_some() 
                && self.output_dir.is_some();
            
            ui.add_enabled_ui(can_start, |ui| {
                if ui
                    .add_sized([200.0, 40.0], egui::Button::new("开始处理"))
                    .clicked()
                {
                    self.error_message = Some("处理逻辑将在任务 12 中实现".to_string());
                }
            });

            if !can_start {
                ui.label("请选择处理功能和输入输出目录");
            }
        });
    }

    fn get_filtered_processors(&self) -> Vec<crate::processor::trait_def::ProcessorInfo> {
        let processors = self.processor_manager.list_processors();
        
        if self.search_query.is_empty() {
            processors
        } else {
            let query = self.search_query.to_lowercase();
            processors
                .into_iter()
                .filter(|p| {
                    p.name.to_lowercase().contains(&query)
                        || p.description.to_lowercase().contains(&query)
                })
                .collect()
        }
    }

    fn render_processor_card(&mut self, ui: &mut egui::Ui, processor: &crate::processor::trait_def::ProcessorInfo) {
        let is_selected = self.selected_processor.as_ref() == Some(&processor.id);
        
        let card_frame = egui::Frame::none()
            .fill(if is_selected {
                ui.visuals().selection.bg_fill
            } else {
                ui.visuals().window_fill()
            })
            .rounding(8.0)
            .stroke(egui::Stroke::new(
                if is_selected { 2.0 } else { 1.0 },
                if is_selected {
                    ui.visuals().selection.stroke.color
                } else {
                    ui.visuals().widgets.noninteractive.bg_stroke.color
                },
            ))
            .inner_margin(12.0);

        card_frame.show(ui, |ui| {
            ui.set_width(250.0);
            ui.set_height(120.0);
            
            ui.vertical(|ui| {
                // 图标和名称
                ui.horizontal(|ui| {
                    if let Some(icon) = &processor.icon {
                        ui.label(egui::RichText::new(icon).size(24.0));
                    }
                    ui.heading(&processor.name);
                });

                ui.add_space(5.0);

                // 描述
                ui.label(&processor.description);

                ui.add_space(5.0);

                // 选择按钮
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    if ui.button(if is_selected { "已选择 ✓" } else { "选择" }).clicked() {
                        if is_selected {
                            self.selected_processor = None;
                        } else {
                            self.selected_processor = Some(processor.id.clone());
                        }
                    }
                });
            });
        });
    }

    fn render_directory_selector(&mut self, ui: &mut egui::Ui) {
        // 输入目录
        ui.horizontal(|ui| {
            ui.label("📁 输入目录:");
            if let Some(path) = &self.input_dir {
                ui.label(path.display().to_string());
            } else {
                ui.label("未选择");
            }
            
            if ui.button("选择...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.input_dir = Some(path);
                }
            }
            
            if self.input_dir.is_some() && ui.button("清除").clicked() {
                self.input_dir = None;
            }
        });

        ui.add_space(5.0);

        // 输出目录
        ui.horizontal(|ui| {
            ui.label("📁 输出目录:");
            if let Some(path) = &self.output_dir {
                ui.label(path.display().to_string());
            } else {
                ui.label("未选择");
            }
            
            if ui.button("选择...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.output_dir = Some(path);
                }
            }
            
            if self.output_dir.is_some() && ui.button("清除").clicked() {
                self.output_dir = None;
            }
        });
    }

    fn render_processing_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("正在处理...");
            ui.add_space(20.0);

            ui.label(format!("当前文件: {}", self.progress.current_file));
            ui.add_space(10.0);

            let progress = self.progress.percentage / 100.0;
            ui.add(
                egui::ProgressBar::new(progress)
                    .text(format!("{:.0}%", self.progress.percentage))
                    .animate(true),
            );

            ui.add_space(10.0);

            ui.label(format!(
                "已处理: {}/{} 文件",
                self.progress.processed_files, self.progress.total_files
            ));

            ui.add_space(20.0);

            if ui.button("取消处理").clicked() {
                // 取消逻辑将在任务 12 中实现
            }
        });
    }

    fn render_settings_view(&mut self, ui: &mut egui::Ui) {
        ui.heading("设置");
        ui.add_space(10.0);
        ui.label("设置界面将在任务 13 中实现");
    }

    fn render_history_view(&mut self, ui: &mut egui::Ui) {
        ui.heading("历史记录");
        ui.add_space(10.0);
        ui.label("历史记录界面将在任务 14 中实现");
    }

    fn toggle_theme(&mut self) {
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
        // 检查进度更新
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

        // 渲染 UI
        self.render_top_panel(ctx);
        self.render_bottom_panel(ctx);
        self.render_central_panel(ctx);

        // 显示错误对话框
        self.show_error(ctx);

        // 处理后台任务
        self.poll_processing_tasks(ctx);
    }
}
