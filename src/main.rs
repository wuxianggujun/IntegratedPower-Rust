mod app;
mod config;
mod controller;
mod engine;
mod error;
mod error_logger;
mod history;
mod models;
mod processor;
mod ui;

use app::Application;

fn main() {
    // 初始化日志系统
    tracing_subscriber::fmt::init();

    // 创建并运行应用程序
    match Application::new() {
        Ok(mut app) => {
            if let Err(e) = app.run() {
                eprintln!("应用程序运行错误: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("应用程序初始化失败: {}", e);
            std::process::exit(1);
        }
    }
}
