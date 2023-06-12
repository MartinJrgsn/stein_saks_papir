use std::marker::Unsize;

use super::*;

pub trait Intersection<A, B>: Unsize<A> + Unsize<B> + AsAny
{

}
impl<T, A, B> Intersection<A, B> for T
where T: Unsize<A> + Unsize<B> + AsAny
{

}

trait A
{

}
impl A for i32
{

}
trait B
{

}