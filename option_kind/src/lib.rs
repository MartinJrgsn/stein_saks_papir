pub trait OptionKind
{
    type Some;
    fn into_option(self) -> Option<Self::Some>; 
}
impl<Some> OptionKind for Option<Some>
{
    type Some = Some;
    fn into_option(self) -> Option<Some>
    {
        self
    }
}