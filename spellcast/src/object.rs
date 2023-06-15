use super::*;

pub trait Object<Trait> = private::Object<Trait>
where
    Trait: ?Sized;

#[macro_export]
macro_rules! impl_object {
    ($trait:path) => {
        impl<T> private::Is<dyn $trait> for T
        where
            T: $trait + std::marker::Unsize<dyn $trait> + ?Sized {}
        impl DowncastFromRef<dyn $trait> for dyn $trait
        {
            fn downcast_from_ref(from: &dyn $trait) -> Option<&Self>
            {
                Some(from)
            }
        
            fn downcast_from_mut(from: &mut dyn $trait) -> Option<&mut Self>
            {
                Some(from)
            }
        }
        impl DowncastFrom<dyn $trait, dyn $trait> for dyn $trait
        {
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
        impl<T> private::Object<dyn $trait> for T
        where
            T: $trait
            + ?Sized
            + Is<dyn $trait>
            + Upcast<dyn $trait>
            + Downcast<dyn $trait, dyn $trait>
        {
    
        }
        assert_is!(dyn $trait: $trait);
    };
}
#[macro_export]
macro_rules! assert_is {
    ($type:ty : $trait:path) => {
        use static_assertions::*;
        assert_obj_safe!(Object<dyn $trait>);
        assert_impl_one!(dyn HumanObj: Unsize<dyn PlayerObj>);
        assert_impl_one!(dyn PlayerObj: Unsize<dyn PlayerObj>);
        assert_impl_one!(dyn PlayerObj: Is<dyn PlayerObj>);
        assert_impl_one!(dyn HumanObj: Unsize<dyn HumanObj>);
        assert_impl_one!(dyn Any: Unsize<dyn Any>);
        assert_impl_one!(dyn $trait: Is<dyn $trait>);
        assert_impl_one!($type: Unsize<dyn $trait>);
        assert_impl_one!($type: Is<dyn $trait>);
        assert_impl_one!($type: Upcast<dyn $trait>);
        assert_impl_one!(dyn $trait: DowncastFrom<dyn $trait, dyn $trait>);
        assert_impl_one!(dyn $trait: DowncastFromRef<dyn $trait>);
        assert_impl_one!(dyn $trait: DowncastRef<dyn $trait>);
        assert_impl_one!(dyn $trait: Downcast<dyn $trait, dyn $trait>);
        assert_impl_one!(dyn $trait: Downcast<$type, dyn $trait>);
        assert_impl_one!(dyn $trait: Object<dyn $trait>);
    };
}