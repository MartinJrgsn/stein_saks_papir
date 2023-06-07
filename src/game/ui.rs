pub mod tui;

pub use tui::*;

pub enum NameError
{
    Taken,
    Invalid,
    Other
}

pub trait UI
{
    fn promt_for_name(self: &mut Self, is_valid: Option<&dyn Fn(&str) -> Option<NameError>>) -> String;
}