use rusqlite::{params, Connection, Result};
use std::error::Error;
use crate::db::{Eucarinogammarus, import_csv, read_input};

pub fn run_console_app() -> Result<(), Box<dyn Error>> {
    // Connect to SQLite database (it will create the database file if it doesn't exist)
    let conn = Connection::open("eucarinogammarus.db")?;

    // Create table if it doesn't exist
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

    // Import data from CSV
    import_csv(&conn, "Eucarinogammarus.csv")?;

    // Command-line interface
    loop {
        println!("\nВыберите опцию:");
        println!("1. Просмотр данных");
        println!("2. Добавить запись");
        println!("3. Редактировать запись");
        println!("4. Удалить запись");
        println!("5. Выход");

        let choice = read_input("Введите ваш выбор: ");

        match choice.trim() {
            "1" => view_data(&conn)?,
            "2" => add_record(&conn)?,
            "3" => edit_record(&conn)?,
            "4" => delete_record(&conn)?,
            "5" => {
                println!("Выход...");
                break;
            }
            _ => println!("Неверный выбор. Пожалуйста, попробуйте снова."),
        }
    }

    Ok(())
}

fn view_data(conn: &Connection) -> Result<(), Box<dyn Error>> {
    println!("Введите имя столбца для сортировки (например, `Код`, `Род` и т.д.):");
    let column = read_input("Столбец: ");

    // Validate column name to prevent SQL injection
    let valid_columns = vec![
        "Код", "Род", "Вид", "Размеры_мм", "Тело", "Окраска", "Распространение",
        "Глубина_м", "Вооруж_тела", "Средний_ряд_I_VII", "Средн_ряд_VIII_X",
        "Сред_ряд_урозом", "Боковой_ряд", "Краевой_ряд", "Особен_воор",
        "Эпимир_пласт", "Верх_антенны", "Прид_жгутик", "Нижн_антенны",
        "Базип_III_V", "Уроподы_III", "Головн_сегм", "Глаза", "Тельсон"
    ];

    if !valid_columns.contains(&column.trim()) {
        println!("Неверное имя столбца. Пожалуйста, попробуйте снова.");
        return Ok(());
    }

    let query = format!(
        "SELECT id, Код, Род, Вид, Размеры_мм, Тело, Окраска, Распространение, 
                Глубина_м, Вооруж_тела, Средний_ряд_I_VII, Средн_ряд_VIII_X, Сред_ряд_урозом, 
                Боковой_ряд, Краевой_ряд, Особен_воор, Эпимир_пласт, Верх_антенны, 
                Прид_жгутик, Нижн_антенны, Базип_III_V, Уроподы_III, Головн_сегм, 
                Глаза, Тельсон 
          FROM Eucarinogammarus ORDER BY {}",
        column.trim()
    );

    let mut stmt = conn.prepare(&query)?;
    let records = stmt.query_map([], |row| {
        Ok(Eucarinogammarus {
            id: row.get(0)?,
            code: row.get(1)?,
            genus: row.get(2)?,
            species: row.get(3)?,
            size_mm: row.get(4)?,
            body: row.get(5)?,
            coloration: row.get(6)?,
            distribution: row.get(7)?,
            depth_m: row.get(8)?,
            body_armament: row.get(9)?,
            median_row_i_vii: row.get(10)?,
            median_row_viii_x: row.get(11)?,
            median_row_urozom: row.get(12)?,
            lateral_row: row.get(13)?,
            marginal_row: row.get(14)?,
            special_armament: row.get(15)?,
            epimeral_plate: row.get(16)?,
            upper_antennae: row.get(17)?,
            accessory_flagellum: row.get(18)?,
            lower_antennae: row.get(19)?,
            basipodite_iii_v: row.get(20)?,
            uropods_iii: row.get(21)?,
            head_segment: row.get(22)?,
            eyes: row.get(23)?,
            telson: row.get(24)?,
        })
    })?;

    println!("\nID | Код | Род | Вид | Размеры мм | Тело | Окраска | Распространение | Глубина м | Вооруж тела | Средний ряд I-VII | Средн ряд VIII-X | Сред ряд урозом | Боковой ряд | Краевой ряд | Особен воор | Эпимир пласт | Верх антенны | Прид жгутик | Нижн антенны | Базип III-V | Уроподы III | Головн сегм | Глаза | Тельсон");
    println!("-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------");
    for record in records {
        let r = record?;
        println!(
            "{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {}",
            r.id, r.code, r.genus, r.species, r.size_mm, r.body, r.coloration, r.distribution,
            r.depth_m, r.body_armament, r.median_row_i_vii, r.median_row_viii_x,
            r.median_row_urozom, r.lateral_row, r.marginal_row, r.special_armament,
            r.epimeral_plate, r.upper_antennae, r.accessory_flagellum, r.lower_antennae,
            r.basipodite_iii_v, r.uropods_iii, r.head_segment, r.eyes, r.telson
        );
    }

    Ok(())
}

fn add_record(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let code = read_input("Введите Код: ");
    let genus = read_input("Введите Род: ");
    let species = read_input("Введите Вид: ");
    let size_mm = read_input("Введите Размеры мм: ");
    let body = read_input("Введите Тело: ");
    let coloration = read_input("Введите Окраска: ");
    let distribution = read_input("Введите Распространение: ");
    let depth_m = read_input("Введите Глубина м: ");
    let body_armament = read_input("Введите Вооруж тела: ");
    let median_row_i_vii = read_input("Введите Средний ряд I-VII: ");
    let median_row_viii_x = read_input("Введите Средн ряд VIII-X: ");
    let median_row_urozom = read_input("Введите Сред ряд урозом: ");
    let lateral_row = read_input("Введите Боковой ряд: ");
    let marginal_row = read_input("Введите Краевой ряд: ");
    let special_armament = read_input("Введите Особен воор: ");
    let epimeral_plate = read_input("Введите Эпимир пласт: ");
    let upper_antennae = read_input("Введите Верх антенны: ");
    let accessory_flagellum = read_input("Введите Прид жгутик: ");
    let lower_antennae = read_input("Введите Нижн антенны: ");
    let basipodite_iii_v = read_input("Введите Базип III-V: ");
    let uropods_iii = read_input("Введите Уроподы III: ");
    let head_segment = read_input("Введите Головн сегм: ");
    let eyes = read_input("Введите Глаза: ");
    let telson = read_input("Введите Тельсон: ");

    conn.execute(
        "INSERT INTO Eucarinogammarus (
            Код, Род, Вид, Размеры_мм, Тело, Окраска, Распространение, 
            Глубина_м, Вооруж_тела, Средний_ряд_I_VII, Средн_ряд_VIII_X, Сред_ряд_урозом, 
            Боковой_ряд, Краевой_ряд, Особен_воор, Эпимир_пласт, Верх_антенны, 
            Прид_жгутик, Нижн_антенны, Базип_III_V, Уроподы_III, Головн_сегм, 
            Глаза, Тельсон
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, 
            ?8, ?9, ?10, ?11, ?12, 
            ?13, ?14, ?15, ?16, ?17, 
            ?18, ?19, ?20, ?21, ?22, 
            ?23, ?24
        )",
        params![
            code.trim(),
            genus.trim(),
            species.trim(),
            size_mm.trim(),
            body.trim(),
            coloration.trim(),
            distribution.trim(),
            depth_m.trim(),
            body_armament.trim(),
            median_row_i_vii.trim(),
            median_row_viii_x.trim(),
            median_row_urozom.trim(),
            lateral_row.trim(),
            marginal_row.trim(),
            special_armament.trim(),
            epimeral_plate.trim(),
            upper_antennae.trim(),
            accessory_flagellum.trim(),
            lower_antennae.trim(),
            basipodite_iii_v.trim(),
            uropods_iii.trim(),
            head_segment.trim(),
            eyes.trim(),
            telson.trim(),
        ],
    )?;

    println!("Запись успешно добавлена.");
    Ok(())
}

fn edit_record(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let id = read_input("Введите ID записи для редактирования: ");
    println!("Доступные столбцы для редактирования:");
    println!("Код, Род, Вид, Размеры_мм, Тело, Окраска, Распространение, Глубина_м, Вооруж_тела, Средний_ряд_I_VII, Средн_ряд_VIII_X, Сред_ряд_урозом, Боковой_ряд, Краевой_ряд, Особен_воор, Эпимир_пласт, Верх_антенны, Прид_жгутик, Нижн_антенны, Базип_III_V, Уроподы_III, Головн_сегм, Глаза, Тельсон");
    let column = read_input("Введите столбец для редактирования: ");

    // Validate column name to prevent SQL injection
    let valid_columns = vec![
        "Код", "Род", "Вид", "Размеры_мм", "Тело", "Окраска", "Распространение",
        "Глубина_м", "Вооруж_тела", "Средний_ряд_I_VII", "Средн_ряд_VIII_X",
        "Сред_ряд_урозом", "Боковой_ряд", "Краевой_ряд", "Особен_воор",
        "Эпимир_пласт", "Верх_антенны", "Прид_жгутик", "Нижн_антенны",
        "Базип_III_V", "Уроподы_III", "Головн_сегм", "Глаза", "Тельсон"
    ];

    if !valid_columns.contains(&column.trim()) {
        println!("Неверное имя столбца. Пожалуйста, попробуйте снова.");
        return Ok(());
    }

    let new_value = read_input("Введите новое значение: ");

    let query = format!(
        "UPDATE Eucarinogammarus SET {} = ?1 WHERE id = ?2",
        column.trim()
    );

    conn.execute(&query, params![new_value.trim(), id.trim()])?;

    println!("Запись успешно обновлена.");
    Ok(())
}

fn delete_record(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let id = read_input("Введите ID записи для удаления: ");

    conn.execute("DELETE FROM Eucarinogammarus WHERE id = ?1", params![id.trim()])?;

    println!("Запись успешно удалена.");
    Ok(())
}