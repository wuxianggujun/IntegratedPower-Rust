// 设置视图
use crate::app::IntegratedPowerApp;

pub fn render(_app: &IntegratedPowerApp, ui: &mut egui::Ui) {
    ui.add_space(20.0);
    ui.heading("设置");
    ui.add_space(10.0);
    ui.label("设置界面将在任务 13 中实现");
}
