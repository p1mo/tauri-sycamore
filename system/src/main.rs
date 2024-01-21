#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;
use tauri::Manager;
use tauri::PhysicalPosition;

use serde::{ Serialize, Deserialize };





#[derive(Serialize, Deserialize)]
struct GreetEmit {
    msg: String,
}




#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}




fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {

            let window = app.get_window("main").unwrap();

            #[cfg(debug_assertions)]{
                set_monitor_2_center_top(&window);
                window.open_devtools();
                //window.close_devtools();
            }


            let handle = app.app_handle().clone();

            window.listen("testevent", move |event: tauri::Event| {

                let payload = event.payload();

                let message = format!("rust: {}", payload);

                println!("testevent: {}", message);
          
                handle.emit("testrevent", message).unwrap();

            });

            //window.unlisten(handler);

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::Resized(_) => {

            },
            tauri::WindowEvent::Moved(_) => {

            },
            tauri::WindowEvent::CloseRequested { api , .. } => {

                api.prevent_close();

                println!("CloseRequested...");

                window.app_handle().exit(0);

            },
            tauri::WindowEvent::Destroyed => {

            },
            tauri::WindowEvent::Focused(_) => {

            },
            tauri::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size , .. } => {

                println!("ScaleFactorChanged: {} w:{} h:{}", scale_factor, new_inner_size.width, new_inner_size.height);

            },
            tauri::WindowEvent::FileDrop(_) => {

            },
            tauri::WindowEvent::ThemeChanged(_) => {

            },
            _ => {

            },
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


fn set_monitor_2_center_top(window : &Window) {
    if let Ok(monitors) = window.available_monitors() {
        window.set_position(*monitors[1].position()).unwrap();
        window.center().unwrap();
        let cx = window.inner_position().unwrap().x;
        let mpos = PhysicalPosition{ y : 3, x: cx };
        window.set_position(mpos).unwrap();
    }
}