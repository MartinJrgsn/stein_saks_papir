pub trait Boxed<'a>
{
    type Ref;
    type RefMut;
}

impl<'a, T> Boxed<'a> for Box<T>
where
    T: ?Sized + 'a
{
    type Ref = &'a T;
    type RefMut = &'a mut T;
}