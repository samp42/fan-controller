// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_serial_ports() -> Vec<String> {
    serialport::available_ports()
        .unwrap_or(vec![])
        .iter()
        .map(|port| port.clone().port_name)
        .collect()
}

fn main() {
    let openFile = CustomMenuItem::new("open_file".to_string(), "Open File");
    // let saveFile: CustomMenuItem::new
    let close = CustomMenuItem::new("close".to_string(), "Close Window");
    let fileSubmenu = Submenu::new("File", Menu::new().add_item(openFile).add_item(close));

    // let viewSubmenu = Submenu::new("View");
    // let windowSubmenu = Submenu::new("Window");
    
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(fileSubmenu);
    //     .add_submenu(viewSubmenu)
    //     .add_submenu(windowSubmenu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, list_serial_ports])
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
              "quit" => {
                std::process::exit(0);
              }
              "close" => {
                event.window().close().unwrap();
              }
              _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
