mod ast;
mod grammar;
mod lexer;

pub use ast::Expr;
pub use lexer::Token;
pub use lexer::tokenize;

pub fn parse(input: &str) -> Result<Expr, String> {
    let tokens = lexer::tokenize(input)?;

    return grammar::parse_tokens(tokens);
}
