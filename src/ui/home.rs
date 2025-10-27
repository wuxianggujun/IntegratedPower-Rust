// 主页视图 - 左右分栏布局
use crate::app::IntegratedPowerApp;
use crate::processor::trait_def::ProcessorInfo;

pub fn render(app: &mut IntegratedPowerApp, ui: &mut egui::Ui) {
    // 左侧功能选择面板
    egui::SidePanel::left("function_selector_panel")
        .resizable(true)
        .default_width(350.0)
        .min_width(300.0)
        .max_width(500.0)
        .show_inside(ui, |ui| {
            render_function_list(app, ui);
        });

    // 右侧配置面板
    egui::CentralPanel::default().show_inside(ui, |ui| {
        if let Some(selected_id) = &app.selected_processor.clone() {
            render_config_panel(app, ui, &selected_id);
        } else {
            render_empty_state(ui);
        }
    });
}

fn render_function_list(app: &mut IntegratedPowerApp, ui: &mut egui::Ui) {
    ui.add_space(10.0);
    
    ui.heading("📋 处理功能");
    ui.add_space(10.0);

    // 搜索栏
    ui.horizontal(|ui| {
        ui.label("🔍");
        ui.add(
            egui::TextEdit::singleline(&mut app.search_query)
                .hint_text("搜索功能...")
                .desired_width(ui.available_width()),
        );
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    // 功能列表
    egui::ScrollArea::vertical().show(ui, |ui| {
        let processors = get_filtered_processors(app);
        
        for processor in processors.iter() {
            render_compact_card(app, ui, processor);
            ui.add_space(8.0);
        }
    });
}

fn get_filtered_processors(app: &IntegratedPowerApp) -> Vec<ProcessorInfo> {
    let processors = app.processor_manager.list_processors();
    
    if app.search_query.is_empty() {
        processors
    } else {
        let query = app.search_query.to_lowercase();
        processors
            .into_iter()
            .filter(|p| {
                p.name.to_lowercase().contains(&query)
                    || p.description.to_lowercase().contains(&query)
            })
            .collect()
    }
}

fn render_compact_card(app: &mut IntegratedPowerApp, ui: &mut egui::Ui, processor: &ProcessorInfo) {
    let is_selected = app.selected_processor.as_ref() == Some(&processor.id);
    
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

    let response = card_frame.show(ui, |ui| {
        // 设置固定宽度，确保所有卡片宽度一致
        ui.set_width(ui.available_width());
        ui.set_min_height(70.0);
        
        ui.horizontal(|ui| {
            // 图标
            if let Some(icon) = &processor.icon {
                ui.label(egui::RichText::new(icon).size(24.0));
            } else {
                ui.label(egui::RichText::new("📦").size(24.0));
            }
            
            ui.add_space(10.0);
            
            // 名称和描述 - 使用 available_width 确保填满剩余空间
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
                
                ui.label(
                    egui::RichText::new(&processor.name)
                        .size(15.0)
                        .strong(),
                );
                ui.add_space(2.0);
                ui.label(
                    egui::RichText::new(&processor.description)
                        .size(12.0)
                        .color(ui.visuals().weak_text_color()),
                );
            });
        });
    });

    if response.response.interact(egui::Sense::click()).clicked() {
        if is_selected {
            app.selected_processor = None;
        } else {
            app.selected_processor = Some(processor.id.clone());
        }
    }

    if response.response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
}

fn render_empty_state(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(100.0);
        ui.label(
            egui::RichText::new("👈 请从左侧选择一个处理功能")
                .size(18.0)
                .color(ui.visuals().weak_text_color()),
        );
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("选择后可以配置功能参数并开始处理")
                .size(14.0)
                .color(ui.visuals().weak_text_color()),
        );
    });
}

fn render_config_panel(app: &mut IntegratedPowerApp, ui: &mut egui::Ui, processor_id: &str) {
    // 获取或创建配置
    let config = app.processor_configs.get_or_create(processor_id).clone();
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(10.0);
        
        // 功能标题
        let (name, icon, description) = match processor_id {
            "export_cargo_analysis" => (
                "导出货物分析表",
                "📦",
                "分析货物数据并生成详细的分析报表"
            ),
            "auxiliary_material" => (
                "辅材处理",
                "🔧",
                "处理和整理辅材相关数据"
            ),
            _ => ("未知功能", "❓", ""),
        };
        
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(icon).size(36.0));
            ui.add_space(10.0);
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(name)
                        .size(24.0)
                        .strong(),
                );
                ui.label(
                    egui::RichText::new(description)
                        .size(13.0)
                        .color(ui.visuals().weak_text_color()),
                );
            });
        });

        ui.add_space(30.0);
        ui.separator();
        ui.add_space(20.0);

        // 输入输出配置
        let mut updated_config = config.clone();
        render_io_section(ui, &mut updated_config);

        ui.add_space(30.0);

        // 功能配置
        render_function_config(ui, processor_id, &mut updated_config);

        ui.add_space(40.0);

        // 保存配置按钮
        if ui.button("💾 保存配置").clicked() {
            *app.processor_configs.get_or_create(processor_id) = updated_config.clone();
            if let Err(e) = app.save_processor_configs() {
                app.error_message = Some(format!("保存配置失败: {}", e));
            }
        }

        ui.add_space(10.0);

        // 开始按钮
        render_start_button(app, ui, &updated_config);

        // 更新配置到 app
        *app.processor_configs.get_or_create(processor_id) = updated_config;

        ui.add_space(40.0);
    });
}

fn render_io_section(ui: &mut egui::Ui, config: &mut crate::models::ProcessorConfig) {
    ui.label(egui::RichText::new("⚙️ 输入输出设置").size(18.0).strong());
    ui.add_space(15.0);

    // 输入类型选择
    ui.horizontal(|ui| {
        ui.label("输入类型:");
        ui.radio_value(&mut config.input_type, crate::models::InputType::File, "📄 单个文件");
        ui.radio_value(&mut config.input_type, crate::models::InputType::Folder, "📁 文件夹");
    });

    ui.add_space(10.0);

    // 输入路径
    render_input_card(ui, config);
    ui.add_space(12.0);
    
    // 输出目录
    render_output_card(ui, config);
    ui.add_space(12.0);

    // 输出文件名
    render_filename_card(ui, config);
}

fn render_input_card(ui: &mut egui::Ui, config: &mut crate::models::ProcessorConfig) {
    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .rounding(10.0)
        .inner_margin(16.0)
        .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let icon = match config.input_type {
                    crate::models::InputType::File => "📄",
                    crate::models::InputType::Folder => "📁",
                };
                ui.label(egui::RichText::new(icon).size(28.0));
                ui.add_space(12.0);
                
                ui.vertical(|ui| {
                    let label = match config.input_type {
                        crate::models::InputType::File => "输入文件",
                        crate::models::InputType::Folder => "输入文件夹",
                    };
                    ui.label(egui::RichText::new(label).size(15.0).strong());
                    ui.add_space(4.0);
                    
                    if let Some(p) = &config.input_path {
                        ui.label(
                            egui::RichText::new(p.display().to_string())
                                .size(12.0)
                                .color(ui.visuals().text_color()),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new("未选择")
                                .size(12.0)
                                .italics()
                                .color(ui.visuals().weak_text_color()),
                        );
                    }
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if config.input_path.is_some() && ui.button("清除").clicked() {
                        config.input_path = None;
                    }
                    
                    let button_text = match config.input_type {
                        crate::models::InputType::File => "选择文件",
                        crate::models::InputType::Folder => "选择文件夹",
                    };
                    
                    if ui.add_sized(egui::vec2(100.0, 32.0), egui::Button::new(button_text)).clicked() {
                        let selected = match config.input_type {
                            crate::models::InputType::File => {
                                rfd::FileDialog::new()
                                    .add_filter("Excel 文件", &["xlsx", "xls"])
                                    .pick_file()
                            }
                            crate::models::InputType::Folder => {
                                rfd::FileDialog::new().pick_folder()
                            }
                        };
                        
                        if let Some(path) = selected {
                            config.input_path = Some(path);
                        }
                    }
                });
            });
        });
}

fn render_output_card(ui: &mut egui::Ui, config: &mut crate::models::ProcessorConfig) {
    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .rounding(10.0)
        .inner_margin(16.0)
        .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("📂").size(28.0));
                ui.add_space(12.0);
                
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("输出目录").size(15.0).strong());
                    ui.add_space(4.0);
                    
                    if let Some(p) = &config.output_dir {
                        ui.label(
                            egui::RichText::new(p.display().to_string())
                                .size(12.0)
                                .color(ui.visuals().text_color()),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new("未选择")
                                .size(12.0)
                                .italics()
                                .color(ui.visuals().weak_text_color()),
                        );
                    }
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if config.output_dir.is_some() && ui.button("清除").clicked() {
                        config.output_dir = None;
                    }
                    
                    if ui.add_sized(egui::vec2(100.0, 32.0), egui::Button::new("选择目录")).clicked() {
                        if let Some(selected) = rfd::FileDialog::new().pick_folder() {
                            config.output_dir = Some(selected);
                        }
                    }
                });
            });
        });
}

fn render_filename_card(ui: &mut egui::Ui, config: &mut crate::models::ProcessorConfig) {
    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .rounding(10.0)
        .inner_margin(16.0)
        .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("📝").size(28.0));
                ui.add_space(12.0);
                
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("输出文件名").size(15.0).strong());
                    ui.add_space(4.0);
                    
                    ui.add(
                        egui::TextEdit::singleline(&mut config.output_filename)
                            .hint_text("例如: 结果.xlsx")
                            .desired_width(ui.available_width() - 120.0),
                    );
                });
            });
        });
}

fn render_function_config(ui: &mut egui::Ui, processor_id: &str, config: &mut crate::models::ProcessorConfig) {
    ui.label(egui::RichText::new("📝 处理选项").size(18.0).strong());
    ui.add_space(15.0);

    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .rounding(8.0)
        .inner_margin(16.0)
        .show(ui, |ui| {
            match processor_id {
                "export_cargo_analysis" => {
                    ui.label("📊 分析选项");
                    ui.add_space(10.0);
                    
                    let mut include_summary = config.get_bool("include_summary");
                    if ui.checkbox(&mut include_summary, "包含统计汇总").changed() {
                        config.set_bool("include_summary".to_string(), include_summary);
                    }
                    
                    let mut generate_charts = config.get_bool("generate_charts");
                    if ui.checkbox(&mut generate_charts, "生成趋势图表").changed() {
                        config.set_bool("generate_charts".to_string(), generate_charts);
                    }
                    
                    let mut export_logs = config.get_bool("export_logs");
                    if ui.checkbox(&mut export_logs, "导出详细日志").changed() {
                        config.set_bool("export_logs".to_string(), export_logs);
                    }
                }
                "auxiliary_material" => {
                    ui.label("🔧 处理选项");
                    ui.add_space(10.0);
                    
                    let mut auto_classify = config.get_bool("auto_classify");
                    if ui.checkbox(&mut auto_classify, "自动分类").changed() {
                        config.set_bool("auto_classify".to_string(), auto_classify);
                    }
                    
                    let mut remove_duplicates = config.get_bool("remove_duplicates");
                    if ui.checkbox(&mut remove_duplicates, "去除重复项").changed() {
                        config.set_bool("remove_duplicates".to_string(), remove_duplicates);
                    }
                    
                    let mut generate_summary = config.get_bool("generate_summary");
                    if ui.checkbox(&mut generate_summary, "生成汇总表").changed() {
                        config.set_bool("generate_summary".to_string(), generate_summary);
                    }
                }
                _ => {}
            }
        });
}

fn render_start_button(app: &mut IntegratedPowerApp, ui: &mut egui::Ui, config: &crate::models::ProcessorConfig) {
    ui.vertical_centered(|ui| {
        let can_start = config.input_path.is_some() 
            && config.output_dir.is_some() 
            && !config.output_filename.is_empty();
        
        let button_color = if can_start {
            egui::Color32::from_rgb(76, 175, 80)
        } else {
            ui.visuals().widgets.inactive.bg_fill
        };
        
        let button = egui::Button::new(
            egui::RichText::new("🚀 开始处理")
                .size(18.0)
                .strong()
                .color(if can_start {
                    egui::Color32::WHITE
                } else {
                    ui.visuals().weak_text_color()
                }),
        )
        .fill(button_color)
        .rounding(10.0)
        .min_size(egui::vec2(220.0, 55.0));
        
        let response = ui.add_enabled(can_start, button);
        
        if response.clicked() {
            app.error_message = Some("处理逻辑将在任务 12 中实现".to_string());
        }
        
        ui.add_space(8.0);
        
        if !can_start {
            let missing = if config.input_path.is_none() {
                "请选择输入路径"
            } else if config.output_dir.is_none() {
                "请选择输出目录"
            } else {
                "请输入输出文件名"
            };
            
            ui.label(
                egui::RichText::new(format!("⚠ {}", missing))
                    .size(13.0)
                    .italics()
                    .color(ui.visuals().warn_fg_color),
            );
        } else {
            ui.label(
                egui::RichText::new("✓ 准备就绪，点击开始处理")
                    .size(13.0)
                    .color(egui::Color32::from_rgb(76, 175, 80)),
            );
        }
    });
}
