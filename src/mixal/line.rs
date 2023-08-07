use std::convert::TryFrom;
use std::string::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum LineLexerState {
    InLoc,
    PostLoc,
    InOp,
    PostOp,
    InAddress,
    PostAddress,
}

#[derive(Clone, Debug)]
pub struct Line {
    pub loc: Option<String>,
    pub op: String,
    pub address: Option<String>,
}

impl TryFrom<&str> for Line {
    type Error = usize;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut state = LineLexerState::InLoc;
        let mut range_loc = 0..0;
        let mut range_op = 0..0;
        let mut range_address = value.len()..value.len();
        for (char_no, ch) in value.chars().enumerate() {
            if !ch.is_ascii() {
                return Err(char_no);
            }

            match state {
                LineLexerState::InLoc => {
                    if ch.is_ascii_whitespace() {
                        range_loc.end = char_no;
                        state = LineLexerState::PostLoc;
                    }
                }
                LineLexerState::PostLoc => {
                    if ch.is_ascii_graphic() {
                        range_op.start = char_no;
                        state = LineLexerState::InOp;
                    }
                }
                LineLexerState::InOp => {
                    if ch.is_ascii_whitespace() {
                        range_op.end = char_no;
                        state = LineLexerState::PostOp;
                    }
                }
                LineLexerState::PostOp => {
                    if ch.is_ascii_graphic() {
                        range_address.start = char_no;
                        state = LineLexerState::InAddress;
                    }
                }
                LineLexerState::InAddress => {
                    if ch.is_ascii_whitespace() {
                        range_address.end = char_no;
                        state = LineLexerState::PostAddress;
                    }
                }
                LineLexerState::PostAddress => {}
            };
        }

        if state == LineLexerState::InOp {
            // A part is omitted, fixing up OP range
            range_op.end = value.len();
            state = LineLexerState::PostAddress;
        }

        if state == LineLexerState::InAddress {
            // Comment part is omitted, fixing up ADDRESS range
            range_address.end = value.len();
            state = LineLexerState::PostAddress;
        }

        if state != LineLexerState::PostAddress {
            // Instruction end too early
            Err(value.len())
        } else {
            Ok(Line {
                loc: if range_loc.is_empty() {
                    None
                } else {
                    Some(value[range_loc].to_string())
                },
                op: value[range_op].to_string(),
                address: if range_address.is_empty() {
                    None
                } else {
                    Some(value[range_address].to_string())
                },
            })
        }
    }
}
