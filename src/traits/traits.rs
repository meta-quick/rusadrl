
use crate::model::error::OdrlError;
use crate::model::stateworld::StateWorld;
pub trait Validate {
    fn validate(&mut self) -> Result<(), OdrlError>;
}

pub trait OperandValue {
    fn value(&self, world: &mut StateWorld) -> Option<String>;
}