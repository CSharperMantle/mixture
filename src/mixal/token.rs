use std::string::String;

#[derive(Clone, Debug)]
pub enum Token {
    Symbol(String),
    Number(String),
    Plus,
    Minus,
    Star,
    Slash,
    DoubleSlash,
    Colon,
    Asterisk,
}
