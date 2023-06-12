use super::*;

use std::{any::Any};

pub trait Is<Trait>
where
    Trait: ?Sized
{
    
}
pub macro Is($trait:path)
{
    impl<T> Is<dyn $trait> for T where T: $trait + ?Sized {}
    impl DowncastFrom<dyn $trait> for dyn $trait
    {
        fn is(from: &dyn $trait) -> bool
        {
            true
        }
    
        fn downcast_from_ref(from: &dyn $trait) -> Option<&Self>
        {
            Some(from)
        }
    
        fn downcast_from_mut(from: &mut dyn $trait) -> Option<&mut Self>
        {
            Some(from)
        }
    
        fn downcast_from(from: Box<dyn $trait>) -> Result<Box<Self>, Box<dyn $trait>>
        {
            Ok(from)
        }
    }
    impl TryConvertInto<dyn $trait, dyn $trait> for dyn $trait
    {
        fn try_convert_into(self: Box<Self>) -> Result<Box<dyn $trait>, Box<dyn $trait>>
        {
            Ok(self)
        }
    }
}
Is!(Any);