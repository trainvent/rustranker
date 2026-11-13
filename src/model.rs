use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Everything the ranker needs; the caller has already removed what the
/// reader may not or does not want to see.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RankInput {
    pub now_ms: i64,
    pub candidates: Vec<Candidate>,
    /// Entries ignored in a row, per supported author.
    #[serde(default)]
    pub ignores: BTreeMap<String, u32>,
    pub limit: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    /// Unique per publication, e.g. `petition:abc`; also the final tie-breaker.
    pub key: String,
    pub kind: Kind,
    /// Who created the publication.
    pub author: String,
    /// When it was published or, for a signature, signed.
    pub at_ms: i64,
    /// Why it is offered. Empty for a popular local entry that no reason
    /// picked, which keeps the feed from closing into a bubble.
    #[serde(default)]
    pub reasons: Vec<Reason>,
    /// Signatures, votes or responses so far.
    #[serde(default)]
    pub popularity: u32,
}

impl Candidate {
    pub fn is_discovery(&self) -> bool {
        self.reasons.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Petition,
    Poll,
    Survey,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Reason {
    /// Tagged with a topic the reader follows.
    Topic,
    /// Published by an author the reader supported.
    Published { author: String },
    /// Publicly signed by an author the reader supported.
    Signed { author: String },
}
