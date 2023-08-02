moddef::moddef!(
    flat(pub) mod {
        dyncast_obj,
        dyncast_ref,
        dyncast_,
        try_dyncast_ref,
        try_dyncast
    }
);

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