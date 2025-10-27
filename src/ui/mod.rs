// UI 模块 - 负责所有界面渲染
pub mod home;
pub mod processing;
pub mod settings;
pub mod history;
pub mod log_viewer;

pub use log_viewer::LogViewer;

use crate::app::IntegratedPowerApp;
use crate::models::AppView;

// 渲染顶部面板
pub fn render_top_panel(app: &mut IntegratedPowerApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel")
        .frame(egui::Frame::none().fill(ctx.style().visuals.window_fill()).inner_margin(12.0))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 应用标题
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

                // 导航按钮
                let button_size = egui::vec2(100.0, 32.0);
                
                if ui
                    .add_sized(
                        button_size,
                        egui::SelectableLabel::new(
                            app.current_view == AppView::Home,
                            egui::RichText::new("🏠 主页").size(15.0),
                        ),
                    )
                    .clicked()
                {
                    app.current_view = AppView::Home;
                }

                ui.add_space(5.0);

                if ui
                    .add_sized(
                        button_size,
                        egui::SelectableLabel::new(
                            app.current_view == AppView::Settings,
                            egui::RichText::new("⚙️ 设置").size(15.0),
                        ),
                    )
                    .clicked()
                {
                    app.current_view = AppView::Settings;
                }

                ui.add_space(5.0);

                if ui
                    .add_sized(
                        button_size,
                        egui::SelectableLabel::new(
                            app.current_view == AppView::History,
                            egui::RichText::new("📜 历史").size(15.0),
                        ),
                    )
                    .clicked()
                {
                    app.current_view = AppView::History;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 主题切换按钮
                    let theme_icon = match app.config_manager.get_config().theme {
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
                        app.toggle_theme();
                    }
                    
                    ui.add_space(5.0);
                    
                    // 日志查看器按钮
                    if ui
                        .add_sized(
                            egui::vec2(40.0, 32.0),
                            egui::Button::new(egui::RichText::new("📋").size(18.0)),
                        )
                        .on_hover_text("查看日志")
                        .clicked()
                    {
                        app.log_viewer.show = !app.log_viewer.show;
                    }
                    
                    ui.add_space(8.0);
                });
            });
        });
}

// 渲染底部面板
pub fn render_bottom_panel(app: &IntegratedPowerApp, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("bottom_panel")
        .frame(
            egui::Frame::none()
                .fill(ctx.style().visuals.window_fill())
                .inner_margin(egui::Margin::symmetric(16.0, 8.0)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 状态指示器
                let (status_text, status_color) = match &app.processing_state {
                    crate::models::ProcessingState::Idle => ("● 就绪", egui::Color32::from_rgb(76, 175, 80)),
                    crate::models::ProcessingState::Processing { .. } => {
                        ("● 处理中...", egui::Color32::from_rgb(33, 150, 243))
                    }
                    crate::models::ProcessingState::Completed(_) => {
                        ("● 完成", egui::Color32::from_rgb(76, 175, 80))
                    }
                    crate::models::ProcessingState::Error(_) => {
                        ("● 错误", egui::Color32::from_rgb(244, 67, 54))
                    }
                };

                ui.label(egui::RichText::new(status_text).color(status_color).size(14.0));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 从当前选中的处理器配置中获取输入路径
                    let input_path = app
                        .selected_processor
                        .as_ref()
                        .and_then(|id| app.processor_configs.get(id))
                        .and_then(|config| config.input_path.as_ref());

                    if let Some(input_path) = input_path {
                        // 如果是文件夹，统计文件数量
                        if input_path.is_dir() {
                            if let Ok(entries) = std::fs::read_dir(input_path) {
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
                            // 如果是单个文件
                            ui.label(
                                egui::RichText::new("📄 已选择输入文件")
                                    .size(14.0)
                                    .color(ctx.style().visuals.weak_text_color()),
                            );
                        }
                    } else {
                        ui.label(
                            egui::RichText::new("📄 未选择输入文件")
                                .size(14.0)
                                .color(ctx.style().visuals.weak_text_color()),
                        );
                    }
                });
            });
        });
}

// 渲染中央面板
pub fn render_central_panel(app: &mut IntegratedPowerApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        match app.current_view {
            AppView::Home => home::render(app, ui),
            AppView::Processing => processing::render(app, ui),
            AppView::Settings => settings::render(app, ui),
            AppView::History => history::render(app, ui),
        }
    });
}
