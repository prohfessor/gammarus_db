use eframe::egui;
use crate::app::EucarinogammarusApp;

pub fn render(ui: &mut egui::Ui, app: &mut EucarinogammarusApp) {
    ui.heading("Редактирование записи");
    
    ui.horizontal(|ui| {
        ui.label("ID записи:");
        ui.text_edit_singleline(&mut app.edit_id);
    });
    
    ui.horizontal(|ui| {
        ui.label("Столбец для редактирования:");
        egui::ComboBox::from_id_source("edit_column")
            .selected_text(&app.edit_column)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.edit_column, "Код".to_string(), "Код");
                ui.selectable_value(&mut app.edit_column, "Род".to_string(), "Род");
                ui.selectable_value(&mut app.edit_column, "Вид".to_string(), "Вид");
                ui.selectable_value(&mut app.edit_column, "Размеры_мм".to_string(), "Размеры мм");
                ui.selectable_value(&mut app.edit_column, "Тело".to_string(), "Тело");
                ui.selectable_value(&mut app.edit_column, "Окраска".to_string(), "Окраска");
                ui.selectable_value(&mut app.edit_column, "Распространение".to_string(), "Распространение");
                ui.selectable_value(&mut app.edit_column, "Глубина_м".to_string(), "Глубина м");
                ui.selectable_value(&mut app.edit_column, "Вооруж_тела".to_string(), "Вооруж тела");
                ui.selectable_value(&mut app.edit_column, "Средний_ряд_I_VII".to_string(), "Средний ряд I-VII");
                ui.selectable_value(&mut app.edit_column, "Средн_ряд_VIII_X".to_string(), "Средн ряд VIII-X");
                ui.selectable_value(&mut app.edit_column, "Сред_ряд_урозом".to_string(), "Сред ряд урозом");
                ui.selectable_value(&mut app.edit_column, "Боковой_ряд".to_string(), "Боковой ряд");
                ui.selectable_value(&mut app.edit_column, "Краевой_ряд".to_string(), "Краевой ряд");
                ui.selectable_value(&mut app.edit_column, "Особен_воор".to_string(), "Особен воор");
                ui.selectable_value(&mut app.edit_column, "Эпимир_пласт".to_string(), "Эпимир пласт");
                ui.selectable_value(&mut app.edit_column, "Верх_антенны".to_string(), "Верх антенны");
                ui.selectable_value(&mut app.edit_column, "Прид_жгутик".to_string(), "Прид жгутик");
                ui.selectable_value(&mut app.edit_column, "Нижн_антенны".to_string(), "Нижн антенны");
                ui.selectable_value(&mut app.edit_column, "Базип_III_V".to_string(), "Базип III-V");
                ui.selectable_value(&mut app.edit_column, "Уроподы_III".to_string(), "Уроподы III");
                ui.selectable_value(&mut app.edit_column, "Головн_сегм".to_string(), "Головн сегм");
                ui.selectable_value(&mut app.edit_column, "Глаза".to_string(), "Глаза");
                ui.selectable_value(&mut app.edit_column, "Тельсон".to_string(), "Тельсон");
            });
    });
    
    ui.horizontal(|ui| {
        ui.label("Новое значение:");
        ui.text_edit_singleline(&mut app.edit_value);
    });
    
    if ui.button("Обновить запись").clicked() {
        if let Err(e) = app.edit_record() {
            app.status_message = format!("Ошибка: {}", e);
        }
    }
}