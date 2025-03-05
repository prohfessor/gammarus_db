use eframe::egui;
use crate::app::EucarinogammarusApp;
use crate::db::{load_records, load_records_sorted};

pub fn render(ui: &mut egui::Ui, app: &mut EucarinogammarusApp) {
    ui.horizontal(|ui| {
        ui.label("Поиск:");
        ui.text_edit_singleline(&mut app.search_term);
        
        ui.label("Сортировать по:");
        egui::ComboBox::from_id_source("sort_column")
            .selected_text(&app.sort_column)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.sort_column, "id".to_string(), "ID");
                ui.selectable_value(&mut app.sort_column, "Код".to_string(), "Код");
                ui.selectable_value(&mut app.sort_column, "Род".to_string(), "Род");
                ui.selectable_value(&mut app.sort_column, "Вид".to_string(), "Вид");
            });
        
        if ui.button("Применить").clicked() {
            // Обновление записей с учетом сортировки
            if let Ok(conn) = app.conn.lock() {
                if let Ok(records) = load_records_sorted(&conn, &app.sort_column) {
                    app.records = records;
                }
            }
        }
    });
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("records_grid")
            .striped(true)
            .spacing([5.0, 5.0])
            .show(ui, |ui| {
                // Заголовки столбцов
                ui.label("ID");
                ui.label("Код");
                ui.label("Род");
                ui.label("Вид");
                ui.label("Размеры мм");
                ui.end_row();
                
                // Отображение записей
                for record in app.filtered_records() {
                    ui.label(record.id.to_string());
                    ui.label(&record.code);
                    ui.label(&record.genus);
                    ui.label(&record.species);
                    ui.label(&record.size_mm);
                    ui.end_row();
                }
            });
    });
}