// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use ooxml::document::SpreadsheetDocument;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub is_done: bool,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn add_item(title: &str) -> Result<i64, String> {
    println!("add_item: {}", title);
    let conn = get_conn().await.map_err(|e| e.to_string())?;
    let id = sqlx::query("INSERT INTO todos (title,is_done) VALUES (?,?)")
        .bind(title)
        .bind(false)
        .execute(&conn)
        .await
        .map_err(|e| e.to_string())?
        .last_insert_rowid();
    println!("add_item: {}", id);
    Ok(id)
}

#[tauri::command]
async fn list() -> Result<Vec<Todo>, String> {
    let conn = get_conn().await.map_err(|e| e.to_string())?;
    let data = sqlx::query_as("SELECT * FROM todos ORDER BY id DESC")
        .fetch_all(&conn)
        .await
        .map_err(|e| e.to_string())?;
    Ok(data)
}

#[tauri::command]
async fn edit(item: Todo) -> Result<u64, String> {
    let conn = get_conn().await.map_err(|e| e.to_string())?;
    let aff = sqlx::query("UPDATE todos SET title=? WHERE id=?")
        .bind(&item.title)
        .bind(&item.id)
        .execute(&conn)
        .await
        .map_err(|e| e.to_string())?
        .rows_affected();
    Ok(aff)
}

#[tauri::command]
async fn del(id: i64) -> Result<u64, String> {
    let conn = get_conn().await.map_err(|e| e.to_string())?;
    let aff = sqlx::query("DELETE FROM todos WHERE id=?")
        .bind(id)
        .execute(&conn)
        .await
        .map_err(|e| e.to_string())?
        .rows_affected();
    Ok(aff)
}

#[tauri::command]
async fn check(id: i64) -> Result<u64, String> {
    let conn = get_conn().await.map_err(|e| e.to_string())?;
    let aff = sqlx::query("UPDATE todos SET is_done=(NOT is_done) WHERE id=?")
        .bind(id)
        .execute(&conn)
        .await
        .map_err(|e| e.to_string())?
        .rows_affected();
    Ok(aff)
}

#[tauri::command]
async fn del_done() -> Result<u64, String> {
    let conn = get_conn().await.map_err(|e| e.to_string())?;
    let aff = sqlx::query("DELETE FROM todos WHERE is_done=?")
        .bind(true)
        .execute(&conn)
        .await
        .map_err(|e| e.to_string())?
        .rows_affected();
    Ok(aff)
}

#[tauri::command]
async fn check_all(is_done: bool) -> Result<u64, String> {
    let conn = get_conn().await.map_err(|e| e.to_string())?;
    let aff = sqlx::query("UPDATE todos SET is_done=?")
        .bind(is_done)
        .execute(&conn)
        .await
        .map_err(|e| e.to_string())?
        .rows_affected();
    Ok(aff)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn get_value_by_index(index: i32) -> &'static str {
    match index {
        0 => "project_name",
        1 => "group_members",
        2 => "target",
        3 => "time",
        4 => "remark",
        _ => ""
    }
}

#[tauri::command]
fn read_excel(path: &str) -> HashMap<String, Vec<Vec<HashMap<&'static str, String>>>> {
    let xlsx =
        SpreadsheetDocument::open(path).unwrap();
    let workbook = xlsx.get_workbook();
    let sheet_names = workbook.worksheet_names();
    let mut result = HashMap::new();
    for name in sheet_names {
        let sheet_name = String::from(name);
        let sheet_data = Vec::new();
        result.insert(sheet_name.clone(), sheet_data);
        let sheet = workbook.get_worksheet_by_name(&sheet_name).unwrap();
        let mut rows_data = Vec::new();
        for rows in sheet.rows() {
            let mut row_data = HashMap::new();
            // get cell values
            let cols: Vec<_> = rows
                .into_iter()
                .map(|cell| cell.value().unwrap_or_default())
                .collect();
            for (index,cell) in cols.iter().enumerate() {
                let cell_data = cell.to_string();
                // TODO: 处理时间字段？？
                if get_value_by_index(index as i32) != "" {
                    row_data.insert(get_value_by_index(index as i32), cell_data);
                }
            }
            rows_data.push(row_data.clone());
        }
        result.get_mut(&sheet_name).map(|val| val.push(rows_data));
    }
    result
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // Initialize the database.
            db::init();

            Ok(())
        })
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            greet,add_item, list, edit, del, check, del_done, check_all,read_excel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn get_conn() -> std::result::Result<sqlx::SqlitePool, sqlx::Error> {
    let db_path = std::env::var("TODO_DB").unwrap_or("../todo.db".to_string());
    let db_url = format!("sqlite://{}", db_path);
    sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&db_url)
        .await
}
