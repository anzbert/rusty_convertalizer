#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    // let native_options = eframe::NativeOptions::default();
    let native_options = eframe::NativeOptions {
        drag_and_drop_support: true,
        // initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "rusty_convertalizer",
        native_options,
        Box::new(|cc| Box::new(rusty_convertalizer::App::new(cc))),
    );
}
