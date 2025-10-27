// Qt 应用程序 - 使用 CXX-Qt Lib（不依赖自定义 CXX-Qt 桥接生成）
use crate::controller::main_controller::MainController;
use crate::error::Result;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QQuickStyle, QUrl, QString};
use std::path::PathBuf;
use std::sync::Arc;

/// Qt 应用程序包装器
pub struct QtApp {
    _controller: Arc<MainController>,
}

impl QtApp {
    /// 创建新的 Qt 应用程序
    pub fn new(controller: MainController) -> Self {
        Self {
            _controller: Arc::new(controller),
        }
    }

    /// 运行 Qt 应用程序
    pub fn run(&self) -> Result<()> {
        tracing::info!("Initializing CXX-Qt application");

        // 创建 Qt GUI 应用程序
        let mut app = QGuiApplication::new();
        // 设置应用名称与版本（便于调试与关于信息）
        app.pin_mut()
            .set_application_name(&QString::from("IntegratedPower"));
        app.pin_mut()
            .set_application_version(&QString::from(env!("CARGO_PKG_VERSION")));

        // 设置 Qt Quick Controls 风格，启用可自定义的控件样式。
        // 优先使用环境变量 INTEGRATED_POWER_QT_STYLE 指定的风格，例如：Material/Fusion/Basic。
        // 未设置时默认使用 Fusion，并设置 Basic 作为回退。
        QQuickStyle::set_fallback_style(&QString::from("Basic"));
        let style = std::env::var("INTEGRATED_POWER_QT_STYLE").unwrap_or_else(|_| "Fusion".to_string());
        QQuickStyle::set_style(&QString::from(style));

        // 创建 QML 引擎
        let mut engine = QQmlApplicationEngine::new();

        // 如需与 QML 交互，可在后续恢复 CXX-Qt 桥接并注册对象到 QML 上下文

        // 解析 QML 路径（支持环境变量、工作目录与可执行文件目录）
        let qml_path = Self::resolve_qml_path();
        let qml_dir = qml_path.parent().unwrap_or_else(|| std::path::Path::new("."));

        // 增加导入路径，便于 QML 中使用相对 import
        engine
            .pin_mut()
            .add_import_path(&QString::from(qml_dir.to_string_lossy().as_ref()));

        let url = QUrl::from_local_file(&QString::from(qml_path.to_string_lossy().as_ref()));
        engine.pin_mut().load(&url);
        tracing::info!("Loaded QML file: {:?}", qml_path);

        // 运行事件循环
        let _ = app.pin_mut().exec();

        Ok(())
    }

    /// 解析 QML 主文件路径，按以下顺序：
    /// 1) 环境变量 INTEGRATED_POWER_QML 指定路径
    /// 2) 当前工作目录下 resources/qml/MainWindow.qml
    /// 3) 可执行文件所在目录的 resources/qml/MainWindow.qml
    fn resolve_qml_path() -> PathBuf {
        if let Ok(env_path) = std::env::var("INTEGRATED_POWER_QML") {
            let p = PathBuf::from(env_path);
            if p.exists() {
                return p;
            }
        }

        // 工作目录
        if let Ok(cwd) = std::env::current_dir() {
            let p = cwd.join("resources/qml/MainWindow.qml");
            if p.exists() {
                return p;
            }
        }

        // 可执行目录
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                let p = dir.join("resources/qml/MainWindow.qml");
                if p.exists() {
                    return p;
                }
            }
        }

        // 回退到工作目录位置（即使不存在，也返回预期路径，供错误信息打印）
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("resources/qml/MainWindow.qml")
    }
}
