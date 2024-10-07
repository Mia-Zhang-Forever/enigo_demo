// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate core;
use enigo::{Direction, Enigo, Key, Keyboard};
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
                .register("Command+ctrl+n", move || {
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

fn handle_selection() {
    let mut enigo = Enigo::new(&enigo::Settings::default()).unwrap();
    #[cfg(target_os = "macos")]
    {
        enigo.key(Key::Meta, Direction::Release).unwrap();
        // copy
        enigo.key(Key::Meta, Direction::Press).unwrap();
        // enigo.key(Key::Unicode('c'), Direction::Click).unwrap();
        enigo.raw(8, Direction::Click).unwrap();
        enigo.key(Key::Meta, Direction::Release).unwrap();
    }

    #[cfg(not(target_os = "macos"))]
    {
        // For Windows and Linux, use Ctrl key
        enigo.key(Key::LControl, Direction::Press).unwrap();
        enigo.key(Key::Unicode('c'), Direction::Click).unwrap();
        enigo.key(Key::LControl, Direction::Release).unwrap();
    }

    enigo.key(Key::Backspace, Direction::Click).unwrap();
}

fn get_context(app_handle: Arc<Mutex<Option<AppHandle>>>) {
    println!("preparing to copy text...");

    handle_selection();
    for _ in 0..3 {
        println!("waiting for 5 seconds");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("----------> finished handle_selection");
    let user_prompt = {
        let mut handle = app_handle.lock().unwrap();
        handle
            .as_mut()
            .unwrap()
            .clipboard_manager()
            .clipboard
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .get_text()
            .unwrap_or("".to_string())
    };

    println!("----------> finished getting user_prompt");
    println!("copied... {}", user_prompt);
}
