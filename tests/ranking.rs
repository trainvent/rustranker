use std::collections::BTreeMap;

use rustranker::{Candidate, Kind, RankInput, Reason, rank, rank_json_str};

const NOW: i64 = 1_800_000_000_000;
const HOUR: i64 = 60 * 60 * 1000;

fn candidate(
    key: &str,
    kind: Kind,
    author: &str,
    hours_ago: i64,
    reasons: Vec<Reason>,
) -> Candidate {
    Candidate {
        key: key.into(),
        kind,
        author: author.into(),
        at_ms: NOW - hours_ago * HOUR,
        reasons,
        popularity: 0,
    }
}

fn published(author: &str) -> Vec<Reason> {
    vec![Reason::Published {
        author: author.into(),
    }]
}

fn input(candidates: Vec<Candidate>, limit: usize) -> RankInput {
    RankInput {
        now_ms: NOW,
        candidates,
        ignores: BTreeMap::new(),
        limit,
    }
}

#[test]
fn newer_entries_come_first_when_reasons_are_equal() {
    let order = rank(&input(
        vec![
            candidate("old", Kind::Petition, "a", 48, vec![Reason::Topic]),
            candidate("new", Kind::Poll, "b", 1, vec![Reason::Topic]),
        ],
        10,
    ));
    assert_eq!(order, ["new", "old"]);
}

#[test]
fn a_supported_author_outweighs_a_topic_of_the_same_age() {
    let order = rank(&input(
        vec![
            candidate("topic", Kind::Petition, "a", 5, vec![Reason::Topic]),
            candidate("author", Kind::Poll, "b", 5, published("b")),
        ],
        10,
    ));
    assert_eq!(order, ["author", "topic"]);
}

#[test]
fn ignored_entries_lower_an_author_instead_of_dropping_them() {
    let mut ranking = input(
        vec![
            candidate("anna", Kind::Petition, "anna", 5, published("anna")),
            candidate("topic", Kind::Poll, "b", 5, vec![Reason::Topic]),
        ],
        10,
    );
    ranking.ignores.insert("anna".into(), 3);
    assert_eq!(rank(&ranking), ["topic", "anna"]);
}

#[test]
fn at_most_two_per_author_in_every_ten() {
    let mut candidates: Vec<_> = (0..5)
        .map(|i| {
            candidate(
                &format!("anna{i}"),
                Kind::Petition,
                "anna",
                i,
                published("anna"),
            )
        })
        .collect();
    candidates.extend((0..8).map(|i| {
        candidate(
            &format!("other{i}"),
            Kind::Poll,
            &format!("o{i}"),
            50 + i,
            vec![Reason::Topic],
        )
    }));
    let order = rank(&input(candidates, 10));
    assert_eq!(
        order.iter().filter(|key| key.starts_with("anna")).count(),
        2
    );
    assert_eq!(&order[..2], ["anna0", "anna1"]);
}

#[test]
fn every_ten_mixes_the_kinds() {
    let mut candidates: Vec<_> = (0..12)
        .map(|i| {
            candidate(
                &format!("p{i:02}"),
                Kind::Petition,
                &format!("a{i}"),
                i,
                vec![Reason::Topic],
            )
        })
        .collect();
    candidates.push(candidate("poll", Kind::Poll, "x", 90, vec![Reason::Topic]));
    candidates.push(candidate(
        "survey",
        Kind::Survey,
        "y",
        95,
        vec![Reason::Topic],
    ));
    // Much older than every petition, yet right after the first five of them.
    let order = rank(&input(candidates, 10));
    assert_eq!(
        &order[..7],
        ["p00", "p01", "p02", "p03", "p04", "poll", "survey"]
    );
    assert_eq!(
        order.len(),
        10,
        "the mix gives way rather than cut the feed short"
    );
}

#[test]
fn popular_local_entries_fill_the_fifth_and_tenth_place() {
    let mut candidates: Vec<_> = (0..10)
        .map(|i| {
            let kind = [Kind::Petition, Kind::Poll, Kind::Survey][i as usize % 3];
            candidate(
                &format!("r{i}"),
                kind,
                &format!("a{i}"),
                i,
                vec![Reason::Topic],
            )
        })
        .collect();
    let mut quiet = candidate("quiet", Kind::Petition, "q", 1, vec![]);
    quiet.popularity = 2;
    let mut popular = candidate("popular", Kind::Poll, "p", 1, vec![]);
    popular.popularity = 900;
    candidates.extend([quiet, popular]);
    let order = rank(&input(candidates, 10));
    assert_eq!(order[4], "popular");
    assert_eq!(order[9], "quiet");
}

#[test]
fn without_discovery_entries_the_feed_stays_full() {
    let candidates = (0..4)
        .map(|i| {
            candidate(
                &format!("r{i}"),
                Kind::Petition,
                &format!("a{i}"),
                i,
                vec![Reason::Topic],
            )
        })
        .collect();
    assert_eq!(rank(&input(candidates, 10)), ["r0", "r1", "r2", "r3"]);
}

#[test]
fn rules_give_way_when_nothing_else_is_left() {
    let candidates = (0..4)
        .map(|i| {
            candidate(
                &format!("anna{i}"),
                Kind::Petition,
                "anna",
                i,
                published("anna"),
            )
        })
        .collect();
    assert_eq!(rank(&input(candidates, 10)).len(), 4);
}

#[test]
fn equal_scores_are_ordered_by_key_and_the_limit_holds() {
    let candidates = ["c", "a", "b"]
        .iter()
        .map(|key| candidate(key, Kind::Petition, key, 2, vec![Reason::Topic]))
        .collect();
    assert_eq!(rank(&input(candidates, 2)), ["a", "b"]);
}

#[test]
fn json_in_json_out() {
    let order = rank_json_str(
        r#"{"nowMs": 1800000000000, "limit": 5, "ignores": {"anna": 1},
            "candidates": [
              {"key": "petition:a", "kind": "petition", "author": "anna", "atMs": 1799990000000,
               "reasons": [{"kind": "signed", "author": "anna"}, {"kind": "topic"}], "popularity": 12},
              {"key": "poll:b", "kind": "poll", "author": "ben", "atMs": 1799000000000}
            ]}"#,
    )
    .unwrap();
    assert_eq!(order, r#"["petition:a","poll:b"]"#);
    assert!(rank_json_str("{}").is_err());
}
