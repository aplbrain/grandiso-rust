# grandiso-rust

Ultra-fast implementation of the queue-based GrandIso subgraph isomorphism algorithm.

## Benchmarks

Legit benchmarks forthcoming, but as a rough ballpark, counting triangles in a complete 200-graph takes ~40s in Python, and 10s in Rust.

All times in mm:ss.

| Size (K{n}) | Python | Rust    |
| ----------- | ------ | ------- |
| 50          | <0:00  | <0:00.1 |
| 100         | 0:04   | <0:01   |
| 200         | 0:40   | 0:10    |
| 400         | 4:51   | 1:33    |
