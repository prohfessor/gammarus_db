use eframe::egui;
use crate::app::EucarinogammarusApp;

pub fn render(ui: &mut egui::Ui, app: &mut EucarinogammarusApp) {
    ui.heading("Удаление записи");
    
    ui.horizontal(|ui| {
        ui.label("ID записи для удаления:");
        ui.text_edit_singleline(&mut app.delete_id);
    });
    
    if ui.button("Удалить запись").clicked() {
        if let Err(e) = app.delete_record() {
            app.status_message = format!("Ошибка: {}", e);
        }
    }
}