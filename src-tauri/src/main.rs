#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{WindowBuilder, WindowUrl};

fn main() {
  let ctx = tauri::generate_context!();
  tauri::Builder::default()
    .create_window("main".into(), WindowUrl::default(), |win, webview| {
      let win = win
        .title("sanic")
        .resizable(true)
        .transparent(false)
        .decorations(true)
        .always_on_top(false)
        .inner_size(1000.0, 800.0)
        .min_inner_size(300.0, 200.0)
        .fullscreen(false);
      return (win, webview);
    })
    .run(ctx)
    .expect("error running application");
}
