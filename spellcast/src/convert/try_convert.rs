use std::marker::Unsize;

use crate::downcast::{DowncastRef, DowncastFromRef};

use super::*;

pub trait TryConvert<Obj>: Is<Obj>
where
    Obj: ?Sized
{
    fn try_convert_from(object: Box<Obj>) -> Result<Box<Self>, Box<Obj>>;
    fn try_convert(object: &mut Option<Box<Obj>>) -> bool;
    fn try_convert_get(object: &mut Option<Box<Obj>>) -> Option<&Self>;
    fn try_convert_get_mut(object: &mut Option<Box<Obj>>) -> Option<&mut Self>;
    fn try_convert_copy(object: &mut Box<Obj>) -> bool where Box<Obj>: Copy;
    fn try_convert_copy_get(object: &mut Box<Obj>) -> Option<&Self> where Box<Obj>: Copy;
    fn try_convert_copy_get_mut(object: &mut Box<Obj>) -> Option<&mut Self> where Box<Obj>: Copy;
}
impl<'a, To, Obj> TryConvert<Obj> for To
where
    To: Is<Obj> + Unsize<Obj> + ?Sized + DowncastFromRef<Obj>,
    Obj: Is<Obj> + TryConvertInto<To, Obj> + DowncastRef<To> + ?Sized
{
    fn try_convert_from(object: Box<Obj>) -> Result<Box<Self>, Box<Obj>>
    {
        object.try_convert_into()
    }
    fn try_convert(object: &mut Option<Box<Obj>>) -> bool
    {
        if let Some(object_taken) = object.take()
        {
            if let Ok(converted) = object_taken.try_convert_into()
            {
                *object = Some(converted);
                return true
            }
        }
        false
    }
    fn try_convert_get(object: &mut Option<Box<Obj>>) -> Option<&Self>
    {
        if !Self::try_convert(object)
        {
            return None
        }
        object.as_ref().and_then(|object| (&**object).downcast_ref())
    }
    fn try_convert_get_mut(object: &mut Option<Box<Obj>>) -> Option<&mut Self>
    {
        if !Self::try_convert(object)
        {
            return None
        }
        object.as_mut().and_then(|object| (&mut **object).downcast_mut())
    }
    fn try_convert_copy(object: &mut Box<Obj>) -> bool
    where Box<Obj>: Copy
    {
        if let Ok(converted) = object.try_convert_into()
        {
            *object = converted;
            return true
        }
        false
    }
    fn try_convert_copy_get(object: &mut Box<Obj>) -> Option<&Self>
    where Box<Obj>: Copy
    {
        if !Self::try_convert_copy(object)
        {
            return None
        }
        todo!()//(&**object).downcast_ref()
    }
    fn try_convert_copy_get_mut(object: &mut Box<Obj>) -> Option<&mut Self>
    where Box<Obj>: Copy
    {
        if !Self::try_convert_copy(object)
        {
            return None
        }
        todo!()//(&mut **object).downcast_mut()
    }
}