// 日志查看器
use crate::logger::{LogEntry, LogLevel, LOGGER};

pub struct LogViewer {
    pub show: bool,
    filter_level: Option<LogLevel>,
    auto_scroll: bool,
    // 自动换行：控制普通模式与复制模式的软换行行为
    auto_wrap: bool,
}

impl Default for LogViewer {
    fn default() -> Self {
        Self {
            show: false,
            filter_level: None,
            auto_scroll: true,
            auto_wrap: true,
        }
    }
}

impl LogViewer {
    pub fn render(&mut self, ctx: &egui::Context) {
        if !self.show {
            return;
        }

        egui::Window::new("📋 日志查看器")
            .default_width(900.0)
            .default_height(560.0)
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
                    ui.checkbox(&mut self.auto_wrap, "自动换行");
                    
                    if ui.button("🗑 清空").clicked() {
                        LOGGER.clear();
                    }
                    if ui.button("📋 复制全部").clicked() {
                        // 拼接当前过滤后的日志文本到剪贴板
                        let entries = LOGGER.get_entries();
                        let filtered: Vec<&LogEntry> = entries
                            .iter()
                            .filter(|entry| {
                                self.filter_level.is_none() || self.filter_level == Some(entry.level)
                            })
                            .collect();
                        let mut all_text = String::new();
                        for e in &filtered {
                            use std::fmt::Write as _;
                            let _ = writeln!(all_text, "{} [{}] {}", e.timestamp, e.level.as_str(), e.message);
                        }
                        ui.output_mut(|o| o.copied_text = all_text);
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

                if is_empty {
                    // 空态：占满可用区域并居中显示提示文本
                    let avail = ui.available_size();
                    ui.allocate_ui_with_layout(
                        avail,
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            ui.label(
                                egui::RichText::new("暂无日志")
                                    .size(14.0)
                                    .color(ui.visuals().weak_text_color()),
                            );
                        },
                    );
                } else {
                    // 彩色高亮列表 + 双向滚动
                    egui::ScrollArea::vertical()
                        .stick_to_bottom(self.auto_scroll)
                        .show(ui, |ui| {
                            if self.auto_wrap {
                                for entry in &filtered_entries {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label(
                                            egui::RichText::new(&entry.timestamp)
                                                .size(11.0)
                                                .color(ui.visuals().weak_text_color())
                                                .monospace(),
                                        );
                                        ui.label(
                                            egui::RichText::new(format!(" [{}] ", entry.level.as_str()))
                                                .size(11.0)
                                                .color(entry.level.color())
                                                .monospace(),
                                        );
                                        let label = egui::Label::new(
                                            egui::RichText::new(&entry.message).size(12.0),
                                        ).wrap();
                                        ui.add(label);
                                    });
                                }
                            } else {
                                // 横向过长时提供水平滚动，不撑大窗口
                                egui::ScrollArea::horizontal()
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
                                                    egui::RichText::new(format!(" [{}] ", entry.level.as_str()))
                                                        .size(11.0)
                                                        .color(entry.level.color())
                                                        .monospace(),
                                                );
                                                ui.label(egui::RichText::new(&entry.message).size(12.0));
                                            });
                                        }
                                    });
                            }
                        });
                }
            });
    }
}
