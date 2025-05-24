use std::time;

use log::info;

use crate::diagram_generator::generate_diagram;
use crate::java_parser::parse_java_file;
use crate::types::{FileNode, Project};

#[tauri::command]
pub fn greet(name: &str) -> String {
    info!("Tauri is awesome!");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// read file structure from a path
#[tauri::command]
pub fn read_file_structure(id: &str) -> FileNode {
    let config = parse_config();
    let mut mut_config = config.clone();
    let mut path = "";
    let last_opened = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    for project in config.iter() {
        if project.id.to_string() == id {
            path = &project.path;
            break;
        }
    }

    mut_config.iter_mut().for_each(|project| {
        if project.id.to_string() == id {
            project.last_opened = last_opened;
        }
    });
    write_config(mut_config);

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

fn write_config(config: Vec<Project>) {
    let json_string = serde_json::to_string(&config).unwrap();
    // println!("{}", json_string);
    std::fs::write(ensure_config_exists(), json_string).unwrap();
}

pub fn read_structure_from_path(path: &str) -> Option<FileNode> {
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

pub fn ensure_config_exists() -> String {
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

pub fn parse_config() -> Vec<Project> {
    let json_path = ensure_config_exists();
    let json_string = std::fs::read_to_string(json_path).unwrap();
    if json_string == "" {
        return Vec::new();
    }
    let projects: Vec<Project> = serde_json::from_str(&json_string).unwrap();
    return projects;
}

#[tauri::command]
pub fn add_new_project(id: &str, name: &str, path: &str, last_opened: &str) -> String {
    let mut config = parse_config();
    let new_project = Project {
        id: id.parse().unwrap(),
        name: name.to_string(),
        path: path.to_string(),
        last_opened: last_opened.parse().unwrap(),
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
pub fn get_projects() -> Vec<Project> {
    let config = parse_config();
    return config;
}

#[tauri::command]
pub async fn generate_mermaid_class_diagram(
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
