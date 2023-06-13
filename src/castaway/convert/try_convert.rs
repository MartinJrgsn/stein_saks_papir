pub mod try_convert_into;

pub use try_convert_into::*;

use std::marker::Unsize;

use super::*;

pub trait TryConvert<Obj>: Is<Obj>
where
    Obj: ?Sized
{
    fn try_convert_from(object: Box<Obj>) -> Result<Box<Self>, Box<Obj>>;
    fn try_convert(object: &mut Box<Obj>) -> bool;
    fn try_convert_get(object: &mut Box<Obj>) -> Option<&Self>;
    fn try_convert_get_mut(object: &mut Box<Obj>) -> Option<&mut Self>;
}
impl<To, Obj> TryConvert<Obj> for To
where
    To: Is<Obj> + Unsize<Obj> + ?Sized + DowncastFromRef<Obj>,
    Obj: Is<Obj> + TryConvertInto<To, Obj> + DowncastRef<To> + ?Sized
{
    fn try_convert_from(object: Box<Obj>) -> Result<Box<Self>, Box<Obj>>
    {
        object.try_convert_into()
    }
    fn try_convert(object: &mut Box<Obj>) -> bool
    {
        todo!()
        /*if let Ok(converted) = object.try_convert_into()
        {
            *object = converted;
            return true
        }
        false*/
    }
    fn try_convert_get(object: &mut Box<Obj>) -> Option<&Self>
    {
        if !Self::try_convert(object)
        {
            return None
        }
        todo!()//(&**object).downcast_ref()
    }
    fn try_convert_get_mut(object: &mut Box<Obj>) -> Option<&mut Self>
    {
        if !Self::try_convert(object)
        {
            return None
        }
        todo!()//(&mut **object).downcast_mut()
    }
}