use eframe::egui;
use crate::app::{EucarinogammarusApp, SortDirection};
use crate::db::load_records_sorted;

pub fn render(ui: &mut egui::Ui, app: &mut EucarinogammarusApp) {
    ui.horizontal(|ui| {
        ui.label("Поиск:");
        ui.text_edit_singleline(&mut app.search_term);
    });
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::Grid::new("records_grid")
                .striped(true)
                .spacing([5.0, 5.0])
                .show(ui, |ui| {
                    // Заголовки столбцов
                    // Функция для создания кликабельного заголовка
                        let make_sortable_header = |ui: &mut egui::Ui, app: &mut EucarinogammarusApp, db_column: &str, display_name: &str| {
                            let is_sorted_by_this = app.sort_column == db_column;
                            
                            // Определяем текст заголовка с индикатором направления сортировки
                            let text = if !is_sorted_by_this {
                                display_name.to_string()
                            } else {
                               match app.sort_direction {
                                    SortDirection::Ascending => format!("▲ {}", display_name),
                                    SortDirection::Descending => format!("▼ {}", display_name),
                                }
                            };
                            
                            if ui.button(text).clicked() {
                                // Если колонка уже выбрана, меняем направление сортировки
                                if is_sorted_by_this {
                                    app.sort_direction = match app.sort_direction {
                                        SortDirection::Ascending => SortDirection::Descending,
                                        SortDirection::Descending => SortDirection::Ascending,
                                    };
                                } else {
                                     app.sort_column = db_column.to_string();
                                }
                                if let Ok(conn) = app.conn.lock() {
                                    if let Ok(records) = load_records_sorted(&conn, &app.sort_column, app.sort_direction) {
                                        app.records = records;
                                    }
                                }
                            }
                        };
                        
                        // Заголовки с возможностью сортировки
                        make_sortable_header(ui, app, "id", "ID");
                        make_sortable_header(ui, app, "Код", "Код");
                        make_sortable_header(ui, app, "Род", "Род");
                        make_sortable_header(ui, app, "Вид", "Вид");
                        make_sortable_header(ui, app, "Размеры_мм", "Размеры мм");
                        make_sortable_header(ui, app, "Тело", "Тело");
                        make_sortable_header(ui, app, "Окраска", "Окраска");
                        make_sortable_header(ui, app, "Распространение", "Распространение");
                        make_sortable_header(ui, app, "Глубина_м", "Глубина м");
                        make_sortable_header(ui, app, "Вооруж_тела", "Вооруж. тела");
                        make_sortable_header(ui, app, "Средний_ряд_I_VII", "Средний ряд I-VII");
                        make_sortable_header(ui, app, "Средн_ряд_VIII_X", "Средн. ряд VIII-X");
                        make_sortable_header(ui, app, "Сред_ряд_урозом", "Сред. ряд урозом");
                        make_sortable_header(ui, app, "Боковой_ряд", "Боковой ряд");
                        make_sortable_header(ui, app, "Краевой_ряд", "Краевой ряд");
                        make_sortable_header(ui, app, "Особен_воор", "Особен. воор.");
                        make_sortable_header(ui, app, "Эпимир_пласт", "Эпимир. пласт.");
                        make_sortable_header(ui, app, "Верх_антенны", "Верх. антенны");
                        make_sortable_header(ui, app, "Прид_жгутик", "Прид. жгутик");
                        make_sortable_header(ui, app, "Нижн_антенны", "Нижн. антенны");
                        make_sortable_header(ui, app, "Базип_III_V", "Базип. III-V");
                        make_sortable_header(ui, app, "Уроподы_III", "Уроподы III");
                        make_sortable_header(ui, app, "Головн_сегм", "Головн. сегм.");
                        make_sortable_header(ui, app, "Глаза", "Глаза");
                        make_sortable_header(ui, app, "Тельсон", "Тельсон");
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