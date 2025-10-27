fn main() {
    // CXX-Qt 构建配置
    cxx_qt_build::CxxQtBuilder::new()
        .qt_module("Core")
        .qt_module("Gui")
        .qt_module("Widgets")
        .qt_module("Quick")
        .qt_module("QuickControls2")
        .build();
}
