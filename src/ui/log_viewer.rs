// 日志查看器
use crate::logger::{LogEntry, LogLevel, LOGGER};

pub struct LogViewer {
    pub show: bool,
    filter_level: Option<LogLevel>,
    auto_scroll: bool,
}

impl Default for LogViewer {
    fn default() -> Self {
        Self {
            show: false,
            filter_level: None,
            auto_scroll: true,
        }
    }
}

impl LogViewer {
    pub fn render(&mut self, ctx: &egui::Context) {
        if !self.show {
            return;
        }

        egui::Window::new("📋 日志查看器")
            .default_width(800.0)
            .default_height(500.0)
            .show(ctx, |ui| {
                // 工具栏
                ui.horizontal(|ui| {
                    ui.label("过滤级别:");
                    
                    if ui.selectable_label(self.filter_level.is_none(), "全部").clicked() {
                        self.filter_level = None;
                    }
                    
                    if ui.selectable_label(self.filter_level == Some(LogLevel::Debug), "DEBUG").clicked() {
                        self.filter_level = Some(LogLevel::Debug);
                    }
                    
                    if ui.selectable_label(self.filter_level == Some(LogLevel::Info), "INFO").clicked() {
                        self.filter_level = Some(LogLevel::Info);
                    }
                    
                    if ui.selectable_label(self.filter_level == Some(LogLevel::Warning), "WARN").clicked() {
                        self.filter_level = Some(LogLevel::Warning);
                    }
                    
                    if ui.selectable_label(self.filter_level == Some(LogLevel::Error), "ERROR").clicked() {
                        self.filter_level = Some(LogLevel::Error);
                    }
                    
                    ui.separator();
                    
                    ui.checkbox(&mut self.auto_scroll, "自动滚动");
                    
                    if ui.button("🗑 清空").clicked() {
                        LOGGER.clear();
                    }
                    
                    if ui.button("📁 打开日志文件").clicked() {
                        if let Some(path) = LOGGER.get_log_file_path_str() {
                            #[cfg(target_os = "windows")]
                            {
                                let _ = std::process::Command::new("explorer")
                                    .arg("/select,")
                                    .arg(&path)
                                    .spawn();
                            }
                            
                            #[cfg(target_os = "macos")]
                            {
                                let _ = std::process::Command::new("open")
                                    .arg("-R")
                                    .arg(&path)
                                    .spawn();
                            }
                            
                            #[cfg(target_os = "linux")]
                            {
                                let _ = std::process::Command::new("xdg-open")
                                    .arg(std::path::Path::new(&path).parent().unwrap())
                                    .spawn();
                            }
                        }
                    }
                });
                
                ui.separator();
                
                // 日志列表
                let entries = LOGGER.get_entries();
                let filtered_entries: Vec<&LogEntry> = entries
                    .iter()
                    .filter(|entry| {
                        self.filter_level.is_none() || self.filter_level == Some(entry.level)
                    })
                    .collect();
                
                let is_empty = filtered_entries.is_empty();
                
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .stick_to_bottom(self.auto_scroll)
                    .show(ui, |ui| {
                        for entry in &filtered_entries {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(&entry.timestamp)
                                        .size(11.0)
                                        .color(ui.visuals().weak_text_color())
                                        .monospace(),
                                );
                                
                                ui.label(
                                    egui::RichText::new(format!("[{}]", entry.level.as_str()))
                                        .size(11.0)
                                        .color(entry.level.color())
                                        .monospace(),
                                );
                                
                                ui.label(
                                    egui::RichText::new(&entry.message)
                                        .size(12.0)
                                        .monospace(),
                                );
                            });
                        }
                        
                        if is_empty {
                            ui.vertical_centered(|ui| {
                                ui.add_space(50.0);
                                ui.label(
                                    egui::RichText::new("暂无日志")
                                        .size(14.0)
                                        .color(ui.visuals().weak_text_color()),
                                );
                            });
                        }
                    });
            });
    }
}
