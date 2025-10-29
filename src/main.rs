mod app;
mod config;
mod engine;
mod error;
mod history;
mod logger;
mod models;
mod processor;
mod ui;

use app::IntegratedPowerApp;

fn main() -> Result<(), eframe::Error> {
    // 初始化 tracing 日志输出（控制台），确保现有 tracing 宏生效
    let _ = tracing_subscriber::fmt::try_init();

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
