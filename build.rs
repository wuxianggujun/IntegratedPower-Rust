// 仅用于配置 Qt 链接与 cfg，暂不生成 CXX-Qt 桥接代码
fn main() {
    cxx_qt_build::CxxQtBuilder::new()
        .qt_module("Core")
        .qt_module("Gui")
        .qt_module("Qml")
        .qt_module("Quick")
        .qt_module("QuickControls2")
        .build();
}

