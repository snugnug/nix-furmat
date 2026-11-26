use nixon::{
    ast::{self, AstNode, Expression},
    Element, ElementId, Node, SyntaxKind, TokenNode,
};

#[derive(Debug)]
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
            .get_raw_for_key("min-to-one-line-attrset")
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

fn do_format(expr: Expression, output: &mut String) {
    match expr {
        _ => output.push_str(expr.syntax().text()),
    }
}

fn walk(src: Node, config: &FormatterConfig, output: &mut String) {
    if let Some(expr) = ast::Expression::cast(src) {
        do_format(expr, output);
    }

    for child in src.child_nodes() {
        walk(child, config, output);
    }
}

pub fn run_format(config: FormatterConfig, file: String) {
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

    let mut output = String::with_capacity(content.len());

    walk(document.root(), &config, &mut output);

    println!("{}", output);
}
