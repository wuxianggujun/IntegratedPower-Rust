// 历史记录视图
use crate::app::IntegratedPowerApp;

pub fn render(_app: &IntegratedPowerApp, ui: &mut egui::Ui) {
    ui.add_space(20.0);
    ui.heading("历史记录");
    ui.add_space(10.0);
    ui.label("历史记录界面将在任务 14 中实现");
}
