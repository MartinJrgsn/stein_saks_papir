#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(associated_type_bounds)]
#![feature(trait_alias)]
#![feature(specialization)]

pub mod downcast;
pub mod upcast;
pub mod veecast;
pub mod dyncast;

pub mod is;
pub mod as_any;
pub mod convert;
pub mod object;

pub use is::*;
pub use as_any::*;
pub use convert::*;
pub use object::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}

mod private
{
    use std::marker::Unsize;
    
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
}