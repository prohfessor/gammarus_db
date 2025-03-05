use rusqlite::{params, Connection, Result};
use std::error::Error;
use crate::app::SortDirection;
use std::fs::File;
use csv::Reader;

// Структура для хранения данных
#[derive(Debug, Clone)]
pub struct Eucarinogammarus {
    pub id: i32,
    pub code: String,
    pub genus: String,
    pub species: String,
    pub size_mm: String,
    pub body: String,
    pub coloration: String,
    pub distribution: String,
    pub depth_m: String,
    pub body_armament: String,
    pub median_row_i_vii: String,
    pub median_row_viii_x: String,
    pub median_row_urozom: String,
    pub lateral_row: String,
    pub marginal_row: String,
    pub special_armament: String,
    pub epimeral_plate: String,
    pub upper_antennae: String,
    pub accessory_flagellum: String,
    pub lower_antennae: String,
    pub basipodite_iii_v: String,
    pub uropods_iii: String,
    pub head_segment: String,
    pub eyes: String,
    pub telson: String,
}

// Функция для импорта данных из CSV
pub fn import_csv(conn: &Connection, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    // Skip the header row
    for result in rdr.records().skip(1) {
        let record = result?;
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
                record.get(0).unwrap_or(""),
                record.get(1).unwrap_or(""),
                record.get(2).unwrap_or(""),
                record.get(3).unwrap_or(""),
                record.get(4).unwrap_or(""),
                record.get(5).unwrap_or(""),
                record.get(6).unwrap_or(""),
                record.get(7).unwrap_or(""),
                record.get(8).unwrap_or(""),
                record.get(9).unwrap_or(""),
                record.get(10).unwrap_or(""),
                record.get(11).unwrap_or(""),
                record.get(12).unwrap_or(""),
                record.get(13).unwrap_or(""),
                record.get(14).unwrap_or(""),
                record.get(15).unwrap_or(""),
                record.get(16).unwrap_or(""),
                record.get(17).unwrap_or(""),
                record.get(18).unwrap_or(""),
                record.get(19).unwrap_or(""),
                record.get(20).unwrap_or(""),
                record.get(21).unwrap_or(""),
                record.get(22).unwrap_or(""),
                record.get(23).unwrap_or(""),
            ],
        )?;
    }

    println!("Data imported successfully.");
    Ok(())
}

// Функция для загрузки записей из базы данных
pub fn load_records(conn: &Connection) -> Result<Vec<Eucarinogammarus>, Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT id, Код, Род, Вид, Размеры_мм, Тело, Окраска, Распространение, 
                                        Глубина_м, Вооруж_тела, Средний_ряд_I_VII, Средн_ряд_VIII_X, Сред_ряд_урозом, 
                                        Боковой_ряд, Краевой_ряд, Особен_воор, Эпимир_пласт, Верх_антенны, 
                                        Прид_жгутик, Нижн_антенны, Базип_III_V, Уроподы_III, Головн_сегм, 
                                        Глаза, Тельсон 
                                 FROM Eucarinogammarus")?;
    
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
    
    let mut result = Vec::new();
    for record in records {
        result.push(record?);
    }
    
    Ok(result)
}

// Функция для загрузки записей из базы данных с сортировкой
pub fn load_records_sorted(conn: &Connection, sort_column: &str, direction: SortDirection) -> Result<Vec<Eucarinogammarus>, Box<dyn Error>> {
    let direction_str = match direction {
        SortDirection::Ascending => "ASC",
        SortDirection::Descending => "DESC",
    };
    
    let query = format!(
        "SELECT id, Код, Род, Вид, Размеры_мм, Тело, Окраска, Распространение, 
                Глубина_м, Вооруж_тела, Средний_ряд_I_VII, Средн_ряд_VIII_X, Сред_ряд_урозом, 
                Боковой_ряд, Краевой_ряд, Особен_воор, Эпимир_пласт, Верх_антенны, 
                Прид_жгутик, Нижн_антенны, Базип_III_V, Уроподы_III, Головн_сегм, 
                Глаза, Тельсон 
         FROM Eucarinogammarus ORDER BY {} {}",
        sort_column
,
        direction_str
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
    
    let mut result = Vec::new();
    for record in records {
        result.push(record?);
    }
    
    Ok(result)
}