use eframe::egui;
use crate::app::EucarinogammarusApp;

pub fn render(ui: &mut egui::Ui, app: &mut EucarinogammarusApp) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading("Добавление новой записи");
        
        ui.horizontal(|ui| {
            ui.label("Код:");
            ui.text_edit_singleline(&mut app.new_record.code);
        });
        
        ui.horizontal(|ui| {
            ui.label("Род:");
            ui.text_edit_singleline(&mut app.new_record.genus);
        });
        
        ui.horizontal(|ui| {
            ui.label("Вид:");
            ui.text_edit_singleline(&mut app.new_record.species);
        });
        
        ui.horizontal(|ui| {
            ui.label("Размеры мм:");
            ui.text_edit_singleline(&mut app.new_record.size_mm);
        });
        
        ui.horizontal(|ui| {
            ui.label("Тело:");
            ui.text_edit_singleline(&mut app.new_record.body);
        });
        
        if ui.button("Добавить запись").clicked() {
            if let Err(e) = app.add_record() {
                app.status_message = format!("Ошибка: {}", e);
            }
        }
    });
}