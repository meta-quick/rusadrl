
use crate::model::error::OdrlError;
pub trait Validate {
    fn validate(&mut self) -> Result<(), OdrlError>;
}