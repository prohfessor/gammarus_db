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
        egui::ScrollArea::horizontal().show(ui, |ui| {
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
                    ui.label("Тело");
                    ui.label("Окраска");
                    ui.label("Распространение");
                    ui.label("Глубина м");
                    ui.label("Вооруж. тела");
                    ui.label("Средний ряд I-VII");
                    ui.label("Средн. ряд VIII-X");
                    ui.label("Сред. ряд урозом");
                    ui.label("Боковой ряд");
                    ui.label("Краевой ряд");
                    ui.label("Особен. воор.");
                    ui.label("Эпимир. пласт.");
                    ui.label("Верх. антенны");
                    ui.label("Прид. жгутик");
                    ui.label("Нижн. антенны");
                    ui.label("Базип. III-V");
                    ui.label("Уроподы III");
                    ui.label("Головн. сегм.");
                    ui.label("Глаза");
                    ui.label("Тельсон");
                    ui.end_row();
                
    
                    // Отображение записей
                    for record in app.filtered_records() {
                        ui.label(record.id.to_string());
                        ui.label(&record.code);
                        ui.label(&record.genus);
                        ui.label(&record.species);
                        ui.label(&record.size_mm);
                        ui.label(&record.body);
                        ui.label(&record.coloration);
                        ui.label(&record.distribution);
                        ui.label(&record.depth_m);
                        ui.label(&record.body_armament);
                        ui.label(&record.median_row_i_vii);
                        ui.label(&record.median_row_viii_x);
                        ui.label(&record.median_row_urozom);
                        ui.label(&record.lateral_row);
                        ui.label(&record.marginal_row);
                        ui.label(&record.special_armament);
                        ui.label(&record.epimeral_plate);
                        ui.label(&record.upper_antennae);
                        ui.label(&record.accessory_flagellum);
                        ui.label(&record.lower_antennae);
                        ui.label(&record.basipodite_iii_v);
                        ui.label(&record.uropods_iii);
                        ui.label(&record.head_segment);
                        ui.label(&record.eyes);
                        ui.label(&record.telson);
                        ui.end_row();
                    }
                });
        });
    });
}