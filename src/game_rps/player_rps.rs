use super::*;

pub enum PlayerDecision
{
    Choose(Choice),
    Undecided,
    Quit
}

pub trait PlayerRps: Player
{
    fn make_decision(self: &mut Self, actor: &mut ActorServer<2>, choice_log: &[[Choice; 2]]) -> Result<PlayerDecision, PlayerDecisionError>;
}

impl PlayerRps for Human
{
    fn make_decision(self: &mut Self, actor: &mut ActorServer<2>, choice_log: &[[Choice; 2]]) -> Result<PlayerDecision, PlayerDecisionError>
    {
        if self.is_local(actor)
        {
            let players = actor.get_players_or_wait()?;
            for mem in choice_log {
                println!(
                    "Previous choices: \n{0}: {1}, {2}: {3}",
                    players[0].get_name(),
                    mem[0],
                    players[1].get_name(),
                    mem[1]
                );
            }
    
            let mut input = String::new();
            println!("{0} choose: Rock/Paper/Scissor:", self.get_name());
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let cin = input.trim();
            match cin 
            {
                "r" | "1" => Ok(PlayerDecision::Choose(Choice::Rock)),
                "p" | "2" => Ok(PlayerDecision::Choose(Choice::Paper)),
                "s" | "3" => Ok(PlayerDecision::Choose(Choice::Scissor)),
                "q" => Ok(PlayerDecision::Quit),
                _ => Ok(PlayerDecision::Undecided),
            }
        }
    }
}