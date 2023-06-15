use super::*;

pub trait TryConvertInto<To, Alt>: Is<Alt>
where
    To: ?Sized,
    Alt: ?Sized
{
    fn try_convert_into(self: Box<Self>) -> Result<Box<To>, Box<Alt>>;
}
/*default impl<From, To, Alt> TryConvertInto<To, Alt> for From
where
    From: ConvertInto<To> + Is<Alt> + ?Sized,
    To: ?Sized,
    Alt: ?Sized
{
    fn try_convert_into(self: Box<Self>) -> Result<Box<To>, Box<Alt>>
    {
        Ok(self.convert_into())
    }
}*/