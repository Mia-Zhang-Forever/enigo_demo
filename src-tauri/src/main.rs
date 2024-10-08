// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate core;
use enigo::{Direction, Enigo, Key, Keyboard};
use get_selected_text::get_selected_text;

use std::{
    sync::{Arc, Mutex},
    thread,
};
use tauri::{AppHandle, CustomMenuItem, GlobalShortcutManager};
use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowEvent};
use tokio::runtime::Runtime;

fn make_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "quit");
    let open_board = CustomMenuItem::new("open_board".to_string(), "board");

    let tray_menu = SystemTrayMenu::new().add_item(open_board).add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

fn main() {
    let rt = Arc::new(Runtime::new().unwrap());
    let rt_clone = Arc::clone(&rt);

    let app_handle: Arc<Mutex<Option<AppHandle>>> = Arc::new(Mutex::new(None));
    let app_handle_clone = app_handle.clone();

    tauri::Builder::default()
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            {
                use tauri::ActivationPolicy;
                app.set_activation_policy(ActivationPolicy::Accessory);
            }
            {
                app_handle.lock().unwrap().replace(app.handle().clone());
            }

            let app_handle = app.handle();
            app.global_shortcut_manager()
                .register("Command+ctrl+c", move || {
                    if let Some(window) = app_handle.get_window("board") {
                        window.set_always_on_top(true).unwrap();
                        window.set_focus().unwrap();
                        window.show().unwrap();
                    }
                })
                .unwrap();

            app.global_shortcut_manager()
                .register("Command+ctrl+,", move || {
                    get_context(app_handle_clone.clone());
                })
                .unwrap();

            Ok(())
        })
        .system_tray(make_tray())
        .on_system_tray_event({
            move |app, event| {
                if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                    match id.as_str() {
                        "quit" => std::process::exit(0),

                        "open_board" => {
                            let window = app.get_window("board").unwrap();
                            window.set_always_on_top(true).unwrap();
                            window.show().unwrap();
                        }
                        _ => {}
                    }
                }
            }
        })
        .on_window_event(|event| {
            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_selection() -> String {
    let mut selected_text = String::new();
    match get_selected_text() {
        Ok(text) => {
            selected_text = text;
        }
        Err(_) => {
            println!("error occurred while getting the selected text");
        }
    }

    selected_text
}

fn get_context(app_handle: Arc<Mutex<Option<AppHandle>>>) {
    println!("preparing to copy text...");

    let user_prompt = handle_selection();
    println!("----------> finished getting user_prompt");
    println!("copied... {}", user_prompt);
}
