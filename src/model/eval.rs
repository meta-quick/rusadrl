use crate::model::policy::{OdrlRequest};
use crate::model::stateworld::StateWorld;


pub trait Evaluator {
    fn eval(&self,world: &mut StateWorld, req: &OdrlRequest) -> Result<bool, anyhow::Error>;
}