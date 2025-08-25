#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowBuilder, PhysicalSize};
use tauri_plugin_single_instance::init as single_instance;

fn main() {
  tauri::Builder::default()
    .plugin(single_instance(|app, _args, _cwd| {
      if let Some(win) = app.get_window("main") { let _ = win.set_focus(); }
    }))
    .setup(|app| {
      let window = WindowBuilder::new(
          app, "main", tauri::WindowUrl::App("ui/index.html".into())
        )
        .title("HeatSignal")
        .inner_size(1280.0, 800.0)
        .resizable(false)
        .fullscreen(false)
        .maximizable(false)
        .minimizable(true)
        .decorations(false)
        .visible(true)
        .build()?;

      let _ = window.set_size(tauri::Size::Physical(PhysicalSize::new(1280, 800)));
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running HeatSignal");
}
