mod graph;
mod layout;
mod parser;
mod style;
mod tikz;

use std::collections::HashMap;

use graph::Graph;
use parser::Expr;

fn eval(expr: Expr) -> Graph {
    let mut graph = Graph::new();
    let mut label_to_idx: HashMap<String, usize> = HashMap::new();

    match expr {
        Expr::EdgeList(labels) => {
            let mut prev: Option<usize> = None;
            for label in labels {
                let idx = match label_to_idx.get(&label) {
                    Some(&i) => i,
                    None => {
                        let i = graph.add_node(label.clone());
                        label_to_idx.insert(label, i);
                        i
                    }
                };
                if let Some(p) = prev {
                    graph.add_edge(p, idx);
                }
                prev = Some(idx);
            }
        }
    }

    graph
}

fn main() {
    let input = std::env::args().nth(1).expect("usage: grafst '<expr>'");

    let expr = parser::parse(&input).expect("parse error");
    let mut graph = eval(expr);
    layout::layout(&mut graph);
    print!("{}", tikz::emit(&graph));
}
