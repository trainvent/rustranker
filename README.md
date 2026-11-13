# rustranker

The ranker behind the "Für dich" feed in [StimmApp](https://stimmapp.net), a
platform for petitions, polls and surveys. It is open so that anyone can check
how the feed is ordered.

It is pure and deterministic: candidates and the reader's signals in, an order
out. It never sees a database, the network or anything the reader is not
allowed to see; StimmApp filters that out before calling it.

## How it orders

Each candidate gets a score:

- **Freshness** halves every 3 days.
- **Reasons** add up. A followed topic counts 1, a new publication by an
  author the reader supported 1.5, a public signature by such an author 1.
- **Ignored entries** lower an author: every entry the reader scrolled past in
  a row multiplies that author's reasons by 0.6.
- **Popularity** adds a little: `1 + 0.1 × ln(1 + signatures, votes or answers)`.

The feed is then filled in blocks of 10:

- at most 2 entries from the same author;
- at most 5 of the same kind (petition, poll, survey);
- places 5 and 10 go to popular local entries that no reason picked, so the
  feed does not close into a bubble.

When nothing fits, the kind mix gives way first and the author limit last, so
the feed is never shorter than the candidates allow. Equal scores are ordered
by key. All numbers live in [`src/weights.rs`](src/weights.rs).

## Input and output

```json
{
  "nowMs": 1800000000000,
  "limit": 30,
  "ignores": {"anna": 2},
  "candidates": [
    {
      "key": "petition:abc",
      "kind": "petition",
      "author": "anna",
      "atMs": 1799990000000,
      "reasons": [{"kind": "signed", "author": "anna"}, {"kind": "topic"}],
      "popularity": 12
    }
  ]
}
```

returns the keys in feed order: `["petition:abc"]`. A candidate without
`reasons` is a popular local entry.

## Use

- Rust: `rustranker::rank(&input)`.
- Node: `./build-wasm.sh`, then `require("./pkg/rustranker.js").rank(json)`.
- Tests: `cargo test`.

## License

[AGPL-3.0-only](LICENSE). See [CONTRIBUTING.md](CONTRIBUTING.md) before
sending changes.
