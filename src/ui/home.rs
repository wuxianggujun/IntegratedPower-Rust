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
            "excel_structure_analyzer" => (
                "Excel结构分析器",
                "🔍",
                "分析Excel文件的单个 Sheet 结构，结果输出到日志面板"
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
        render_io_section(ui, &mut updated_config, processor_id);

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

fn render_io_section(ui: &mut egui::Ui, config: &mut crate::models::ProcessorConfig, processor_id: &str) {
    ui.label(egui::RichText::new("⚙️ 输入输出设置").size(18.0).strong());
    ui.add_space(15.0);

    // Excel分析器只需要输入文件
    let is_excel_analyzer = processor_id == "excel_structure_analyzer";
    
    if !is_excel_analyzer {
        // 输入类型选择
        ui.horizontal(|ui| {
            ui.label("输入类型:");
            ui.radio_value(&mut config.input_type, crate::models::InputType::File, "📄 单个文件");
            ui.radio_value(&mut config.input_type, crate::models::InputType::Folder, "📁 文件夹");
        });

        ui.add_space(10.0);
    }

    // 输入路径
    render_input_card(ui, config, is_excel_analyzer);
    ui.add_space(12.0);
    
    // Sheet 选择器（仅当选择了文件时显示）
    if config.input_type == crate::models::InputType::File && config.input_path.is_some() {
        render_sheet_selector(ui, config);
        ui.add_space(12.0);
    }
    
    // Excel分析器不需要输出设置
    if !is_excel_analyzer {
        // 输出目录
        render_output_card(ui, config);
        ui.add_space(12.0);

        // 输出文件名
        render_filename_card(ui, config);
    } else {
        // 为Excel分析器显示提示信息
        egui::Frame::none()
            .fill(ui.visuals().faint_bg_color)
            .rounding(10.0)
            .inner_margin(16.0)
            .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("📋").size(28.0));
                    ui.add_space(12.0);
                    
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("输出设置").size(15.0).strong());
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new("分析结果将直接输出到日志面板")
                                .size(12.0)
                                .color(ui.visuals().weak_text_color()),
                        );
                    });
                });
            });
    }
}

fn render_input_card(ui: &mut egui::Ui, config: &mut crate::models::ProcessorConfig, force_file: bool) {
    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .rounding(10.0)
        .inner_margin(16.0)
        .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let icon = if force_file || config.input_type == crate::models::InputType::File {
                    "📄"
                } else {
                    "📁"
                };
                ui.label(egui::RichText::new(icon).size(28.0));
                ui.add_space(12.0);
                
                ui.vertical(|ui| {
                    let label = if force_file || config.input_type == crate::models::InputType::File {
                        "输入文件"
                    } else {
                        "输入文件夹"
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
                    
                    let button_text = if force_file || config.input_type == crate::models::InputType::File {
                        "选择文件"
                    } else {
                        "选择文件夹"
                    };
                    
                    if ui.add_sized(egui::vec2(100.0, 32.0), egui::Button::new(button_text)).clicked() {
                        let selected = if force_file || config.input_type == crate::models::InputType::File {
                            rfd::FileDialog::new()
                                .add_filter("Excel 文件", &["xlsx", "xls"])
                                .pick_file()
                        } else {
                            rfd::FileDialog::new().pick_folder()
                        };
                        
                        if let Some(path) = selected {
                            config.input_path = Some(path);
                        }
                    }
                });
            });
        });
}

fn render_sheet_selector(ui: &mut egui::Ui, config: &mut crate::models::ProcessorConfig) {
    egui::Frame::none()
        .fill(ui.visuals().faint_bg_color)
        .rounding(10.0)
        .inner_margin(16.0)
        .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("📋").size(28.0));
                ui.add_space(12.0);
                
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("选择 Sheet").size(15.0).strong());
                    ui.add_space(4.0);
                    
                    // 如果还没有加载 sheet 列表，显示加载按钮
                    if config.available_sheets.is_empty() {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("点击加载 Sheet 列表")
                                    .size(12.0)
                                    .italics()
                                    .color(ui.visuals().weak_text_color()),
                            );
                            
                            if ui.button("🔄 加载").clicked() {
                                // 从文件加载 sheet 列表
                                match config.load_sheets_from_file() {
                                    Ok(_) => {
                                        crate::log_info!("成功加载 {} 个 Sheet", config.available_sheets.len());
                                    }
                                    Err(e) => {
                                        crate::log_error!("加载 Sheet 失败: {}", e);
                                        // 显示错误提示
                                    }
                                }
                            }
                        });
                    } else {
                        // 显示 sheet 下拉选择框
                        let selected_text = config.selected_sheet.as_ref()
                            .map(|s| s.as_str())
                            .unwrap_or("所有 Sheet");
                        
                        egui::ComboBox::from_label("")
                            .selected_text(selected_text)
                            .show_ui(ui, |ui| {
                                // "所有 Sheet" 选项
                                if ui.selectable_value(&mut config.selected_sheet, None, "所有 Sheet").clicked() {
                                    // 选中了所有 Sheet
                                }
                                
                                ui.separator();
                                
                                // 各个 sheet 选项
                                for sheet in config.available_sheets.clone() {
                                    let is_selected = config.selected_sheet.as_ref() == Some(&sheet);
                                    if ui.selectable_label(is_selected, &sheet).clicked() {
                                        config.selected_sheet = Some(sheet.clone());
                                    }
                                }
                            });
                        
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(format!("共 {} 个 Sheet", config.available_sheets.len()))
                                .size(11.0)
                                .color(ui.visuals().weak_text_color()),
                        );
                    }
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if !config.available_sheets.is_empty() && ui.button("重新加载").clicked() {
                        config.available_sheets.clear();
                        config.selected_sheet = None;
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
                "excel_structure_analyzer" => {
                    ui.label("🔍 分析选项");
                    ui.add_space(10.0);

                    let mut analyze_structure = config.get_bool("analyze_structure");
                    if ui.checkbox(&mut analyze_structure, "分析表格结构").changed() {
                        config.set_bool("analyze_structure".to_string(), analyze_structure);
                    }
                    
                    let mut detailed_output = config.get_bool("detailed_output");
                    if ui.checkbox(&mut detailed_output, "详细输出模式").changed() {
                        config.set_bool("detailed_output".to_string(), detailed_output);
                    }
                    
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("💡 提示: 分析结果将输出到日志面板")
                        .size(12.0)
                        .color(ui.visuals().weak_text_color()));
                }
                _ => {}
            }
        });
}

fn render_start_button(app: &mut IntegratedPowerApp, ui: &mut egui::Ui, config: &crate::models::ProcessorConfig) {
    ui.vertical_centered(|ui| {
        // Excel分析器只需要输入文件，不需要输出目录
        let is_excel_analyzer = app.selected_processor.as_ref() == Some(&"excel_structure_analyzer".to_string());
        let can_start = config.input_path.is_some()
            && (is_excel_analyzer || (config.output_dir.is_some() && !config.output_filename.is_empty()));
        
        let button_color = if can_start {
            egui::Color32::from_rgb(76, 175, 80)
        } else {
            ui.visuals().widgets.inactive.bg_fill
        };
        
        let button_text = if is_excel_analyzer {
            "🔍 开始分析"
        } else {
            "🚀 开始处理"
        };
        
        let button = egui::Button::new(
            egui::RichText::new(button_text)
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
            if is_excel_analyzer {
                // 执行 Excel 分析（仅处理选中的单个 Sheet）
                if let Some(input_path) = &config.input_path {
                    let analyzer = crate::processor::examples::ExcelStructureAnalyzer::new();
                    let sheet = config.selected_sheet.as_deref();
                    match analyzer.analyze_excel_structure(input_path, sheet) {
                        Ok(_) => {
                            crate::log_info!("Excel分析完成，请查看日志面板获取详细结果");
                            // 自动打开日志查看器
                            app.log_viewer.show = true;
                        }
                        Err(e) => {
                            app.error_message = Some(format!("Excel分析失败: {}", e));
                        }
                    }
                }
            } else {
                app.error_message = Some("处理逻辑将在任务 12 中实现".to_string());
            }
        }
        
        ui.add_space(8.0);
        
        if !can_start {
            let missing = if config.input_path.is_none() {
                "请选择输入文件"
            } else if !is_excel_analyzer && config.output_dir.is_none() {
                "请选择输出目录"
            } else if !is_excel_analyzer && config.output_filename.is_empty() {
                "请输入输出文件名"
            } else {
                "未知错误"
            };
            
            ui.label(
                egui::RichText::new(format!("⚠ {}", missing))
                    .size(13.0)
                    .italics()
                    .color(ui.visuals().warn_fg_color),
            );
        } else {
            let ready_text = if is_excel_analyzer {
                "✓ 准备就绪，点击开始分析"
            } else {
                "✓ 准备就绪，点击开始处理"
            };
            
            ui.label(
                egui::RichText::new(ready_text)
                    .size(13.0)
                    .color(egui::Color32::from_rgb(76, 175, 80)),
            );
        }
    });
}
