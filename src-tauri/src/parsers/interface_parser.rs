use tree_sitter::{Node, Query, QueryCursor};

use crate::types::{ClassField, ClassInfo, ClassMethod, ClassType};

use super::extract_package;

pub fn parse_java_interface(
    source_code: &str,
    root_node: Node<'_>,
) -> Result<ClassInfo, Box<dyn std::error::Error>> {
    let language = tree_sitter_java::language();
    let query = Query::new(
        language,
        "
        (interface_declaration
            name: (identifier) @class_name
            superclass: (superclass
                (type_identifier) @extends)?
            interfaces: (super_interfaces
                (type_list
                    (type_identifier) @implements))?)
    ",
    )?;

    let mut class_cursor = QueryCursor::new();

    let matches = class_cursor.matches(&query, root_node, source_code.as_bytes());
    let class_matches = matches.collect::<Vec<_>>();

    // Extract class information
    let mut class_info = ClassInfo {
        name: String::new(),
        package: extract_package(&source_code, root_node)?,
        fields: Vec::new(),
        methods: Vec::new(),
        extends: None,
        implements: Vec::new(),
        class_type: ClassType::Interface,
    };

    let capture_names = query.capture_names();

    // print out the match
    for match_ in class_matches {
        // println!("Match: {:?}", match_);
        for capture in match_.captures {
            let capture_index = capture.index as usize;
            let capture_name = &capture_names[capture_index];
            match capture_name.as_str() {
                "class_name" => {
                    class_info.name = capture.node.utf8_text(source_code.as_bytes())?.to_string()
                }
                "extends" => {
                    class_info.extends =
                        Some(capture.node.utf8_text(source_code.as_bytes())?.to_string())
                }
                "implements" => class_info
                    .implements
                    .push(capture.node.utf8_text(source_code.as_bytes())?.to_string()),
                _ => {}
            }
        }
        // println!("Class: {:?}", class_info); // Debug print for class info
    }

    // Parse fields
    let field_query = Query::new(
        language,
        "
        (field_declaration
            (modifiers
                [\"public\" \"private\" \"protected\"] @visibility)?
            type: (_) @type
            declarator: (variable_declarator
                name: (identifier) @name))
    ",
    )?;

    let mut query_cursor = QueryCursor::new();
    let field_matches = query_cursor.matches(&field_query, root_node, source_code.as_bytes());
    for match_ in field_matches {
        // println!("Match: {:?}", match_); // Debug print for match
        let mut field = ClassField {
            name: String::new(),
            type_name: String::new(),
            visibility: String::new(),
        };

        for capture in match_.captures {
            match capture.node.kind() {
                "identifier" => {
                    field.name = capture.node.utf8_text(source_code.as_bytes())?.to_string()
                }
                "private" => field.visibility = "private".to_string(),
                "public" => field.visibility = "public".to_string(),
                "protected" => field.visibility = "protected".to_string(),
                "type_identifier" => {
                    field.type_name = capture.node.utf8_text(source_code.as_bytes())?.to_string()
                }
                "generic_type" => {
                    field.type_name = capture.node.utf8_text(source_code.as_bytes())?.to_string()
                }
                "boolean_type" => {
                    field.type_name = capture.node.utf8_text(source_code.as_bytes())?.to_string()
                }
                "visibility" => {
                    field.visibility = capture.node.utf8_text(source_code.as_bytes())?.to_string()
                }
                _ => {}
            }
        }

        // println!("Field: {:?}", field); // Debug print for field identifie

        class_info.fields.push(field);
    }

    // Parse methods
    let method_query = Query::new(
        language,
        "
        (method_declaration
            (modifiers
                [\"public\" \"private\" \"protected\"] @visibility)?
            type: (_) @return_type
            name: (identifier) @name
            parameters: (formal_parameters
                (formal_parameter
                    type: (_) @param_type
                    name: (identifier) @param_name)*))
    ",
    )?;

    let method_matches = query_cursor.matches(&method_query, root_node, source_code.as_bytes());
    for match_ in method_matches {
        // println!("Match: {:?}", match_); // Debug print for match
        let mut method = ClassMethod {
            name: String::new(),
            return_type: String::new(),
            visibility: String::new(),
            parameters: Vec::new(),
        };

        for capture in match_.captures {
            let index = capture.index as usize;
            match index {
                0 => {
                    method.visibility = capture.node.utf8_text(source_code.as_bytes())?.to_string()
                }
                1 => {
                    method.return_type = capture.node.utf8_text(source_code.as_bytes())?.to_string()
                }
                2 => method.name = capture.node.utf8_text(source_code.as_bytes())?.to_string(),
                _ => {}
            }
            // collect all other indexes and combine them into a string
            if index > 2 && index % 2 != 0 {
                let param_name = capture.node.utf8_text(source_code.as_bytes())?.to_string();
                let param_type = capture
                    .node
                    .next_sibling()
                    .and_then(|n| n.utf8_text(source_code.as_bytes()).ok())
                    .unwrap_or("")
                    .to_string();
                method.parameters.push((param_name, param_type));
            }
        }

        // println!("Method: {:?}", method); // Debug print for method

        class_info.methods.push(method);
    }

    Ok(class_info)
}
