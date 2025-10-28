// 日志查看器
use crate::logger::{LogEntry, LogLevel, LOGGER};

pub struct LogViewer {
    pub show: bool,
    filter_level: Option<LogLevel>,
    auto_scroll: bool,
    // 自动换行：控制普通模式与复制模式的软换行行为
    auto_wrap: bool,
    // 复制模式：使用 TextEdit 支持任意跨行选择 + Ctrl+C
    text_mode: bool,
}

impl Default for LogViewer {
    fn default() -> Self {
        Self {
            show: false,
            filter_level: None,
            auto_scroll: true,
            auto_wrap: true,
            text_mode: false,
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
                    ui.checkbox(&mut self.text_mode, "复制模式");
                    
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
                } else if self.text_mode {
                    // 复制模式：单一多行文本，支持跨行选择 + Ctrl+C
                    let mut log_text = String::new();
                    for e in &filtered_entries {
                        use std::fmt::Write as _;
                        let _ = writeln!(
                            log_text,
                            "{} [{}] {}",
                            e.timestamp,
                            e.level.as_str(),
                            e.message
                        );
                    }

                    let mut text = log_text;
                    egui::ScrollArea::both()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let mut te = egui::TextEdit::multiline(&mut text)
                                .desired_rows(24)
                                .font(egui::TextStyle::Monospace);
                            if self.auto_wrap {
                                // 自动换行：宽度跟随窗口，不产生水平拉伸
                                te = te.desired_width(ui.available_width());
                            } else {
                                // 不换行：给内容自然宽度 + code_editor，外层 ScrollArea 提供水平滚动
                                te = te.desired_width(f32::INFINITY).code_editor();
                            }
                            ui.add(te);
                        });
                    ui.label(
                        egui::RichText::new("提示: 选中文本后按 Ctrl+C 可复制；或点击上方‘复制全部’")
                            .size(11.0)
                            .color(ui.visuals().weak_text_color()),
                    );
                } else {
                    // 彩色高亮列表 + 双向滚动（可读性强），不使用右键菜单以免打断选区
                    egui::ScrollArea::vertical()
                        .stick_to_bottom(self.auto_scroll)
                        .show(ui, |ui| {
                            let render_line = |ui: &mut egui::Ui, e: &LogEntry, wrap: bool| {
                                let mut job = egui::text::LayoutJob::default();
                                job.append(
                                    &e.timestamp,
                                    0.0,
                                    egui::TextFormat {
                                        font_id: egui::FontId::monospace(11.0),
                                        color: ui.visuals().weak_text_color(),
                                        ..Default::default()
                                    },
                                );
                                job.append(" ", 0.0, egui::TextFormat { ..Default::default() });
                                let lvl = format!("[{}]", e.level.as_str());
                                job.append(
                                    &lvl,
                                    0.0,
                                    egui::TextFormat {
                                        font_id: egui::FontId::monospace(11.0),
                                        color: e.level.color(),
                                        ..Default::default()
                                    },
                                );
                                job.append(" ", 0.0, egui::TextFormat { ..Default::default() });
                                job.append(
                                    &e.message,
                                    0.0,
                                    egui::TextFormat {
                                        font_id: egui::FontId::proportional(12.0),
                                        color: ui.visuals().text_color(),
                                        ..Default::default()
                                    },
                                );

                                let mut label = egui::Label::new(job).selectable(true);
                                if wrap { label = label.wrap(); }
                                ui.add(label);
                            };

                            if self.auto_wrap {
                                for e in &filtered_entries { render_line(ui, e, true); }
                            } else {
                                egui::ScrollArea::horizontal().show(ui, |ui| {
                                    for e in &filtered_entries { render_line(ui, e, false); }
                                });
                            }
                        });
                }
            });
    }
}
