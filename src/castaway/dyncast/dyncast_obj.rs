pub mod try_as_dyncast_obj;
pub mod try_into_dyncast_obj;
pub mod as_dyncast_obj;
pub mod into_dyncast_obj;

pub use try_as_dyncast_obj::*;
pub use try_into_dyncast_obj::*;
pub use as_dyncast_obj::*;
pub use into_dyncast_obj::*;

use super::*;

pub trait DyncastObj<To>
where
    To: ?Sized
{
    type Obj: IsObjOf<Self> + Unsized;
}