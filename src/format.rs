use nixon::Element;

pub fn format(file: String) {
    let content = match std::fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to read file {}", e);
            return;
        }
    };

    let document = nixon::parse(&content);
    // dbg!(document);
    for element in document.expect("reaosn").root().children() {
        match element {
            // Element::Node(node) => println!("{:?} {}", node.kind(), node.text()),
            Element::Node(node) => {
                println!("{:?} {}", node.kind(), node.text());
                let _ = node
                    .children()
                    .for_each(|c| println!("{:?} {}", c.kind(), c.text()));
            }
            Element::Token(token) => println!("{:?} {}", token.kind(), token.text()),
        }
    }

    // println!("{}", content);
}
