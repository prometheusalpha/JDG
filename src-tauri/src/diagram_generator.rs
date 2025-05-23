use crate::java_parser::ClassInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassDiagram {
    pub classes: Vec<ClassInfo>,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub from: String,
    pub to: String,
    pub type_: String, // "extends", "implements", "association", "composition", "aggregation"
}

pub fn generate_diagram(classes: Vec<ClassInfo>, vertical: bool) -> String {
    let mut diagram = ClassDiagram {
        classes,
        relationships: Vec::new(),
    };

    // Process inheritance relationships
    for class in &diagram.classes {
        if let Some(extends) = &class.extends {
            diagram.relationships.push(Relationship {
                from: class.name.to_owned(),
                to: extends.to_owned(),
                type_: String::from("extends"),
            });
        }

        for implements in &class.implements {
            diagram.relationships.push(Relationship {
                from: class.name.to_owned(),
                to: implements.to_owned(),
                type_: String::from("implements"),
            });
        }
    }

    // Process associations based on field types
    for class in &diagram.classes {
        for field in &class.fields {
            if diagram.classes.iter().any(|c| c.name == field.type_name) {
                diagram.relationships.push(Relationship {
                    from: class.name.to_owned(),
                    to: field.type_name.to_owned(),
                    type_: String::from("association"),
                });
            }
        }
    }
    to_mermaid(&diagram, vertical)
}

pub fn to_mermaid(diagram: &ClassDiagram, vertical: bool) -> String {
    let mut mermaid = String::from("classDiagram\n");
    if vertical {
        mermaid.push_str("    direction LR\n");
    }

    // Add classes with their members
    for class in &diagram.classes {
        // Class declaration
        let kind = match class.is_interface {
            true => "<<interface>>",
            false => "",
        };
        mermaid.push_str(&format!("class {} {{\n", class.name));
        mermaid.push_str(&format!("    {}\n", kind)); // Class head

        // Fields
        for field in &class.fields {
            let field_visibility = match field.visibility.as_str() {
                "public" => "+",
                "private" => "-",
                "protected" => "#",
                _ => "",
            };
            mermaid.push_str(&format!(
                "        {} {}: {}\n",
                field_visibility, field.name, field.type_name
            ));
        }

        // Methods
        for method in &class.methods {
            let params = method
                .parameters
                .iter()
                .map(|(name, type_)| format!("{}: {}", name, type_))
                .collect::<Vec<_>>()
                .join(", ");

            mermaid.push_str(&format!(
                "        {} {}: {}({})\n",
                match method.visibility.as_str() {
                    "public" => "+",
                    "private" => "-",
                    "protected" => "#",
                    _ => "",
                },
                method.name,
                method.return_type,
                params
            ));
        }

        mermaid.push_str("    }\n");
    }

    // Add relationships
    for rel in &diagram.relationships {
        let arrow = match rel.type_.as_str() {
            "extends" => "<|--",
            "implements" => "<|--",
            "association" => "-->",
            "composition" => "*--",
            "aggregation" => "o--",
            _ => "--",
        };

        mermaid.push_str(&format!("    {} {} {}\n", rel.from, arrow, rel.to));
    }

    mermaid
}
