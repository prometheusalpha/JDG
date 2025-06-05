use std::fs;
use tree_sitter::Parser;

use crate::{parsers, types::ClassInfo};

pub fn parse_java_file(file_path: &str) -> Result<ClassInfo, Box<dyn std::error::Error>> {
    let source_code = fs::read_to_string(file_path)?;

    // Initialize tree-sitter parser
    let mut parser = Parser::new();
    let language = tree_sitter_java::language();
    parser.set_language(language)?;

    let tree = parser.parse(&source_code, None).unwrap();
    let root_node = tree.root_node();

    let mut i = 0;
    let mut class_node = None;
    while i < root_node.child_count() {
        let child = root_node.child(i);
        if child.is_none() {
            break;
        }
        if [
            "class_declaration",
            "interface_declaration",
            "enum_declaration",
            "record_declaration"
        ]
        .contains(&child.unwrap().kind())
        {
            class_node = Some(child.unwrap());
            break;
        }
        i += 1;
    }

    if class_node.is_none() {
        return Err("No class found".into());
    }

    // println!("{:?}", class_node.unwrap().kind());

    let class_info = match class_node.unwrap().kind() {
        "class_declaration" => parsers::class_parser::parse_java_class(&source_code, root_node),
        "interface_declaration" => {
            parsers::interface_parser::parse_java_interface(&source_code, root_node)
        }
        "enum_declaration" => parsers::enum_parser::parse_java_enum(&source_code, root_node),
        "record_declaration" => parsers::record_parser::parse_java_record(&source_code, root_node),
        _ => Err("Unsupported class type".into()),
    }?;
    Ok(class_info)
}
