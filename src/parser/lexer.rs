// we want to turn strings of nodes ("a -- b -- c + d -- e -- f") into a list of Tokens.

pub enum Token {
    Ident(String), //identifier; these are our graph nodes
    DoubleDash,
    Plus,
    Eof,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    // peeking lets us look at a character without
    // consuming it! this is going to be important for the double dashes.
    let mut chars = input.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(char) = chars.next() {
        let token: Result<Token, String> = match char {
            ' ' => continue,
            '-' => {
                if chars.peek() == Some(&'-') {
                    chars.next(); // we consume it only if it's there
                    Ok(Token::DoubleDash)
                } else {
                    return Err(format!("dash not followed up"));
                }
            }
            char if char.is_alphanumeric() => {
                let mut name = String::from(char);

                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() {
                        name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                Ok(Token::Ident(name))
            }
            _ => return Err(format!("unexpected char: '{char}'")),
        };
        tokens.push(token?);
    }

    return Ok(tokens);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_edge_list() {
        let tokens = tokenize("a -- b -- c").unwrap();
        assert_eq!(tokens.len(), 5);
    }
}
