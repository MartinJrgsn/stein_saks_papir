use super::*;

pub enum NameError
{
    Taken,
    Invalid,
    Other
}

pub trait UIRps: UI
{
    fn promt_for_name(self: &mut Self, is_valid: Option<&dyn Fn(&str) -> Option<NameError>>) -> Result<String, PromtError>;
    fn promt_for_choice(self: &mut Self, player: &HumanRps, actor: &dyn SessionRpsObj, choice_log: &[[Choice; 2]]) -> Result<Option<PlayerDecision>, PlayerDecisionError>;
    fn on_quit(self: &mut Self);
}

impl UIRps for TUI
{
    fn promt_for_name(self: &mut Self, is_valid: Option<&dyn Fn(&str) -> Option<NameError>>) -> Result<String, PromtError>
    {
        println!("Enter player name:");
        let input = self.await_input()?;
        let name = input.trim();

        if let Some(is_valid_function) = is_valid
        {
            if let Some(error) = is_valid_function(name)
            {
                match error
                {
                    NameError::Taken => println!("The name \"{}\" is already taken!", name),
                    NameError::Invalid => println!("The name \"{}\" is invalid!", name),
                    NameError::Other => ()
                }
                println!("Please choose a different name.");
                return self.promt_for_name(is_valid)
            }
        }

        Ok(name.to_string())
    }
    fn promt_for_choice(self: &mut Self, player: &HumanRps, actor: &dyn SessionRpsObj, choice_log: &[[Choice; 2]]) -> Result<Option<PlayerDecision>, PlayerDecisionError>
    {
        let players = actor.get_players_or_wait();
        for mem in choice_log {
            println!(
                "Previous choices: \n{0}: {1}, {2}: {3}",
                players[0].get_name(),
                mem[0],
                players[1].get_name(),
                mem[1]
            );
        }

        println!("{0} choose:", player.get_name());
        println!(" - [1], [R]ock");
        println!(" - [2], [P]aper");
        println!(" - [3], [S]cissor");
        println!(" - [Q]uit");

        let input = self.await_input()?.trim().to_lowercase();
        match &input as &str
        {
            "r" | "1" => Ok(Some(PlayerDecision::Choose(Choice::Rock))),
            "p" | "2" => Ok(Some(PlayerDecision::Choose(Choice::Paper))),
            "s" | "3" => Ok(Some(PlayerDecision::Choose(Choice::Scissor))),
            "q" => Ok(Some(PlayerDecision::Quit)),
            _ => Ok(None),
        }
    }
    fn on_quit(self: &mut Self)
    {
        println!("Quit game!")
    }
}