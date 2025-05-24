use serde::{Deserialize, Serialize};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub r#type: String,
    pub children: Vec<FileNode>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Project {
    pub id: u16,
    pub name: String,
    pub path: String,
    pub last_opened: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassField {
    pub name: String,
    pub type_name: String,
    pub visibility: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassMethod {
    pub name: String,
    pub return_type: String,
    pub visibility: String,
    pub parameters: Vec<(String, String)>, // (param_name, param_type)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClassType {
    Class,
    AbstractClass,
    Interface,
    Enum,
    Record,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassInfo {
    pub name: String,
    pub package: String,
    pub fields: Vec<ClassField>,
    pub methods: Vec<ClassMethod>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub class_type: ClassType,
}
