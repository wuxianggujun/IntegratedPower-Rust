// 处理视图
use crate::app::IntegratedPowerApp;

pub fn render(app: &IntegratedPowerApp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);
        
        ui.heading("正在处理...");
        ui.add_space(20.0);

        ui.label(format!("当前文件: {}", app.progress.current_file));
        ui.add_space(10.0);

        let progress = app.progress.percentage / 100.0;
        ui.add(
            egui::ProgressBar::new(progress)
                .text(format!("{:.0}%", app.progress.percentage))
                .animate(true),
        );

        ui.add_space(10.0);

        ui.label(format!(
            "已处理: {}/{} 文件",
            app.progress.processed_files, app.progress.total_files
        ));

        ui.add_space(20.0);

        if ui.button("取消处理").clicked() {
            // 取消逻辑将在任务 12 中实现
        }
    });
}
