use std::sync::Arc;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum PromptError
{
    #[error("Text prompt failed.")]
    StdInError(Arc<std::io::Error>)
}