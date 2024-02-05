use std::sync::Arc;

use crate::error::PromptError;

use super::UIObj;

pub trait TUIObj: UIObj
{
    fn await_input(&mut self) -> Result<String, PromptError>
    {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(|error| PromptError::StdInError(Arc::new(error)))?;
        Ok(input)
    }
}