use std::fmt::Debug;

pub trait HumanExtra: Sized + Send + Sync + Default + Debug + Clone + 'static
{

}
impl HumanExtra for ()
{
    
}