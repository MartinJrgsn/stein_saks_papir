use std::mem;
#[derive(Clone, Copy)] // clone explicit , copy implisit
#[repr(u8)]
pub enum Choice {
    Paper = 0,
    Rock = 1,
    Scissor = 2,
}

impl Choice {
    fn is_tie(self, oponents_choice : Choice) -> bool {
        return self as u8 == oponents_choice as u8//;
    }

    fn is_win(self, oponents_choice : Choice) -> bool {
        return (self as u8 + 1) % (Self::LENGTH as u8) == oponents_choice as u8//;
    }

    pub fn get_outcome(self, oponents_choice : Choice) -> Option<bool> {
        if self.is_win(oponents_choice) {
            Some(true)
        }
        else if self.is_tie(oponents_choice) {
            None
        }
        else {
            Some(false)
        }
    }
    
    pub const LENGTH : usize = 3;
    const VALUES : [Choice; Self::LENGTH] = [Choice::Paper, Choice::Rock, Choice::Scissor];
}

impl TryFrom<u8> for Choice {
    type Error = ();
    fn try_from(value : u8) -> Result<Choice, ()> {
        if value >= Choice::LENGTH as u8 {
            return Err(())
        }
        
        Ok(Choice::VALUES[value as usize])
    }
} 

pub struct Game {
    turn : bool,
    my_choice : Option<Choice>,
}

impl Game {
    pub fn new() -> Self {
        println!("got here");
        Self {
            turn : false,
            my_choice : Option::None,
        }
    }
}