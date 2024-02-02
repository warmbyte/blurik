pub struct Lexer {}

#[derive(Debug)]
pub enum Token {
    KeywordConst,
    Identifier(String),
    Assignment,
    StringLiteral(String),
    NumericLiteral(String),
}

impl Lexer {
    pub fn parse(source_code: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut current_token = String::new();
        let mut iter = source_code.chars().peekable();

        while let Some(&c) = iter.peek() {
            println!("{}", c);
            match c {
                ' ' | '\t' | ';' => {
                    iter.next();
                }

                '=' => {
                    tokens.push(Token::Assignment);
                    iter.next();
                }

                '"' => {
                    iter.next();
                    while let Some(&c) = iter.peek() {
                        if c == '"' {
                            tokens.push(Token::StringLiteral(current_token.clone()));
                            current_token = String::new();
                            iter.next();
                            break;
                        } else {
                            current_token.push(c);
                            iter.next();
                        }
                    }
                }

                _ if c.is_numeric() => {
                    while let Some(&c) = iter.peek() {
                        if c.is_numeric() {
                            current_token.push(c);
                            iter.next();
                        } else {
                            tokens.push(Token::NumericLiteral(current_token));
                            current_token = String::new();
                            break;
                        }
                    }
                }
                _ => {
                    while let Some(&c) = iter.peek() {
                        if c.is_ascii_alphabetic() || c == '_' {
                            current_token.push(c);
                            iter.next();
                        } else {
                            break;
                        }
                    }

                    // reserved keyword
                    if current_token == "kons" {
                        tokens.push(Token::KeywordConst);
                    } else {
                        tokens.push(Token::Identifier(current_token));
                    }
                    current_token = String::new();
                    iter.next();
                }
            };
        }

        tokens
    }
}
