// Copyright 2024 meduo <gao.brian@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused)]

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

    #[error("Rule must has a valid action")]
    MissingAction,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}