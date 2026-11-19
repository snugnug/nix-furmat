use nixon::{ast::AstNode, Element, ElementId, SyntaxKind};

pub struct FormatterConfig {
    pub space_on_array_elements: bool,
}

impl FormatterConfig {
    pub fn new(file: &String) -> Self {
        let cfg = ec4rs::properties_of(&file).unwrap_or_default();

        let space_on_array_elements = cfg
            .get_raw_for_key("space-on-array-elements")
            .into_str()
            .parse::<bool>()
            .unwrap_or(false);

        Self {
            space_on_array_elements,
        }
    }
}

pub fn format(config: FormatterConfig, file: String) {
    let content = match std::fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to read file {}", e);
            return;
        }
    };
    let document = nixon::parse(&content).expect("document");

    for diagnostic in document.diagnostics() {
        println!("{:?} {diagnostic}", diagnostic.range());
    }

    for index in 0..document.element_count() {
        let Some(Element::Token(token)) = document.element(ElementId::new(index as u32)) else {
            continue;
        };
        dbg!(&token.parent());
        dbg!(&token.parent().text());

        let kind = token.kind();

        match kind {
            SyntaxKind::LeftBracket => {}
            SyntaxKind::RightBracket => {}
            _ => {}
        }
    }
}
