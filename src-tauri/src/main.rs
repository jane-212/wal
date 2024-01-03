// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::porn;
use crate::app::state::AppState;
use crate::error::IResult;

mod app;
mod error;

fn main() -> IResult<()> {
    tauri::Builder::default()
        .manage(AppState::new()?)
        .invoke_handler(tauri::generate_handler![porn::porn_hot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
