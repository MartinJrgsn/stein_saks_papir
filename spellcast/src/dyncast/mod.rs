pub mod dyncast_obj;
pub mod dyncast_ref;
pub mod dyncast;
pub mod try_dyncast_ref;
pub mod try_dyncast;

pub use dyncast_obj::*;
pub use dyncast_ref::*;
pub use dyncast::*;
pub use try_dyncast_ref::*;
pub use try_dyncast::*;

use super::*;

#[macro_export]
macro_rules! dyncast_impl {
    ($sub:path : $super:path) => {
        impl<Medium> DyncastObj<dyn $sub, dyn $super> for Medium
        where
            Medium: Is<dyn $sub> + Is<dyn $super> + ?Sized,
        {
            type Obj = dyn $sub;
        }
        impl<Medium> DyncastObj<dyn $super, dyn $sub> for Medium
        where
            Medium: Is<dyn $sub> + Is<dyn $super> + ?Sized,
        {
            type Obj = dyn $sub;
        }
    };
}