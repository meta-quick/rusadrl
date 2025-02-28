use thiserror::Error;

#[derive(Error, Debug)]
pub enum OdrlError {
    #[error("Policy must have a valid IRI")]
    InvalidIri,
    #[error("Policy must have at least one rule, permission, prohibition, or obligation")]
    InvalidRuleDefinition,
    #[error("Policy not allow empty rule, at least one rule, permission, prohibition, or obligation")]
    NoneRuleDefinition,

    #[error("Without offer target")]
    MissingOfferTarget,

    #[error("Without offer assigner")]
    MissingOfferAssigner,

    #[error("Without agrement target")]
    MissingAgreementTarget,

    #[error("Without agreement assigner")]
    MissingAgreementAssigner,

    #[error("Without agreement assignee")]
    MissingAgreementAssignee,

    #[error("Asset must has a valid IRI")]
    InvalidAssetIRI,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}