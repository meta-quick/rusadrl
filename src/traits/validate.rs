
use crate::model::error::OdrlError;
pub trait Validate {
    fn validate(&self) -> Result<(), OdrlError>;
}