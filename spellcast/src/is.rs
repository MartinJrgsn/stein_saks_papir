use super::*;

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