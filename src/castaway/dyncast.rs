pub mod dyncast_obj;
pub mod dyncast_ref;
pub mod try_dyncast;

pub use dyncast_obj::*;
pub use dyncast_ref::*;
pub use try_dyncast::*;

use super::*;

pub trait Dyncast<To>: DyncastRef<To>
{
    fn dyncast(self: Box<Self>) -> Box<To>;
}
impl<From, To> Dyncast<To> for From
where
    From: DyncastRef<To> + IntoDyncastObj<To>,
    dyn Upcast<To>: Upcast<To>
{
    fn dyncast(self: Box<Self>) -> Box<To>
    {
        self.into_dyncast_obj().upcast()
    }
}