use std::prelude::v1::*;

use super::op::Op;

pub struct Statement {
    pub loc: Option<String>,

    pub opcode: Op,
}