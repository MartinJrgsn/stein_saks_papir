use std::fmt::Debug;

pub trait HumanExtra: Sized + Send + Sync + Default + Debug + 'static
{

}
impl HumanExtra for ()
{
    
}