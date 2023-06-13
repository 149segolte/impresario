// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

type ConfigManager = Mutex<Option<Config>>;

struct Config {
    config: serde_json::Value,
    path: String,
}

trait ConfigManagerTrait {
    fn initialize(handle: tauri::AppHandle) -> Self;
    fn read_config(&mut self);
    fn save_config(&self);
    fn get_config(&self) -> serde_json::Value;
    fn set_config(&mut self, config: serde_json::Value);
    fn get_value(&self, key: &str) -> Option<serde_json::Value>;
    fn set_value(&mut self, key: &str, value: serde_json::Value);
}

impl ConfigManagerTrait for ConfigManager {
    fn initialize(handle: tauri::AppHandle) -> Self {
        let resource_path = handle
            .path_resolver()
            .resolve_resource("app_config.json")
            .expect("failed to resolve resource");

        let file = std::fs::File::open(&resource_path).unwrap();
        let value = serde_json::from_reader(file).unwrap();
        Mutex::new(Some(Config {
            config: value,
            path: resource_path.to_str().unwrap().to_string(),
        }))
    }

    fn read_config(&mut self) {
        let resource_path = self.lock().unwrap().as_ref().unwrap().path.clone();
        let file = std::fs::File::open(&resource_path).unwrap();
        let value = serde_json::from_reader(file).unwrap();
        self.lock().unwrap().as_mut().unwrap().config = value;
    }

    fn save_config(&self) {
        let resource_path = self.lock().unwrap().as_ref().unwrap().path.clone();
        let file = std::fs::File::open(&resource_path).unwrap();
        serde_json::to_writer(file, &self.lock().unwrap().as_ref().unwrap().config).unwrap();
    }

    fn get_config(&self) -> serde_json::Value {
        self.lock().unwrap().as_ref().unwrap().config.clone()
    }

    fn set_config(&mut self, config: serde_json::Value) {
        self.lock().unwrap().as_mut().unwrap().config = config;
    }

    fn get_value(&self, key: &str) -> Option<serde_json::Value> {
        self.get_config().get(key).cloned()
    }

    fn set_value(&mut self, key: &str, value: serde_json::Value) {
        self.get_config()
            .as_object_mut()
            .unwrap()
            .insert(key.to_string(), value);
    }
}

fn main() {
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let docs = CustomMenuItem::new("docs".to_string(), "Documentation");
    let bug = CustomMenuItem::new("bug".to_string(), "Report a bug");
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(docs)
        .add_item(bug)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(exit);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main");
                match window {
                    Some(w) => {
                        if w.is_visible()
                            .expect("error while getting window visibility")
                        {
                            w.hide().unwrap();
                        } else {
                            w.show().unwrap();
                        }
                    }
                    None => {}
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "settings" => {
                    let window = app.get_window("main");
                    match window {
                        Some(w) => {
                            if w.is_visible()
                                .expect("error while getting window visibility")
                            {
                                w.set_focus().unwrap();
                            } else {
                                w.show().unwrap();
                            }
                        }
                        None => {}
                    }
                }
                "exit" => {
                    app.exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(|app| {
            app.manage(ConfigManager::initialize(app.handle()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
