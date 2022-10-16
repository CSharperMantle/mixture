/// A symbol parsed in MIXAL.
/// 
/// In each line of MIXAL code, a symbol may appear in
/// various locations
/// 
/// # Generic Parameters
/// * `I` - The type used as internal identifier.
/// 
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Symbol<I> {
    Def(I),
    Ref(I),
    Special(I),
}
