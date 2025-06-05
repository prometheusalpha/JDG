use tree_sitter::{Node, Query, QueryCursor};

use crate::types::{ClassField, ClassInfo, ClassType};

use super::extract_package;

/// Creates a tree-sitter query for Java record declarations
fn create_record_query() -> Result<Query, Box<dyn std::error::Error>> {
    let language = tree_sitter_java::language();
    Query::new(
        language,
        "
        (record_declaration
            name: (identifier) @record-name
            parameters: (formal_parameters
                (formal_parameter
                    type: (_) @field-type
                    name: (identifier)
                )+
            ) @record-fields
        )",
    )
    .map_err(|e| e.into())
}

/// Initialize a ClassInfo structure for a Java record
fn init_class_info(
    source_code: &str,
    root_node: Node<'_>,
) -> Result<ClassInfo, Box<dyn std::error::Error>> {
    Ok(ClassInfo {
        name: String::new(),
        package: extract_package(source_code, root_node)?,
        fields: Vec::new(),
        methods: Vec::new(),
        extends: None,
        implements: Vec::new(),
        class_type: ClassType::Record,
    })
}

/// Extract record name from a capture node
fn extract_record_name(
    capture: &tree_sitter::QueryCapture,
    source_code: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(capture.node.utf8_text(source_code.as_bytes())?.to_string())
}

/// Process record fields from a capture node
fn process_record_fields(
    capture: &tree_sitter::QueryCapture,
    source_code: &str,
) -> Result<Vec<ClassField>, Box<dyn std::error::Error>> {
    let mut fields = Vec::new();
    let mut index = 0;

    while index < capture.node.child_count() {
        let child = capture.node.child(index).unwrap();
        if child.kind() == "formal_parameter" {
            if let Some(field) = extract_field_from_parameter(child, source_code)? {
                fields.push(field);
            }
        }
        index += 1;
    }

    Ok(fields)
}

/// Extract a field from a formal parameter node
fn extract_field_from_parameter(
    param_node: Node,
    source_code: &str,
) -> Result<Option<ClassField>, Box<dyn std::error::Error>> {
    let mut i = 0;
    while i < param_node.child_count() {
        let child = param_node.child(i).unwrap();
        if child.kind() == "type_identifier" {
            let field_type = child.utf8_text(source_code.as_bytes())?.to_string();
            let field_name_node = child.next_sibling().unwrap();
            let field_name = field_name_node.utf8_text(source_code.as_bytes())?;

            return Ok(Some(ClassField {
                name: field_name.to_string(),
                type_name: field_type,
                visibility: String::new(),
            }));
        }
        i += 1;
    }

    Ok(None)
}

/// Parse a Java record from source code
pub fn parse_java_record(
    source_code: &str,
    root_node: Node<'_>,
) -> Result<ClassInfo, Box<dyn std::error::Error>> {
    // Create query and cursor
    let query = create_record_query()?;
    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, root_node, source_code.as_bytes());
    let class_matches = matches.collect::<Vec<_>>();

    // Initialize class info
    let mut class_info = init_class_info(source_code, root_node)?;
    let capture_names = query.capture_names();

    // Process matches
    let match_option = class_matches.first();
    if match_option.is_none() {
        return Ok(class_info);
    }
    let match_ = match_option.unwrap();
    for capture in match_.captures {
        let capture_index = capture.index as usize;
        let capture_name = &capture_names[capture_index];

        match capture_name.as_str() {
            "record-name" => class_info.name = extract_record_name(&capture, source_code)?,
            "record-fields" => class_info.fields = process_record_fields(&capture, source_code)?,
            _ => {}
        }
    }

    Ok(class_info)
}
