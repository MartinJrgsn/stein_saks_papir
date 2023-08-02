use super::*;

/// A trait which signifies that Self, a struct, implements a trait, given as a trait-object in the generic
/// 
/// # Example
/// 
/// ```rust
/// use spellcast::Is;
/// 
/// trait A {}
/// trait B {}
/// 
/// struct X;
/// 
/// impl A for X {}
/// 
/// static_assertions::assert_impl_one!(X: Is<dyn A>);
/// ```
pub trait Is<Trait> = private::Is<Trait>
where
    Trait: ?Sized;
pub trait IsObjOf<Type>: private::IsObjOf<Type>
where
    Type: ?Sized {}
impl<Type, Trait> IsObjOf<Type> for Trait
where
    Type: Is<Trait> + ?Sized,
    Trait: ?Sized {}