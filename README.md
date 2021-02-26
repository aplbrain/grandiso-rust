# grandiso-rust

Ultra-fast implementation of the queue-based GrandIso subgraph isomorphism algorithm, implemented in [Rust](https://www.rust-lang.org/).

For the original Python implementation, see [here](https://github.com/aplbrain/grandiso-networkx).

This implementation relies upon the Rust [petgraph](https://github.com/petgraph/petgraph) library for core graph operations.

## Example Usage

```rust
use crate::grandiso;
use petgraph::graphmap::DiGraphMap;
```

```rust
// Create a directed triangle motif:
let mut graphmap: DiGraphMap<i8, &str> = DiGraphMap::new();
graphmap.add_edge(0, 1, "3");
graphmap.add_edge(1, 2, "3");
graphmap.add_edge(2, 0, "3");

// Create a directed triangle host graph:
let mut graphmap_host: DiGraphMap<&str, &str> = DiGraphMap::new();
graphmap_host.add_edge("A", "B", "3");
graphmap_host.add_edge("B", "C", "3");
graphmap_host.add_edge("C", "A", "3");

// Perform the search:
let results = grandiso::find_motifs(graphmap.clone(), graphmap_host.clone());

```

## Benchmarks

Legit benchmarks forthcoming, but as a rough ballpark, counting triangles in a complete 200-graph takes ~40s in Python, and 10s in Rust.

All times in mm:ss.

| Size (K{n}) | Python | Rust    |
| ----------- | ------ | ------- |
| 50          | <0:00  | <0:00.1 |
| 100         | 0:04   | <0:01   |
| 200         | 0:40   | 0:10    |
| 400         | 4:51   | 1:33    |
