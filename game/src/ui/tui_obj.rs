use crate::PromtError;

pub trait TUIObj
{
    fn await_input(&mut self) -> Result<String, PromtError>
    {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(|error| PromtError::StdInError(error))?;
        Ok(input)
    }
}