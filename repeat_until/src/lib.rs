#![feature(tuple_trait)]
#![feature(try_trait_v2)]
#![feature(unboxed_closures)]
#![feature(associated_type_bounds)]
#![feature(fn_traits)]

moddef::moddef!(
    flat(pub) mod {
        repeat_until,
        error
    }
);

#[cfg(test)]
mod tests
{
    #[test]
    fn test()
    {
        
    }
}