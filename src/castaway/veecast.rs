use super::*;

pub trait Veecast<To, Struct>
where
    To: ?Sized
{
    fn veecast_ref(self: &Self) -> Option<&To>;
    fn veecast_mut(self: &mut Self) -> Option<&mut To>;
    fn veecast(self: Box<Self>) -> Result<Box<To>, Box<Self>>;
}
impl<From, To, Struct> Veecast<To, Struct> for From
where
    From: Downcast<Struct, From> + ?Sized,
    Struct: DowncastFrom<From> + Upcast<To> + Is<From>,
    To: ?Sized
{
    fn veecast_ref(self: &Self) -> Option<&To>
    {
        self.downcast_ref().map(|child| child.upcast_ref())
    }
    fn veecast_mut(self: &mut Self) -> Option<&mut To>
    {
        self.downcast_mut().map(|child| child.upcast_mut())
    }
    fn veecast(self: Box<Self>) -> Result<Box<To>, Box<Self>>
    {
        self.downcast().map(|child| child.upcast())
    }
}