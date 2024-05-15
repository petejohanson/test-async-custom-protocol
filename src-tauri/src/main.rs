// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    ipc::{Request, Response, InvokeBody},
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(req: Request<'_>) -> Result<Response, ()> {
    if let InvokeBody::Raw(bytes) = req.body() {
        println!("OH YEAH!");
        Ok(Response::new(format!("Hello, {}! You've been greeted from Rust!", std::str::from_utf8(bytes).unwrap_or_default()).as_bytes().to_vec()))
    } else {
        Ok(Response::new(vec![]))
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
