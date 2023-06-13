use std::marker::Unsize;

use super::*;

pub trait Is<Trait>: Unsize<Trait>
where
    Trait: ?Sized {}
pub trait IsObjOf<Type>
where
    Type: ?Sized {}
impl<Type, Trait> IsObjOf<Type> for Trait
where
    Type: Is<Trait> + ?Sized,
    Trait: ?Sized {}
pub trait Object<Trait>
where
    Trait: ?Sized
{

}