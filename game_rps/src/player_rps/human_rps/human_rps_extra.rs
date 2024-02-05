use game::player::HumanExtra;

use super::*;

#[derive(Default, Debug, Clone)]
pub struct HumanRpsExtra
{
    pub received_decision: Option<PlayerDecision>
}
impl HumanExtra for HumanRpsExtra
{

}