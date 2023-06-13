pub mod convert_into;
pub mod try_convert;

pub use convert_into::*;
pub use try_convert::*;

use std::marker::Unsize;

use super::*;

pub trait Convert<Obj>: Is<Obj>
where
    Obj: ?Sized
{
    fn convert_from(object: Box<Obj>) -> Box<Self>;
    fn convert(object: &mut Box<Obj>);
    fn convert_get(object: &mut Box<Obj>) -> &Self;
    fn convert_get_mut(object: &mut Box<Obj>) -> &mut Self;
}
impl<To, Obj> Convert<Obj> for To
where
    To: Is<Obj> + Unsize<Obj>,
    Obj: ConvertInto<Self> + ?Sized
{
    fn convert_from(object: Box<Obj>) -> Box<Self>
    {
        object.convert_into()
    }
    fn convert(object: &mut Box<Obj>)
    {
        *object = object.convert_into()
    }
    fn convert_get(object: &mut Box<Obj>) -> &Self
    {
        Self::convert(object);
        object.as_any().downcast_ref().unwrap()
    }
    fn convert_get_mut(object: &mut Box<Obj>) -> &mut Self
    {
        Self::convert(object);
        object.as_any_mut().downcast_mut().unwrap()
    }
}