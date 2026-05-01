// grammar.rs controls the logic for turning the tokens (from the lexer) into expressions
// (from ast.rs). M1: a single edge list `ident -- ident -- ...`.

use super::ast::Expr;
use super::lexer::Token;

pub fn parse_tokens(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut iter = tokens.into_iter().peekable();

    let mut nodes: Vec<String> = Vec::new();

    match iter.next() {
        Some(Token::Ident(name)) => nodes.push(name),
        Some(_) => return Err("expected identifier at start of edge list".to_string()),
        None => return Err("empty input".to_string()),
    }

    while let Some(token) = iter.next() {
        match token {
            Token::DoubleDash => match iter.next() {
                Some(Token::Ident(name)) => nodes.push(name),
                Some(_) => return Err("expected identifier after '--'".to_string()),
                None => return Err("trailing '--' with no identifier".to_string()),
            },
            _ => return Err("expected '--' between identifiers".to_string()),
        }
    }

    Ok(Expr::EdgeList(nodes))
}

#[cfg(test)]
mod tests {
    use super::super::lexer::tokenize;
    use super::*;

    #[test]
    fn parses_simple_edge_list() {
        let tokens = tokenize("a -- b -- c").unwrap();
        let expr = parse_tokens(tokens).unwrap();
        assert_eq!(
            expr,
            Expr::EdgeList(vec!["a".into(), "b".into(), "c".into()])
        );
    }

    #[test]
    fn rejects_trailing_dash() {
        let tokens = tokenize("a -- b --").unwrap();
        assert!(parse_tokens(tokens).is_err());
    }
}
