use super::*;

pub trait Is<Trait> = private::Is<Trait>
where
    Trait: ?Sized;
pub trait IsObjOf<Type>: private::IsObjOf<Type>
where
    Type: ?Sized {}
impl<Type, Trait> IsObjOf<Type> for Trait
where
    Type: Is<Trait> + ?Sized,
    Trait: ?Sized {}
pub trait IsImplOf<Obj>: Is<Obj>
where
    Obj: ?Sized
{
    fn is(object: &Obj) -> bool
    where Self: Sized;
}
impl<Struct, Obj> IsImplOf<Obj> for Struct
where
    Struct: AsAny + Is<Obj> + ?Sized,
    Obj: ?Sized
{
    fn is(object: &Obj) -> bool
    where Self: Sized
    {
        object.as_any().is::<Struct>()
    }
}