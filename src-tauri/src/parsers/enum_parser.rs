use tree_sitter::{Node, Query, QueryCursor};

use crate::types::{ClassField, ClassInfo, ClassType};

use super::extract_package;

pub fn parse_java_enum(
    source_code: &str,
    root_node: Node<'_>,
) -> Result<ClassInfo, Box<dyn std::error::Error>> {
    let language = tree_sitter_java::language();
    let query = Query::new(
        language,
        "(enum_declaration
            name: (identifier) @enum-name
             (enum_body (enum_constant name: (identifier) @enum-value)))",
    )?;

    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, root_node, source_code.as_bytes());
    let class_matches = matches.collect::<Vec<_>>();

    // Extract class information
    let mut class_info = ClassInfo {
        name: String::new(),
        package: extract_package(&source_code, root_node)?,
        fields: Vec::new(),
        methods: Vec::new(),
        extends: None,
        implements: Vec::new(),
        class_type: ClassType::Enum,
    };

    let capture_names = query.capture_names();

    // print out the match
    for match_ in class_matches {
        // println!("Match: {:?}", match_);
        for capture in match_.captures {
            let capture_index = capture.index as usize;
            let capture_name = &capture_names[capture_index];
            // println!("Capture: {}", capture_name);
            match capture_name.as_str() {
                "enum-name" => {
                    class_info.name = capture.node.utf8_text(source_code.as_bytes())?.to_string()
                }
                "enum-value" => class_info.fields.push(ClassField {
                    name: capture.node.utf8_text(source_code.as_bytes())?.to_string(),
                    type_name: String::new(),
                    visibility: String::new(),
                }),
                _ => {}
            }
        }
        // println!("Class: {:?}", class_info); // Debug print for class info
    }

    // Parse fields
    Ok(class_info)
}
