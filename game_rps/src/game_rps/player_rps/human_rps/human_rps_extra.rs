use super::*;

#[derive(Default, Debug)]
pub struct HumanRpsExtra
{
    pub(super) received_decision: Option<PlayerDecision>
}
impl HumanExtra for HumanRpsExtra
{

}