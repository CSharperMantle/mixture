/// Represent a type that may be a placeholder.
///
/// # Generic Parameters
/// * `O` - The concrete type.
/// * `P` - The type of placeholder.
///
/// # Example
/// ```rust
/// use mixture::parse::*;
///
/// let x = Maybe::<i32, i32>::Concrete(1);
/// assert_eq!(x.unwrap(), 1);
/// ```
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Maybe<O, P> {
    /// A concrete value.
    Concrete(O),
    /// An abstract value.
    Placeholder(P),
}

impl<TObj, TId> Maybe<TObj, TId> {
    /// Return `true` if the maybe value is [`Maybe::Concrete`].
    ///
    /// # Example
    /// ```rust
    /// use mixture::parse::*;
    ///
    /// let x = Maybe::<i32, i32>::Concrete(1);
    /// assert_eq!(x.is_concrete(), true);
    ///
    /// let y = Maybe::<i32, i32>::Placeholder(1);
    /// assert_eq!(y.is_concrete(), false);
    /// ```
    pub const fn is_concrete(&self) -> bool {
        match self {
            Self::Concrete(_) => true,
            _ => false,
        }
    }

    /// Return `true` if the maybe value is [`Maybe::Placeholder`].
    ///
    /// # Example
    /// ```rust
    /// use mixture::parse::*;
    ///
    /// let x = Maybe::<i32, i32>::Concrete(1);
    /// assert_eq!(x.is_placeholder(), false);
    ///
    /// let y = Maybe::<i32, i32>::Placeholder(1);
    /// assert_eq!(y.is_placeholder(), true);
    /// ```
    pub const fn is_placeholder(&self) -> bool {
        match self {
            Self::Placeholder(_) => true,
            _ => false,
        }
    }

    /// Return the contained [`Maybe::Concrete`] value. Consumes the `self` value.
    ///
    /// # Panics
    /// Panics if the `self` value is an [`Maybe::Placeholder`].
    ///
    /// # Returns
    /// * `TObj` - The contained concrete value.
    ///
    /// # Examples
    ///
    /// Example 1
    /// ```rust
    /// use mixture::parse::*;
    ///
    /// let x = Maybe::<i32, i32>::Concrete(1);
    /// assert_eq!(x.unwrap(), 1);
    /// ```
    /// Example 2
    /// ```rust,should_panic
    /// use mixture::parse::*;
    ///
    /// let x = Maybe::<i32, i32>::Placeholder(1);
    /// assert_eq!(x.unwrap(), 1);  // panics
    /// ```
    pub fn unwrap(self) -> TObj {
        match self {
            Self::Concrete(x) => x,
            Self::Placeholder(_) => panic!("Maybe::unwrap: found a placeholder"),
        }
    }

    /// Try to return the contained [`Maybe::Concrete`] value.
    /// Consumes the `self` value.
    ///
    /// # Returns
    /// * `Ok(TObj)` - The contained concrete value.
    /// * `Err(())` - The value is a placeholder.
    ///
    /// # Example
    /// ```rust
    /// use mixture::parse::*;
    ///
    /// let x = Maybe::<i32, i32>::Concrete(1);
    /// assert_eq!(x.try_unwrap(), Ok(1));
    ///
    /// let y = Maybe::<i32, i32>::Placeholder(1);
    /// assert_eq!(y.try_unwrap(), Err(()));  // panics
    /// ```
    pub fn try_unwrap(self) -> Result<TObj, ()> {
        match self {
            Self::Concrete(x) => Ok(x),
            Self::Placeholder(_) => Err(()),
        }
    }
}
