use std::convert::TryFrom;
use std::string::*;
use std::vec;
use std::vec::*;

use crate::mixal::*;

fn parse_atom(input: &str) -> Result<Token, ()> {
    if regex::REGEX_IDENTIFIER.is_match(input) {
        Ok(Token::Symbol(input.to_string()))
    } else if regex::REGEX_NUMBER.is_match(input) {
        Ok(Token::Number(input.to_string()))
    } else if regex::REGEX_ASTERISK.is_match(input) {
        Ok(Token::Asterisk)
    } else {
        Err(())
    }
}

fn parse_op(input: &str) -> Result<Token, ()> {
    match input {
        "+" => Ok(Token::Plus),
        "-" => Ok(Token::Minus),
        "*" => Ok(Token::Star),
        "/" => Ok(Token::Slash),
        "//" => Ok(Token::DoubleSlash),
        ":" => Ok(Token::Colon),
        _ => Err(()),
    }
}

fn parse_expr(input: &str) -> Result<Vec<Token>, ()> {
    let mut tokens = vec![];
    let mut op;
    let mut this;
    let mut rest;
    if let Some(r) = regex::REGEX_EXPR_INITIAL.captures(input) {
        op = r.name("this_unop");
        this = r.name("this_atom").unwrap_or_else(|| unreachable!());
        rest = r.name("rest");
    } else {
        return Err(());
    };

    loop {
        if let Ok(t) = parse_op(op.map_or("", |m| m.as_str())) {
            tokens.push(t);
        }
        tokens.push(parse_atom(this.as_str()).unwrap_or_else(|_| unreachable!()));
        if let Some(rest_match) = rest {
            let rest_match = rest_match.as_str();
            if rest_match.is_empty() {
                break Ok(tokens);
            }
            if let Some(r) = regex::REGEX_EXPR_REST.captures(rest_match) {
                op = r.name("this_binop");
                this = r.name("this_atom").unwrap_or_else(|| unreachable!());
                rest = r.name("rest");
            } else {
                break Err(());
            }
        }
    }
}

fn parse_w_value(input: &str) -> Result<Vec<ParsedAddress>, ()> {
    let mut w_atoms = vec![];
    let mut this;
    let mut rest;
    if let Some(c) = regex::REGEX_W_VALUE_INITIAL.captures(input) {
        this = c.name("this").unwrap_or_else(|| unreachable!());
        rest = c.name("rest");
    } else {
        return Err(());
    };

    loop {
        if let Ok(addr) = parse_normal_address(this.as_str()) {
            w_atoms.push(addr);
        }
        if let Some(rest_match) = rest {
            let rest_match = rest_match.as_str();
            if rest_match.is_empty() {
                break Ok(w_atoms);
            }
            if let Some(r) = regex::REGEX_W_VALUE_REST.captures(rest_match) {
                this = r.name("this").unwrap_or_else(|| unreachable!());
                rest = r.name("rest");
            } else {
                break Err(());
            }
        }
    }
}

fn parse_const(input: &str) -> Result<Vec<ParsedAddress>, ()> {
    if let Some(c) = regex::REGEX_CONST.captures(input) {
        let w_value = c.name("w_value").unwrap_or_else(|| unreachable!());
        parse_w_value(w_value.as_str())
    } else {
        Err(())
    }
}

#[derive(Clone, Debug)]
pub struct ParsedAddress {
    pub a_part: Vec<Token>,
    pub i_part: Option<Vec<Token>>,
    pub f_part: Option<Vec<Token>>,
}

fn parse_normal_address(input: &str) -> Result<ParsedAddress, ()> {
    if let Some(c) = regex::REGEX_ADDRESS_A_AF.captures(input) {
        let a_part = parse_expr(c.name("a").unwrap_or_else(|| unreachable!()).as_str())?;
        let f_part = match c.name("f") {
            Some(m) => match parse_expr(m.as_str()) {
                Ok(v) => Ok(Some(v)),
                Err(_) => Err(()),
            },
            None => Ok(None),
        }?;
        Ok(ParsedAddress {
            a_part,
            i_part: None,
            f_part,
        })
    } else if let Some(c) = regex::REGEX_ADDRESS_AI_AIF.captures(input) {
        let a_part = parse_expr(c.name("a").unwrap_or_else(|| unreachable!()).as_str())?;
        let i_part = parse_expr(c.name("i").unwrap_or_else(|| unreachable!()).as_str())?;
        let f_part = match c.name("f") {
            Some(m) => match parse_expr(m.as_str()) {
                Ok(v) => Ok(Some(v)),
                Err(_) => Err(()),
            },
            None => Ok(None),
        }?;
        Ok(ParsedAddress {
            a_part,
            i_part: Some(i_part),
            f_part,
        })
    } else {
        Err(())
    }
}

#[derive(Clone, Debug)]
pub enum AddressType {
    Normal(ParsedAddress),
    Const(Vec<ParsedAddress>),
    WValue(Vec<ParsedAddress>),
    Literal(String),
}

#[derive(Clone, Copy, Debug)]
pub struct ParseError {
    pub line_no: usize,
    pub char_no: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct ParsedLine {
    pub loc: Option<Token>,
    pub op: String,
    pub address: Option<AddressType>,
}

impl TryFrom<&str> for ParsedLine {
    type Error = usize;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let line = Line::try_from(value)?;
        let op = line.op;
        let loc = line.loc.map(Token::Symbol);
        let address = line.address.map(|s| {
            if let Ok(result) = parse_normal_address(&s) {
                AddressType::Normal(result)
            } else if let Ok(result) = parse_w_value(&s) {
                AddressType::WValue(result)
            } else if let Ok(result) = parse_const(&s) {
                AddressType::Const(result)
            } else {
                AddressType::Literal(s)
            }
        });
        Ok(ParsedLine { loc, op, address })
    }
}
