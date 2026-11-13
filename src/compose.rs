use std::collections::HashMap;

use crate::model::{Candidate, Kind};
use crate::weights::Weights;

/// Fills the feed position by position from candidates sorted best first.
/// Inside each block: at most `max_per_author` per author and `max_per_kind`
/// per kind, and the discovery positions go to entries no reason picked.
/// When nothing fits, the kind mix gives way first and the author limit last,
/// so the feed never comes out shorter than the candidates allow.
pub fn compose(ranked: Vec<&Candidate>, limit: usize, weights: &Weights) -> Vec<String> {
    let (mut discovery, mut regular): (Vec<_>, Vec<_>) = ranked
        .into_iter()
        .partition(|candidate| candidate.is_discovery());
    let mut out = Vec::new();
    let mut authors: HashMap<&str, usize> = HashMap::new();
    let mut kinds: HashMap<Kind, usize> = HashMap::new();

    while out.len() < limit && !(discovery.is_empty() && regular.is_empty()) {
        let position = out.len() % weights.block_size;
        if position == 0 {
            authors.clear();
            kinds.clear();
        }
        let author_fits = |candidate: &&Candidate| {
            authors.get(candidate.author.as_str()).copied().unwrap_or(0) < weights.max_per_author
        };
        let fits = |candidate: &&Candidate| {
            author_fits(candidate)
                && kinds.get(&candidate.kind).copied().unwrap_or(0) < weights.max_per_kind
        };
        let wants_discovery = weights.discovery_positions.contains(&position);
        let picked = wants_discovery
            .then(|| take(&mut discovery, &fits))
            .flatten()
            .or_else(|| take(&mut regular, &fits))
            .or_else(|| take(&mut regular, &author_fits))
            .or_else(|| take(&mut discovery, &author_fits))
            .or_else(|| take(&mut regular, &|_| true))
            .or_else(|| take(&mut discovery, &|_| true));
        let Some(candidate) = picked else { break };
        *authors.entry(candidate.author.as_str()).or_default() += 1;
        *kinds.entry(candidate.kind).or_default() += 1;
        out.push(candidate.key.clone());
    }
    out
}

fn take<'a>(
    pool: &mut Vec<&'a Candidate>,
    fits: &dyn Fn(&&'a Candidate) -> bool,
) -> Option<&'a Candidate> {
    let index = pool.iter().position(fits)?;
    Some(pool.remove(index))
}
