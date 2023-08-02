#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(associated_type_bounds)]
#![feature(trait_alias)]
#![feature(specialization)]

moddef::moddef!(
    pub mod {
        downcast,
        upcast,
        veecast,
        dyncast,
        convert
    },
    flat(pub) mod {
        is,
        as_any,
        object
    }
);

#[cfg(test)]
mod tests {
    #[allow(unused)]
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