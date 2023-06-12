use std::any::Any;

use super::*;

pub trait ConvertInto<To>
where
    To: ?Sized
{
    fn convert_into(self: Box<Self>) -> Box<To>;
}