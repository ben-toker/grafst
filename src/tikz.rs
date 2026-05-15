use std::fmt::Write;

use crate::graph::Graph;

pub fn emit(graph: &Graph) -> String {
    let mut out = String::new();

    writeln!(out, "\\begin{{tikzpicture}}").unwrap();

    for (i, node) in graph.nodes.iter().enumerate() {
        writeln!(
            out,
            "  \\node[draw, circle] (v{}) at ({:.3}, {:.3}) {{$v_{{{}}}$}};",
            i, node.position.0, node.position.1, i
        )
        .unwrap();
    }

    for edge in &graph.edges {
        writeln!(out, "  \\draw (v{}) -- (v{});", edge.from, edge.to).unwrap();
    }

    writeln!(out, "\\end{{tikzpicture}}").unwrap();

    out
}
