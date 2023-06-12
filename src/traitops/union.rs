use std::marker::Unsize;

use super::*;

pub trait Union<A, B>: AsAny
{

}
default impl<T, A, B> Union<A, B> for T
where T: Unsize<A> + AsAny
{

}
impl<T, A, B> Union<A, B> for T
where T: Unsize<B> + AsAny
{

}

fn test()
{
    use std::ops::Add;
    
    let i = Box::new(0i32) as Box<Union<Add<i32>, FnOnce>>;
    let y: i32 = i.downcast();
}