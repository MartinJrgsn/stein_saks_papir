use super::*;

#[derive(Clone)] 
pub struct Human 
{
    uid: Port,
    name: String
}

impl Human 
{
    pub const MEMORY_LENGTH : usize = 5;

    pub fn new(uid: Port, name: String) -> Self
    {
        Self
        {
            uid,
            name
        }
    }

    pub fn get_uid(&self) -> Port
    {
        self.uid
    }
}

impl Player for Human
{
    fn as_human(self: &Self) -> Option<&Human>
    {
        Some(self)
    }
    fn as_human_mut(self: &mut Self) -> Option<&mut Human>
    {
        Some(self)
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}