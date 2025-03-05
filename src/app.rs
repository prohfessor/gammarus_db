use eframe::egui;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::db::{Eucarinogammarus, import_csv, load_records};
use crate::views::{view_tab, add_tab, edit_tab, delete_tab};

#[derive(Debug, PartialEq)]
pub enum Tab {
    View,
    Add,
    Edit,
    Delete,
}

pub struct EucarinogammarusApp {
    pub conn: Arc<Mutex<Connection>>,
    pub records: Vec<Eucarinogammarus>,
    pub selected_tab: Tab,
    pub search_term: String,
    pub sort_column: String,
    pub new_record: NewRecord,
    pub edit_id: String,
    pub edit_column: String,
    pub edit_value: String,
    pub delete_id: String,
    pub status_message: String,
}

#[derive(Debug, Default, Clone)]
pub struct NewRecord {
    pub code: String,
    pub genus: String,
    pub species: String,
    pub size_mm: String,
    pub body: String,
}

impl EucarinogammarusApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Настройка стиля
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(24.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(14.0, egui::FontFamily::Monospace)),
            (egui::TextStyle::Button, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
        ].into();
        cc.egui_ctx.set_style(style);
        
        // Подключение к базе данных
        let conn = Connection::open("eucarinogammarus.db").expect("Не удалось подключиться к базе данных");
        
        // Создание таблицы, если она не существует
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Eucarinogammarus (
                id INTEGER PRIMARY KEY,
                Код TEXT,
                Род TEXT,
                Вид TEXT,
                Размеры_мм TEXT,
                Тело TEXT,
                Окраска TEXT,
                Распространение TEXT,
                Глубина_м TEXT,
                Вооруж_тела TEXT,
                Средний_ряд_I_VII TEXT,
                Средн_ряд_VIII_X TEXT,
                Сред_ряд_урозом TEXT,
                Боковой_ряд TEXT,
                Краевой_ряд TEXT,
                Особен_воор TEXT,
                Эпимир_пласт TEXT,
                Верх_антенны TEXT,
                Прид_жгутик TEXT,
                Нижн_антенны TEXT,
                Базип_III_V TEXT,
                Уроподы_III TEXT,
                Головн_сегм TEXT,
                Глаза TEXT,
                Тельсон TEXT
            )",
            [],
        ).expect("Не удалось создать таблицу");
        
        // Импорт данных из CSV, если таблица пуста
        let count: i64 = {
            let mut stmt = conn.prepare("SELECT COUNT(*) FROM Eucarinogammarus").expect("Ошибка запроса");
            stmt.query_row([], |row| row.get(0)).expect("Ошибка получения количества записей")
        };
        
        if count == 0 {
            import_csv(&conn, "Eucarinogammarus.csv").expect("Ошибка импорта данных");
        }
        
        // Загрузка записей
        let records = load_records(&conn).unwrap_or_default();
        
        // Создание экземпляра приложения
        let conn = Arc::new(Mutex::new(conn));
        
        Self {
            conn,
            records,
            selected_tab: Tab::View,
            search_term: String::new(),
            sort_column: "id".to_string(),
            new_record: NewRecord::default(),
            edit_id: String::new(),
            edit_column: String::new(),
            edit_value: String::new(),
            delete_id: String::new(),
            status_message: String::new(),
        }
    }
    
    pub fn refresh_records(&mut self) {
        if let Ok(conn) = self.conn.lock() {
            if let Ok(records) = load_records(&conn) {
                self.records = records;
            }
        }
    }
    
    pub fn add_record(&mut self) -> Result<(), Box<dyn Error>> {
        // Сначала получаем данные из полей
        let code = self.new_record.code.clone();
        let genus = self.new_record.genus.clone();
        let species = self.new_record.species.clone();
        let size_mm = self.new_record.size_mm.clone();
        let body = self.new_record.body.clone();
        
        // Затем выполняем операцию с базой данных
        if let Ok(conn) = self.conn.lock() {
            conn.execute(
                "INSERT INTO Eucarinogammarus (
                    Код, Род, Вид, Размеры_мм, Тело
                ) VALUES (
                    ?1, ?2, ?3, ?4, ?5
                )",
                params![code, genus, species, size_mm, body],
            )?;
        }
        
        // Очистка полей после добавления
        self.new_record = NewRecord::default();
        
        // Обновление статуса и записей
        self.status_message = "Запись успешно добавлена".to_string();
        self.refresh_records();
        
        Ok(())
    }
    
    pub fn edit_record(&mut self) -> Result<(), Box<dyn Error>> {
        // Проверка валидности столбца
        let valid_columns = vec![
            "Код", "Род", "Вид", "Размеры_мм", "Тело", "Окраска", "Распространение",
            "Глубина_м", "Вооруж_тела", "Средний_ряд_I_VII", "Средн_ряд_VIII_X",
            "Сред_ряд_урозом", "Боковой_ряд", "Краевой_ряд", "Особен_воор",
            "Эпимир_пласт", "Верх_антенны", "Прид_жгутик", "Нижн_антенны",
            "Базип_III_V", "Уроподы_III", "Головн_сегм", "Глаза", "Тельсон"
        ];
        
        if !valid_columns.contains(&self.edit_column.as_str()) {
            self.status_message = "Неверное имя столбца".to_string();
            return Ok(());
        }
        
        let id = self.edit_id.parse::<i32>().unwrap_or(0);
        if id <= 0 {
            self.status_message = "Неверный ID".to_string();
            return Ok(());
        }
        
        // Сначала получаем данные из полей
        let column = self.edit_column.clone();
        let value = self.edit_value.clone();
        
        // Затем выполняем операцию с базой данных
        if let Ok(conn) = self.conn.lock() {
            let query = format!(
                "UPDATE Eucarinogammarus SET {} = ?1 WHERE id = ?2",
                column
            );
            
            conn.execute(&query, params![value, id])?;
        }
        
        // Обновление статуса и записей
        self.status_message = "Запись успешно обновлена".to_string();
        self.refresh_records();
        
        Ok(())
    }
    
    pub fn delete_record(&mut self) -> Result<(), Box<dyn Error>> {
        let id = self.delete_id.parse::<i32>().unwrap_or(0);
        if id <= 0 {
            self.status_message = "Неверный ID".to_string();
            return Ok(());
        }
        
        // Сначала получаем данные из полей
        let id_value = id;
        
        // Затем выполняем операцию с базой данных
        if let Ok(conn) = self.conn.lock() {
            conn.execute("DELETE FROM Eucarinogammarus WHERE id = ?1", params![id_value])?;
        }
        
        // Обновление статуса и записей
        self.status_message = "Запись успешно удалена".to_string();
        self.refresh_records();
        
        Ok(())
    }
    
    pub fn filtered_records(&self) -> Vec<&Eucarinogammarus> {
        self.records.iter()
            .filter(|r| {
                let search = self.search_term.to_lowercase();
                self.search_term.is_empty() ||
                r.code.to_lowercase().contains(&search) ||
                r.genus.to_lowercase().contains(&search) ||
                r.species.to_lowercase().contains(&search) ||
                r.size_mm.to_lowercase().contains(&search) ||
                r.body.to_lowercase().contains(&search) ||
                r.coloration.to_lowercase().contains(&search) ||
                r.distribution.to_lowercase().contains(&search) ||
                r.depth_m.to_lowercase().contains(&search) ||
                r.body_armament.to_lowercase().contains(&search) ||
                r.median_row_i_vii.to_lowercase().contains(&search) ||
                r.median_row_viii_x.to_lowercase().contains(&search) ||
                r.median_row_urozom.to_lowercase().contains(&search) ||
                r.lateral_row.to_lowercase().contains(&search) ||
                r.marginal_row.to_lowercase().contains(&search) ||
                r.special_armament.to_lowercase().contains(&search) ||
                r.epimeral_plate.to_lowercase().contains(&search) ||
                r.upper_antennae.to_lowercase().contains(&search) ||
                r.accessory_flagellum.to_lowercase().contains(&search) ||
                r.lower_antennae.to_lowercase().contains(&search) ||
                r.basipodite_iii_v.to_lowercase().contains(&search) ||
                r.uropods_iii.to_lowercase().contains(&search) ||
                r.head_segment.to_lowercase().contains(&search) ||
                r.eyes.to_lowercase().contains(&search) ||
                r.telson.to_lowercase().contains(&search)
            })
            .collect()
    }
}

impl eframe::App for EucarinogammarusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("База данных Eucarinogammarus");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Обновить").clicked() {
                        self.refresh_records();
                        self.status_message = "Данные обновлены".to_string();
                    }
                });
            });
            
            ui.horizontal(|ui| {
                if ui.selectable_label(self.selected_tab == Tab::View, "Просмотр").clicked() {
                    self.selected_tab = Tab::View;
                }
                if ui.selectable_label(self.selected_tab == Tab::Add, "Добавить").clicked() {
                    self.selected_tab = Tab::Add;
                }
                if ui.selectable_label(self.selected_tab == Tab::Edit, "Редактировать").clicked() {
                    self.selected_tab = Tab::Edit;
                }
                if ui.selectable_label(self.selected_tab == Tab::Delete, "Удалить").clicked() {
                    self.selected_tab = Tab::Delete;
                }
            });
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.selected_tab {
                Tab::View => view_tab::render(ui, self),
                Tab::Add => add_tab::render(ui, self),
                Tab::Edit => edit_tab::render(ui, self),
                Tab::Delete => delete_tab::render(ui, self),
            }
        });
        
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status_message);
            });
        });
    }
}