use native_windows_gui as nwg;
use native_windows_derive as nwd;
use nwd::NwgUi;
use nwg::{NativeUi, ControlHandle};
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::cell::RefCell;
use std::rc::Rc;
use crate::db::{Eucarinogammarus, import_csv, load_records, load_records_sorted};

pub fn run_gui_app() -> Result<(), Box<dyn Error>> {
    // Инициализация библиотеки native-windows-gui
    nwg::init().expect("Не удалось инициализировать GUI");
    
    // Создание соединения с базой данных
    let conn = Connection::open("eucarinogammarus.db")?;
    
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
    )?;
    
    // Импорт данных из CSV, если таблица пуста
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM Eucarinogammarus")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    
    if count == 0 {
        import_csv(&conn, "Eucarinogammarus.csv")?;
    }
    
    // Загрузка записей
    let records = load_records(&conn)?;
    
    // Создание и запуск приложения
    let app = EucarinogammarusApp {
        conn: Rc::new(RefCell::new(conn)),
        records: Rc::new(RefCell::new(records)),
        window: Default::default(),
        tabs: Default::default(),
        
        // Вкладка просмотра
        search_input: Default::default(),
        sort_button: Default::default(),
        refresh_button: Default::default(),
        records_list: Default::default(),
        
        // Вкладка добавления
        code_input: Default::default(),
        genus_input: Default::default(),
        species_input: Default::default(),
        size_mm_input: Default::default(),
        body_input: Default::default(),
        add_button: Default::default(),
        
        // Вкладка редактирования
        edit_id_input: Default::default(),
        edit_column_input: Default::default(),
        edit_value_input: Default::default(),
        edit_button: Default::default(),
        
        // Вкладка удаления
        delete_id_input: Default::default(),
        delete_button: Default::default(),
        
        // Статусная строка
        status_label: Default::default(),
    };
    
    let _app = EucarinogammarusApp::build_ui(app).expect("Не удалось построить UI");
    
    // Запуск цикла обработки сообщений
    nwg::dispatch_thread_events();
    
    Ok(())
}

#[derive(Default, NwgUi)]
pub struct EucarinogammarusApp {
    // Данные
    #[nwg_control(skip)]
    conn: Rc<RefCell<Connection>>,
    
    #[nwg_control(skip)]
    records: Rc<RefCell<Vec<Eucarinogammarus>>>,
    
    // Главное окно
    #[nwg_control(size: (800, 600), position: (300, 300), title: "База данных Eucarinogammarus")]
    window: nwg::Window,
    
    // Вкладки
    #[nwg_control(parent: window, position: (0, 0), size: (800, 550))]
    #[nwg_events(OnNotice: [EucarinogammarusApp::on_tab_change])]
    tabs: nwg::TabsContainer,
    
    // Вкладка просмотра
    #[nwg_control(parent: tabs, text: "Поиск:", position: (10, 10), size: (50, 25))]
    search_label: nwg::Label,
    
    #[nwg_control(parent: tabs, position: (70, 10), size: (200, 25))]
    search_input: nwg::TextInput,
    
    #[nwg_control(parent: tabs, text: "Сортировать по:", position: (280, 10), size: (100, 25))]
    sort_label: nwg::Label,
    
    #[nwg_control(parent: tabs, text: "ID", position: (390, 10), size: (100, 25))]
    #[nwg_events(OnButtonClick: [EucarinogammarusApp::on_sort_click])]
    sort_button: nwg::Button,
    
    #[nwg_control(parent: tabs, text: "Обновить", position: (500, 10), size: (100, 25))]
    #[nwg_events(OnButtonClick: [EucarinogammarusApp::on_refresh_click])]
    refresh_button: nwg::Button,
    
    #[nwg_control(parent: tabs, position: (10, 50), size: (780, 450), list_style: nwg::ListViewStyle::Detailed)]
    #[nwg_events(OnListViewClick: [EucarinogammarusApp::on_record_select])]
    records_list: nwg::ListView,
    
    // Вкладка добавления
    #[nwg_control(parent: tabs, text: "Код:", position: (10, 10), size: (100, 25))]
    code_label: nwg::Label,
    
    #[nwg_control(parent: tabs, position: (120, 10), size: (200, 25))]
    code_input: nwg::TextInput,
    
    #[nwg_control(parent: tabs, text: "Род:", position: (10, 45), size: (100, 25))]
    genus_label: nwg::Label,
    
    #[nwg_control(parent: tabs, position: (120, 45), size: (200, 25))]
    genus_input: nwg::TextInput,
    
    #[nwg_control(parent: tabs, text: "Вид:", position: (10, 80), size: (100, 25))]
    species_label: nwg::Label,
    
    #[nwg_control(parent: tabs, position: (120, 80), size: (200, 25))]
    species_input: nwg::TextInput,
    
    #[nwg_control(parent: tabs, text: "Размеры мм:", position: (10, 115), size: (100, 25))]
    size_mm_label: nwg::Label,
    
    #[nwg_control(parent: tabs, position: (120, 115), size: (200, 25))]
    size_mm_input: nwg::TextInput,
    
    #[nwg_control(parent: tabs, text: "Тело:", position: (10, 150), size: (100, 25))]
    body_label: nwg::Label,
    
    #[nwg_control(parent: tabs, position: (120, 150), size: (200, 25))]
    body_input: nwg::TextInput,
    
    #[nwg_control(parent: tabs, text: "Добавить запись", position: (120, 200), size: (150, 30))]
    #[nwg_events(OnButtonClick: [EucarinogammarusApp::on_add_click])]
    add_button: nwg::Button,
    
    // Вкладка редактирования
    #[nwg_control(parent: tabs, text: "ID записи:", position: (10, 10), size: (100, 25))]
    edit_id_label: nwg::Label,
    
    #[nwg_control(parent: tabs, position: (120, 10), size: (200, 25))]
    edit_id_input: nwg::TextInput,
    
    #[nwg_control(parent: tabs, text: "Столбец:", position: (10, 45), size: (100, 25))]
    edit_column_label: nwg::Label,
    
    #[nwg_control(parent: tabs, position: (120, 45), size: (200, 25))]
    edit_column_input: nwg::TextInput,
    
    #[nwg_control(parent: tabs, text: "Новое значение:", position: (10, 80), size: (100, 25))]
    edit_value_label: nwg::Label,
    
    #[nwg_control(parent: tabs, position: (120, 80), size: (200, 25))]
    edit_value_input: nwg::TextInput,
    
    #[nwg_control(parent: tabs, text: "Обновить запись", position: (120, 130), size: (150, 30))]
    #[nwg_events(OnButtonClick: [EucarinogammarusApp::on_edit_click])]
    edit_button: nwg::Button,
    
    // Вкладка удаления
    #[nwg_control(parent: tabs, text: "ID записи для удаления:", position: (10, 10), size: (150, 25))]
    delete_id_label: nwg::Label,
    
    #[nwg_control(parent: tabs, position: (170, 10), size: (200, 25))]
    delete_id_input: nwg::TextInput,
    
    #[nwg_control(parent: tabs, text: "Удалить запись", position: (170, 50), size: (150, 30))]
    #[nwg_events(OnButtonClick: [EucarinogammarusApp::on_delete_click])]
    delete_button: nwg::Button,
    
    // Статусная строка
    #[nwg_control(parent: window, text: "", position: (10, 560), size: (780, 25))]
    status_label: nwg::Label,
}

impl EucarinogammarusApp {
    fn init(&self) {
        // Инициализация вкладок
        self.tabs.add_tab("Просмотр", Some(1));
        self.tabs.add_tab("Добавить", Some(2));
        self.tabs.add_tab("Редактировать", Some(3));
        self.tabs.add_tab("Удалить", Some(4));
        
        // Инициализация списка записей
        self.records_list.insert_column("ID", 50);
        self.records_list.insert_column("Код", 100);
        self.records_list.insert_column("Род", 150);
        self.records_list.insert_column("Вид", 150);
        self.records_list.insert_column("Размеры мм", 100);
        
        // Заполнение списка записей
        self.refresh_records_list();
    }
    
    fn refresh_records_list(&self) {
        // Очистка списка
        self.records_list.clear();
        
        // Получение записей
        let records = self.records.borrow();
        
        // Заполнение списка
        for record in records.iter() {
            let row = self.records_list.insert_item(record.id.to_string());
            self.records_list.set_item_text(row, 1, &record.code);
            self.records_list.set_item_text(row, 2, &record.genus);
            self.records_list.set_item_text(row, 3, &record.species);
            self.records_list.set_item_text(row, 4, &record.size_mm);
        }
    }
    
    fn on_tab_change(&self) {
        // Обработка изменения вкладки
        let tab_index = self.tabs.selected_tab();
        
        // Обновление списка записей при переходе на вкладку просмотра
        if tab_index == 0 {
            self.refresh_records_list();
        }
    }
    
    fn on_sort_click(&self) {
        // Получение текущего столбца сортировки
        let current_sort = self.sort_button.text();
        
        // Циклическое переключение столбцов сортировки
        let new_sort = match current_sort.as_str() {
            "ID" => "Код",
            "Код" => "Род",
            "Род" => "Вид",
            "Вид" => "ID",
            _ => "ID",
        };
        
        // Установка нового столбца сортировки
        self.sort_button.set_text(new_sort);
        
        // Обновление записей с учетом сортировки
        if let Ok(conn) = self.conn.try_borrow() {
            if let Ok(records) = load_records_sorted(&conn, new_sort) {
                *self.records.borrow_mut() = records;
                self.refresh_records_list();
            }
        }
    }
    
    fn on_refresh_click(&self) {
        // Обновление записей
        if let Ok(conn) = self.conn.try_borrow() {
            if let Ok(records) = load_records(&conn) {
                *self.records.borrow_mut() = records;
                self.refresh_records_list();
                self.status_label.set_text("Данные обновлены");
            }
        }
    }
    
    fn on_record_select(&self) {
        // Обработка выбора записи в списке
        let selected = self.records_list.selected_item();
        
        if selected >= 0 {
            // Получение ID выбранной записи
            let id = self.records_list.item_text(selected as usize, 0);
            
            // Установка ID в поля редактирования и удаления
            self.edit_id_input.set_text(&id);
            self.delete_id_input.set_text(&id);
        }
    }
    
    fn on_add_click(&self) {
        // Получение данных из полей ввода
        let code = self.code_input.text();
        let genus = self.genus_input.text();
        let species = self.species_input.text();
        let size_mm = self.size_mm_input.text();
        let body = self.body_input.text();
        
        // Добавление записи в базу данных
        if let Ok(conn) = self.conn.try_borrow() {
            let result = conn.execute(
                "INSERT INTO Eucarinogammarus (
                    Код, Род, Вид, Размеры_мм, Тело
                ) VALUES (
                    ?1, ?2, ?3, ?4, ?5
                )",
                params![code, genus, species, size_mm, body],
            );
            
            match result {
                Ok(_) => {
                    // Очистка полей ввода
                    self.code_input.set_text("");
                    self.genus_input.set_text("");
                    self.species_input.set_text("");
                    self.size_mm_input.set_text("");
                    self.body_input.set_text("");
                    
                    // Обновление списка записей
                    if let Ok(records) = load_records(&conn) {
                        *self.records.borrow_mut() = records;
                        self.refresh_records_list();
                    }
                    
                    self.status_label.set_text("Запись успешно добавлена");
                },
                Err(e) => {
                    self.status_label.set_text(&format!("Ошибка: {}", e));
                }
            }
        }
    }
    
    fn on_edit_click(&self) {
        // Получение данных из полей ввода
        let id = self.edit_id_input.text();
        let column = self.edit_column_input.text();
        let value = self.edit_value_input.text();
        
        // Проверка валидности столбца
        let valid_columns = vec![
            "Код", "Род", "Вид", "Размеры_мм", "Тело", "Окраска", "Распространение",
            "Глубина_м", "Вооруж_тела", "Средний_ряд_I_VII", "Средн_ряд_VIII_X",
            "Сред_ряд_урозом", "Боковой_ряд", "Краевой_ряд", "Особен_воор",
            "Эпимир_пласт", "Верх_антенны", "Прид_жгутик", "Нижн_антенны",
            "Базип_III_V", "Уроподы_III", "Головн_сегм", "Глаза", "Тельсон"
        ];
        
        if !valid_columns.contains(&column.as_str()) {
            self.status_label.set_text("Неверное имя столбца");
            return;
        }
        
        let id_num = id.parse::<i32>().unwrap_or(0);
        if id_num <= 0 {
            self.status_label.set_text("Неверный ID");
            return;
        }
        
        // Обновление записи в базе данных
        if let Ok(conn) = self.conn.try_borrow() {
            let query = format!(
                "UPDATE Eucarinogammarus SET {} = ?1 WHERE id = ?2",
                column
            );
            
            match conn.execute(&query, params![value, id_num]) {
                Ok(_) => {
                    // Обновление списка записей
                    if let Ok(records) = load_records(&conn) {
                        *self.records.borrow_mut() = records;
                        self.refresh_records_list();
                    }
                    
                    self.status_label.set_text("Запись успешно обновлена");
                },
                Err(e) => {
                    self.status_label.set_text(&format!("Ошибка: {}", e));
                }
            }
        }
    }
    
    fn on_delete_click(&self) {
        // Получение ID записи для удаления
        let id = self.delete_id_input.text();
        
        let id_num = id.parse::<i32>().unwrap_or(0);
        if id_num <= 0 {
            self.status_label.set_text("Неверный ID");
            return;
        }
        
        // Удаление записи из базы данных
        if let Ok(conn) = self.conn.try_borrow() {
            match conn.execute("DELETE FROM Eucarinogammarus WHERE id = ?1", params![id_num]) {
                Ok(_) => {
                    // Обновление списка записей
                    if let Ok(records) = load_records(&conn) {
                        *self.records.borrow_mut() = records;
                        self.refresh_records_list();
                    }
                    
                    self.status_label.set_text("Запись успешно удалена");
                },
                Err(e) => {
                    self.status_label.set_text(&format!("Ошибка: {}", e));
                }
            }
        }
    }
}