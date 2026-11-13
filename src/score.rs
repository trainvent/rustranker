use crate::model::{Candidate, RankInput, Reason};
use crate::weights::Weights;

/// How strongly [candidate] fits the reader right now. Discovery entries are
/// scored by freshness and popularity only; they compete among themselves.
pub fn score(candidate: &Candidate, input: &RankInput, weights: &Weights) -> f64 {
    let age = (input.now_ms - candidate.at_ms).max(0) as f64;
    let freshness = 0.5_f64.powf(age / weights.half_life_ms);
    let popularity = 1.0 + weights.popularity * (1.0 + candidate.popularity as f64).ln();
    let relevance = if candidate.is_discovery() {
        1.0
    } else {
        candidate
            .reasons
            .iter()
            .map(|reason| reason_weight(reason, input, weights))
            .sum()
    };
    freshness * popularity * relevance
}

fn reason_weight(reason: &Reason, input: &RankInput, weights: &Weights) -> f64 {
    let (base, author) = match reason {
        Reason::Topic => return weights.topic,
        Reason::Published { author } => (weights.published, author),
        Reason::Signed { author } => (weights.signed, author),
    };
    let ignored = input.ignores.get(author).copied().unwrap_or(0);
    base * weights.ignore_decay.powi(ignored as i32)
}
