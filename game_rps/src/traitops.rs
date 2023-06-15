//pub mod union;
pub mod intersection;

//pub use union::*;
pub use intersection::*;

use super::*;

/*#[macro_export]
macro_rules! new_intersection_trait {
    {$intersection_name:ident for $a:path | $b:path} => {
        trait $intersection_name: $a + $b {}
        impl<T> $intersection_name for T
        where T: $a + $b {}
        impl<T> Downcast<T> for dyn $union_name where T: $union_name {}
    };
}

#[macro_export]
macro_rules! new_union_trait {
    {$union_name:ident for $a:path | $b:path} => {
        trait $union_name {}
        impl<T> $union_name for T
        where T: $a {}
        impl<T> $union_name for T
        where T: $b {}
        impl<T> Downcast<T> for dyn $union_name where T: $union_name {}
    };
}
new_union_trait!{U for Add | Sub}*/