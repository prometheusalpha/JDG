use log::info;

mod diagram_generator;
mod java_parser;

use diagram_generator::generate_diagram;
use java_parser::parse_java_file;

#[tauri::command]
fn greet(name: &str) -> String {
    info!("Tauri is awesome!");
    println!("Tauri is awesome!");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FileNode {
    name: String,
    path: String,
    r#type: String,
    children: Vec<FileNode>,
}

// read file structure from a path
#[tauri::command]
fn read_file_structure(id: &str) -> FileNode {
    let config = parse_config();
    let mut path = "";
    for project in config.iter() {
        if project.id.to_string() == id {
            path = &project.path;
        }
    }
    if path == "" {
        return FileNode {
            name: "".to_string(),
            path: "".to_string(),
            r#type: "".to_string(),
            children: Vec::new(),
        };
    }
    return read_structure_from_path(path).unwrap();
}

fn read_structure_from_path(path: &str) -> Option<FileNode> {
    let path = std::path::Path::new(path);
    let mut root = FileNode {
        name: path.file_name().unwrap().to_str().unwrap().to_string(),
        path: path.to_str().unwrap().to_string(),
        r#type: if path.is_dir() { "folder" } else { "file" }.to_string(),
        children: Vec::new(),
    };
    if !path.is_dir() {
        if path.extension().unwrap_or_default() == "java" {
            return Some(root);
        } else {
            return None;
        }
    }
    for entry in path.read_dir().unwrap() {
        // ignore some common folders
        const IGNORE: [&str; 5] = ["node_modules", "target", "build", "dist", ".git"];
        let entry = entry.unwrap();
        if IGNORE.contains(&entry.file_name().to_str().unwrap()) {
            continue;
        }
        let node = read_structure_from_path(&entry.path().to_str().unwrap());
        if node.is_none() {
            continue;
        }
        root.children.push(node.unwrap());
    }
    if root.children.len() == 0 {
        return None;
    }
    return Some(root);
}

fn ensure_config_exists() -> String {
    let os = std::env::consts::OS;
    let mut home_dir = std::env::var("HOME").unwrap();
    if os == "windows" {
        home_dir = std::env::var("USERPROFILE").unwrap();
    } else if os == "macos" {
        home_dir = std::env::var("HOME").unwrap();
    } else if os == "linux" {
        home_dir = std::env::var("HOME").unwrap();
    }
    // create folder if it doesn't exist
    let mut folder_path = home_dir.clone();
    folder_path.push_str("/Documents/jdg");
    if !std::path::Path::new(&folder_path).exists() {
        std::fs::create_dir(&folder_path).unwrap();
    }
    // create json file
    let mut json_path = folder_path.clone();
    json_path.push_str("/projects.json");
    if !std::path::Path::new(&json_path).exists() {
        std::fs::File::create(&json_path).unwrap();
    }
    return json_path;
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Project {
    id: u16,
    name: String,
    path: String,
    lastopened: u128,
}

fn parse_config() -> Vec<Project> {
    let json_path = ensure_config_exists();
    let json_string = std::fs::read_to_string(json_path).unwrap();
    if json_string == "" {
        return Vec::new();
    }
    let projects: Vec<Project> = serde_json::from_str(&json_string).unwrap();
    return projects;
}

#[tauri::command]
fn add_new_project(id: &str, name: &str, path: &str, last_opened: &str) -> String {
    let mut config = parse_config();
    let new_project = Project {
        id: id.parse().unwrap(),
        name: name.to_string(),
        path: path.to_string(),
        lastopened: last_opened.parse().unwrap(),
    };

    for project in config.iter() {
        if project.path == new_project.path {
            return "Project already exists".to_string();
        }
    }
    config.push(new_project);

    let json_string = serde_json::to_string(&config).unwrap();
    std::fs::write(ensure_config_exists(), json_string).unwrap();
    return "Success".to_string();
}

#[tauri::command]
fn get_projects() -> Vec<Project> {
    let config = parse_config();
    return config;
}

#[tauri::command]
async fn generate_mermaid_class_diagram(
    file_paths: Vec<String>,
    vertical: bool,
) -> Result<String, String> {
    let mut classes = Vec::new();

    for path in file_paths {
        // println!("Parsing {}", path);
        match parse_java_file(&path) {
            Ok(class_info) => classes.push(class_info),
            Err(e) => return Err(format!("Failed to parse {}: {}", path, e)),
        }
    }

    Ok(generate_diagram(classes, vertical))
}

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
            greet,
            add_new_project,
            get_projects,
            read_file_structure,
            generate_mermaid_class_diagram,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
