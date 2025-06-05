use tree_sitter::{Parser, Query, QueryCursor};

pub mod class_parser;
pub mod enum_parser;
pub mod interface_parser;
pub mod record_parser;

fn extract_package(
    source_code: &str,
    root_node: tree_sitter::Node,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut parser = Parser::new();
    let language = tree_sitter_java::language();
    parser.set_language(language)?;

    let package_query = Query::new(
        language,
        "
    (package_declaration
        (scoped_identifier) @package)
        ",
    )?;

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&package_query, root_node, source_code.as_bytes());

    for match_ in matches {
        if let Some(capture) = match_.captures.first() {
            if package_query.capture_names()[capture.index as usize] == "package" {
                return Ok(capture.node.utf8_text(source_code.as_bytes())?.to_string());
            }
        }
    }

    Ok(String::new())
}
