use nixon::{
    ast::{self, AstNode, Expression},
    Element, ElementId, Node, SyntaxKind, TokenNode,
};

pub struct FormatterConfig {
    pub space_on_array_elements: bool,
    pub max_elems_on_one_expression: usize,
}

impl FormatterConfig {
    pub fn new(file: &String) -> Self {
        let cfg = ec4rs::properties_of(&file).unwrap_or_default();

        let space_on_array_elements = cfg
            .get_raw_for_key("space-on-array-elements")
            .into_str()
            .parse::<bool>()
            .unwrap_or(false);

        let max_elems_on_one_expression = cfg
            .get_raw_for_key("max-elemsn-on-one-expression")
            .into_str()
            .parse::<usize>()
            .unwrap_or(1);

        Self {
            space_on_array_elements,
            max_elems_on_one_expression,
        }
    }
}

fn should_inline(expr: Expression<'_, '_>, limit: usize) -> bool {
    match expr {
        Expression::List(list) => list.items().count() > limit,
        Expression::AttributeSet(set) => set.entries().count() > limit,
        _ => false,
    }
}

// fn grab_children(element: Element, elements: &mut Vec<Element>) {
fn grab_children<'a>(element: Element<'a, 'a>, elements: &mut Vec<Element<'a, 'a>>) {
    match element {
        Element::Node(node) => {
            elements.push(element);
            for child in node.children().filter(|a| matches!(a, Element::Node(_))) {
                elements.push(child);
                grab_children(child, elements);
            }
        }
        _ => {}
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
    let document = nixon::parse_syntax(&content).expect("document");

    for diagnostic in document.diagnostics() {
        println!("{:?} {diagnostic}", diagnostic.range());
    }

    let mut elements: Vec<Element> = Vec::new();

    for node in document.root().children() {
        grab_children(node, &mut elements);
    }

    dbg!(&elements);

    for element in elements {
        match element {
            Element::Node(node) => match node.kind() {
                SyntaxKind::List => {
                    let Some(list) = ast::List::cast(node) else {
                        continue;
                    };
                }
                SyntaxKind::AttributeSet => {
                    dbg!(ast::AttributeSet::cast(node));
                }
                // nixon::ast::List(expr) | nixon::ast::AttributeSet(expr) => {
                //     dbg!(should_inline(expr));
                // }
                _ => {}
            },
            _ => {}
        }
    }

    // for index in 0..document.element_count() {
    //     let Some(Element::Token(token)) = document.element(ElementId::new(index as u32)) else {
    //         continue;
    //     };
    //     // dbg!(&token);
    //     // dbg!(&token.text());

    //     let kind = token.kind();

    //     match kind {
    //         SyntaxKind::LeftBracket => {
    //             let root = token.parent();

    //             let nodes = root
    //                 .children()
    //                 .filter(|e| matches!(e, nixon::Element::Node(_)))
    //                 .count();

    //             let last = root.children().last();
    //         }
    //         SyntaxKind::LeftBrace => {
    //             let root = token.parent();

    //             let nodes = root
    //                 .children()
    //                 .filter(|e| matches!(e, nixon::Element::Node(_)))
    //                 .count();

    //             let last = root.children().last();
    //         }
    //         SyntaxKind::RightBracket => {}
    //         _ => {}
    //     }
    // }
}
