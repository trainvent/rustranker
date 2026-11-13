/// Every number that shapes the order, in one place.
#[derive(Debug, Clone)]
pub struct Weights {
    /// After this long an entry's freshness has halved.
    pub half_life_ms: f64,
    pub topic: f64,
    pub published: f64,
    pub signed: f64,
    /// Each ignored entry in a row multiplies an author's reasons by this.
    pub ignore_decay: f64,
    /// How much `ln(1 + popularity)` adds to an entry's score.
    pub popularity: f64,
    /// Composition rules apply to each block of this many entries.
    pub block_size: usize,
    pub max_per_author: usize,
    pub max_per_kind: usize,
    /// Positions inside a block (0-based) kept for popular local entries
    /// that no reason picked.
    pub discovery_positions: Vec<usize>,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            half_life_ms: 3.0 * 24.0 * 60.0 * 60.0 * 1000.0,
            topic: 1.0,
            published: 1.5,
            signed: 1.0,
            ignore_decay: 0.6,
            popularity: 0.1,
            block_size: 10,
            max_per_author: 2,
            max_per_kind: 5,
            discovery_positions: vec![4, 9],
        }
    }
}
