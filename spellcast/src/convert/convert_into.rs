pub trait ConvertInto<To>
where
    To: ?Sized
{
    fn convert_into(self: Box<Self>) -> Box<To>;
}