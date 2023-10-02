#![allow(clippy::unwrap_used)]

use const_format::formatcp;
use once_cell::sync::Lazy;
use regex::Regex;

// Example matches: 1, IDENT, +1, +IDENT, *, +23-IDENT**/987, 0:5, BEGIN:END
// Captures: this_unop, this_atom, rest
pub(crate) const STR_REGEX_EXPR_INITIAL: &str = r"^(?P<this_unop>[+\-]?)(?P<this_atom>[A-Z0-9]+|\*)(?P<rest>(?:(?:[+\-*/:]|(?://))(?:[A-Z0-9]+|\*))*)$";
pub(crate) static REGEX_EXPR_INITIAL: Lazy<Regex> =
    Lazy::new(|| Regex::new(STR_REGEX_EXPR_INITIAL).unwrap());

// Example matches: +1, -IDENT, **, /1+IDENT-2, :END
// Captures: this_binop, this_atom, rest
pub(crate) const STR_REGEX_EXPR_REST: &str = r"^(?P<this_binop>(?:[+\-*/:]|(?://)))(?P<this_atom>[A-Z0-9]+|\*)(?P<rest>(?:(?:[+\-*/:]|(?://))(?:[A-Z0-9]+|\*))*)$";
pub(crate) static REGEX_EXPR_REST: Lazy<Regex> =
    Lazy::new(|| Regex::new(STR_REGEX_EXPR_REST).unwrap());

// Capture-less version of STR_REGEX_EXPR_INITIAL.
// Captures: <N/A>
pub(crate) const STR_REGEX_NG_EXPR_INITIAL: &str =
    r"(?:[+\-]?)(?:[A-Z0-9]+|\*)(?:(?:(?:[+\-*/:]|(?://))(?:[A-Z0-9]+|\*))*)";

// Example matches: IDENT, 20BY20, 101A
// Captures: <N/A>
pub(crate) const STR_REGEX_IDENTIFIER: &str = r"^[A-Z0-9]*[A-Z][A-Z-0-9]*$";
pub(crate) static REGEX_IDENTIFIER: Lazy<Regex> =
    Lazy::new(|| Regex::new(STR_REGEX_IDENTIFIER).unwrap());

// Example matches: 123, 1, 000
// Captures: <N/A>
pub(crate) const STR_REGEX_NUMBER: &str = r"^[0-9]+$";
pub(crate) static REGEX_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(STR_REGEX_NUMBER).unwrap());

// Example matches: *
// Captures: <N/A>
pub(crate) const STR_REGEX_ASTERISK: &str = r"^\*$";
pub(crate) static REGEX_ASTERISK: Lazy<Regex> =
    Lazy::new(|| Regex::new(STR_REGEX_ASTERISK).unwrap());

// Example matches: 1000(0:1), IDENT, IDENT+1(FIELD), 1000(BEGIN:END)
// Captures: a, f?
pub(crate) const STR_REGEX_ADDRESS_A_AF: &str =
    formatcp!(r"^(?P<a>{STR_REGEX_NG_EXPR_INITIAL})(?:\((?P<f>{STR_REGEX_NG_EXPR_INITIAL})\))?$");
pub(crate) static REGEX_ADDRESS_A_AF: Lazy<Regex> =
    Lazy::new(|| Regex::new(STR_REGEX_ADDRESS_A_AF).unwrap());

// Example matches: 1000,1(0:5), IDENT,IDX, IDENT+1,IDX-1
// Captures: a, i, f?
pub(crate) const STR_REGEX_ADDRESS_AI_AIF: &str =
    formatcp!(r"^(?P<a>{STR_REGEX_NG_EXPR_INITIAL}),(?P<i>{STR_REGEX_NG_EXPR_INITIAL})(?:\((?P<f>{STR_REGEX_NG_EXPR_INITIAL})\))?$");
pub(crate) static REGEX_ADDRESS_AI_AIF: Lazy<Regex> =
    Lazy::new(|| Regex::new(STR_REGEX_ADDRESS_AI_AIF).unwrap());

pub(crate) const STR_REGEX_NG_W_ATOM: &str =
    formatcp!(r"(?:{STR_REGEX_NG_EXPR_INITIAL})(?:\((?:{STR_REGEX_NG_EXPR_INITIAL})\))?");

// Example matches: 1000(0:1),2000,3000(4:5), IDENT, IDENT(FIELD)
// Captures: this, rest
pub(crate) const STR_REGEX_W_VALUE_INITIAL: &str =
    formatcp!(r"^(?P<this>{STR_REGEX_NG_W_ATOM})(?P<rest>(?:,{STR_REGEX_NG_W_ATOM})*)$");
pub(crate) static REGEX_W_VALUE_INITIAL: Lazy<Regex> =
    Lazy::new(|| Regex::new(STR_REGEX_W_VALUE_INITIAL).unwrap());

// Capture-less version of STR_REGEX_W_VALUE_INITIAL.
// Captures: <N/A>
pub(crate) const STR_REGEX_NG_W_VALUE_INITIAL: &str =
    formatcp!(r"(?:{STR_REGEX_NG_W_ATOM})(?:(?:,{STR_REGEX_NG_W_ATOM})*)");

// Example matches: ,1000(0:1),2000,3000(4:5), ,IDENT, ,IDENT(FIELD)
// Captures: this, rest
pub(crate) const STR_REGEX_W_VALUE_REST: &str =
    formatcp!(r"^,(?P<this>{STR_REGEX_NG_W_ATOM})(?P<rest>(?:,{STR_REGEX_NG_W_ATOM})*)$");
pub(crate) static REGEX_W_VALUE_REST: Lazy<Regex> =
    Lazy::new(|| Regex::new(STR_REGEX_W_VALUE_REST).unwrap());

// Example matches: =1000(0:1),2000,3000(4:5)=, =IDENT+1=, =IDENT(FIELD)=
// Captures: w_value
pub(crate) const STR_REGEX_CONST: &str =
    formatcp!(r"^=(?P<w_value>{STR_REGEX_NG_W_VALUE_INITIAL})=$");
pub(crate) static REGEX_CONST: Lazy<Regex> = Lazy::new(|| Regex::new(STR_REGEX_CONST).unwrap());
