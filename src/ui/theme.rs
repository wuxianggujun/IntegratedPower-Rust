use crate::config::manager::Theme as ConfigTheme;

/// UI 主题颜色
#[derive(Debug, Clone)]
pub struct ThemeColors {
    // 主色调
    pub primary: String,
    // 背景色
    pub background: String,
    // 次要背景
    pub secondary_background: String,
    // 文字色
    pub text: String,
    // 次要文字
    pub secondary_text: String,
    // 边框色
    pub border: String,
    // 成功色
    pub success: String,
    // 警告色
    pub warning: String,
    // 错误色
    pub error: String,
}

impl ThemeColors {
    /// 创建浅色主题
    pub fn light() -> Self {
        Self {
            primary: "#2196F3".to_string(),
            background: "#FFFFFF".to_string(),
            secondary_background: "#F5F5F5".to_string(),
            text: "#212121".to_string(),
            secondary_text: "#757575".to_string(),
            border: "#E0E0E0".to_string(),
            success: "#4CAF50".to_string(),
            warning: "#FF9800".to_string(),
            error: "#F44336".to_string(),
        }
    }

    /// 创建深色主题
    pub fn dark() -> Self {
        Self {
            primary: "#42A5F5".to_string(),
            background: "#1E1E1E".to_string(),
            secondary_background: "#2D2D2D".to_string(),
            text: "#E0E0E0".to_string(),
            secondary_text: "#B0B0B0".to_string(),
            border: "#404040".to_string(),
            success: "#66BB6A".to_string(),
            warning: "#FFA726".to_string(),
            error: "#EF5350".to_string(),
        }
    }

    /// 从配置主题创建
    pub fn from_config_theme(theme: ConfigTheme) -> Self {
        match theme {
            ConfigTheme::Light => Self::light(),
            ConfigTheme::Dark => Self::dark(),
        }
    }

    /// 生成 Qt 样式表
    pub fn to_stylesheet(&self) -> String {
        format!(
            r#"
QMainWindow {{
    background-color: {background};
}}

QWidget {{
    color: {text};
    background-color: {background};
    font-family: "Segoe UI", "Microsoft YaHei", sans-serif;
    font-size: 14px;
}}

QPushButton {{
    background-color: {primary};
    color: white;
    border: none;
    border-radius: 4px;
    padding: 8px 16px;
    min-width: 80px;
}}

QPushButton:hover {{
    background-color: {primary};
    opacity: 0.9;
}}

QPushButton:pressed {{
    background-color: {primary};
    opacity: 0.8;
}}

QPushButton:disabled {{
    background-color: {secondary_background};
    color: {secondary_text};
}}

QLineEdit, QTextEdit {{
    background-color: {secondary_background};
    border: 1px solid {border};
    border-radius: 4px;
    padding: 6px;
    color: {text};
}}

QLineEdit:focus, QTextEdit:focus {{
    border: 2px solid {primary};
}}

QListWidget, QTreeWidget, QTableWidget {{
    background-color: {background};
    border: 1px solid {border};
    border-radius: 4px;
    color: {text};
}}

QListWidget::item:selected, QTreeWidget::item:selected, QTableWidget::item:selected {{
    background-color: {primary};
    color: white;
}}

QListWidget::item:hover, QTreeWidget::item:hover, QTableWidget::item:hover {{
    background-color: {secondary_background};
}}

QProgressBar {{
    border: 1px solid {border};
    border-radius: 4px;
    text-align: center;
    background-color: {secondary_background};
}}

QProgressBar::chunk {{
    background-color: {primary};
    border-radius: 3px;
}}

QLabel {{
    color: {text};
    background-color: transparent;
}}

QGroupBox {{
    border: 1px solid {border};
    border-radius: 4px;
    margin-top: 10px;
    padding-top: 10px;
    color: {text};
}}

QGroupBox::title {{
    subcontrol-origin: margin;
    subcontrol-position: top left;
    padding: 0 5px;
    color: {text};
}}

QMenuBar {{
    background-color: {background};
    color: {text};
}}

QMenuBar::item:selected {{
    background-color: {secondary_background};
}}

QMenu {{
    background-color: {background};
    border: 1px solid {border};
    color: {text};
}}

QMenu::item:selected {{
    background-color: {primary};
    color: white;
}}

QToolBar {{
    background-color: {secondary_background};
    border: none;
    spacing: 3px;
}}

QStatusBar {{
    background-color: {secondary_background};
    color: {text};
}}

QScrollBar:vertical {{
    border: none;
    background-color: {secondary_background};
    width: 12px;
    margin: 0px;
}}

QScrollBar::handle:vertical {{
    background-color: {border};
    min-height: 20px;
    border-radius: 6px;
}}

QScrollBar::handle:vertical:hover {{
    background-color: {secondary_text};
}}

QScrollBar::add-line:vertical, QScrollBar::sub-line:vertical {{
    height: 0px;
}}

QScrollBar:horizontal {{
    border: none;
    background-color: {secondary_background};
    height: 12px;
    margin: 0px;
}}

QScrollBar::handle:horizontal {{
    background-color: {border};
    min-width: 20px;
    border-radius: 6px;
}}

QScrollBar::handle:horizontal:hover {{
    background-color: {secondary_text};
}}

QScrollBar::add-line:horizontal, QScrollBar::sub-line:horizontal {{
    width: 0px;
}}

QComboBox {{
    background-color: {secondary_background};
    border: 1px solid {border};
    border-radius: 4px;
    padding: 6px;
    color: {text};
}}

QComboBox:hover {{
    border: 1px solid {primary};
}}

QComboBox::drop-down {{
    border: none;
}}

QComboBox QAbstractItemView {{
    background-color: {background};
    border: 1px solid {border};
    selection-background-color: {primary};
    color: {text};
}}

QCheckBox, QRadioButton {{
    color: {text};
}}

QCheckBox::indicator, QRadioButton::indicator {{
    width: 18px;
    height: 18px;
    border: 2px solid {border};
    border-radius: 3px;
    background-color: {background};
}}

QCheckBox::indicator:checked, QRadioButton::indicator:checked {{
    background-color: {primary};
    border-color: {primary};
}}

QTabWidget::pane {{
    border: 1px solid {border};
    background-color: {background};
}}

QTabBar::tab {{
    background-color: {secondary_background};
    color: {text};
    padding: 8px 16px;
    border: 1px solid {border};
    border-bottom: none;
    border-top-left-radius: 4px;
    border-top-right-radius: 4px;
}}

QTabBar::tab:selected {{
    background-color: {background};
    color: {primary};
}}

QTabBar::tab:hover {{
    background-color: {border};
}}
"#,
            background = self.background,
            secondary_background = self.secondary_background,
            text = self.text,
            secondary_text = self.secondary_text,
            border = self.border,
            primary = self.primary,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_theme() {
        let theme = ThemeColors::light();
        assert_eq!(theme.primary, "#2196F3");
        assert_eq!(theme.background, "#FFFFFF");
    }

    #[test]
    fn test_dark_theme() {
        let theme = ThemeColors::dark();
        assert_eq!(theme.primary, "#42A5F5");
        assert_eq!(theme.background, "#1E1E1E");
    }

    #[test]
    fn test_stylesheet_generation() {
        let theme = ThemeColors::light();
        let stylesheet = theme.to_stylesheet();
        assert!(stylesheet.contains("QMainWindow"));
        assert!(stylesheet.contains("#2196F3"));
    }

    #[test]
    fn test_from_config_theme() {
        let light = ThemeColors::from_config_theme(ConfigTheme::Light);
        assert_eq!(light.background, "#FFFFFF");

        let dark = ThemeColors::from_config_theme(ConfigTheme::Dark);
        assert_eq!(dark.background, "#1E1E1E");
    }
}
