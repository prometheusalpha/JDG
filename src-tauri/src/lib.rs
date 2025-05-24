mod diagram_generator;
mod java_parser;
mod parsers;
mod commands;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info) // Set minimum log level to Info to disable Trace
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::add_new_project,
            commands::get_projects,
            commands::read_file_structure,
            commands::generate_mermaid_class_diagram,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
