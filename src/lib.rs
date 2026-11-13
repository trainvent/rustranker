//! Orders StimmApp's personal feed ("Für dich", level 2).
//!
//! Pure and deterministic: candidates and the reader's signals in, an order
//! out. No database, no network. Same input, same output.

pub mod compose;
pub mod model;
pub mod score;
pub mod weights;

use wasm_bindgen::prelude::*;

pub use model::{Candidate, Kind, RankInput, Reason};
pub use weights::Weights;

/// The keys of [input]'s candidates in feed order, at most `limit`.
pub fn rank(input: &RankInput) -> Vec<String> {
    rank_with(input, &Weights::default())
}

pub fn rank_with(input: &RankInput, weights: &Weights) -> Vec<String> {
    let mut scored: Vec<(f64, &Candidate)> = input
        .candidates
        .iter()
        .map(|candidate| (score::score(candidate, input, weights), candidate))
        .collect();
    scored.sort_by(|(a, left), (b, right)| b.total_cmp(a).then_with(|| left.key.cmp(&right.key)));
    compose::compose(
        scored.into_iter().map(|(_, candidate)| candidate).collect(),
        input.limit,
        weights,
    )
}

/// [rank] on JSON: a `RankInput` in, an array of keys out.
pub fn rank_json_str(input: &str) -> Result<String, String> {
    let input: RankInput = serde_json::from_str(input).map_err(|error| error.to_string())?;
    serde_json::to_string(&rank(&input)).map_err(|error| error.to_string())
}

/// The WebAssembly entry point used by StimmApp's Cloud Function.
#[wasm_bindgen(js_name = rank)]
pub fn rank_wasm(input: &str) -> Result<String, JsError> {
    rank_json_str(input).map_err(|error| JsError::new(&error))
}
