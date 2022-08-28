use crate::parse::maybe::*;

#[test]
fn test_concrete() {
    let x = Maybe::<i32, i32>::Concrete(1);
    assert_eq!(x.unwrap(), 1);
}

#[test]
fn test_is_concrete() {
    let x = Maybe::<i32, i32>::Concrete(1);
    let y = Maybe::<i32, i32>::Placeholder(1);
    assert_eq!(x.is_concrete(), true);
    assert_eq!(y.is_concrete(), false);
}

#[test]
fn test_is_placeholder() {
    let x = Maybe::<i32, i32>::Concrete(1);
    let y = Maybe::<i32, i32>::Placeholder(1);
    assert_eq!(x.is_placeholder(), false);
    assert_eq!(y.is_placeholder(), true);
}

#[test]
fn test_try_unwrap() {
    let x = Maybe::<i32, i32>::Concrete(1);
    let y = Maybe::<i32, i32>::Placeholder(1);
    assert_eq!(x.try_unwrap(), Ok(1));
    assert_eq!(y.try_unwrap(), Err(()));
}
