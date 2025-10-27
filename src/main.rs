mod app;
mod config;
mod engine;
mod error;
mod history;
mod models;
mod processor;
mod ui;

use app::IntegratedPowerApp;

fn main() -> Result<(), eframe::Error> {
    // 配置窗口选项
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("IntegratedPower"),
        ..Default::default()
    };

    // 启动应用
    eframe::run_native(
        "IntegratedPower",
        options,
        Box::new(|cc| Ok(Box::new(IntegratedPowerApp::new(cc)))),
    )
}
