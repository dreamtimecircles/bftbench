# `bftbench-rust`

An asynchronous, [Tokio]-based Rust BFT benchmarking framework and an example shortcircuited
binding that uses channels.

Run with:

```bash
cargo run --bin bft-bench-shortcircuit -- --config bft-bench-shortcircuit/Benchmark.example.toml
```

An example output follows:

```
[2023-02-08T07:17:11.444625Z INFO ThreadId(1) bft_bench_shortcircuit bft-bench-shortcircuit/src/bin/bft-bench-shortcircuit.rs:48] 'target/debug/bft-bench-shortcircuit' starting, configuration loaded: Config { run_duration: 5s, write_interval: 1s, transaction_size: 100, nodes: [ReadWrite(ReadWriteNode { node: WriteNode { endpoint: NodeEndpoint { host: "localhost", port: 4000 } } }), Write(WriteNode { endpoint: NodeEndpoint { host: "localhost", port: 4000 } })] }
[2023-02-08T07:17:11.536682Z INFO ThreadId(1) bft_bench_shortcircuit bft-bench-shortcircuit/src/bin/bft-bench-shortcircuit.rs:56] Starting benchmark
[2023-02-08T07:17:11.537216Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:457] Starting readers
[2023-02-08T07:17:11.537270Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:462] Starting reader for node 0
[2023-02-08T07:17:11.537361Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:91] Read nodes: {0}
[2023-02-08T07:17:11.537403Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:429] Starting writers
[2023-02-08T07:17:11.537448Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:439] Starting writer for node 0
[2023-02-08T07:17:11.537528Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:439] Starting writer for node 1
[2023-02-08T07:17:11.537601Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:101] The benchmark will be run for 5 seconds
[2023-02-08T07:17:16.539915Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:128] Benchmark duration elapsed, requesting readers' and writers' completion
[2023-02-08T07:17:16.540013Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:479] Signalling workers to stop
[2023-02-08T07:17:16.545532Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:236] Writer 1 completed
[2023-02-08T07:17:16.545657Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:238] Writers to go: {0}
[2023-02-08T07:17:16.545689Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:239] Readers to go: {0}
[2023-02-08T07:17:16.545716Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:236] Writer 0 completed
[2023-02-08T07:17:16.545787Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:238] Writers to go: {}
[2023-02-08T07:17:16.545859Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:239] Readers to go: {0}
[2023-02-08T07:18:15.546178Z INFO ThreadId(1) bft_bench_core bft-bench-core/src/lib.rs:347] Reader 0 completed
[2023-02-08T07:18:15.575923Z INFO ThreadId(1) bft_bench_shortcircuit bft-bench-shortcircuit/src/bin/bft-bench-shortcircuit.rs:58] Benchmark completed, stats follow: {"global_write_stats":{"writes_successful_nanos_avg":{"Ok":47774},"writes_failed_nanos_avg":{"Err":"no data"}},"global_read_stats":{"read_successful_nanos_avg":{"Ok":400767750},"read_failed_nanos_avg":{"Err":"no data"},"roundtrip_nanos_avg":{"Ok":317749}},"nodes_write_stats":[{"writes_successful_nanos_avg":{"Ok":53717},"writes_failed_nanos_avg":{"Err":"no data"}},{"writes_successful_nanos_avg":{"Ok":41831},"writes_failed_nanos_avg":{"Err":"no data"}}],"nodes_read_stats":[{"read_successful_nanos_avg":{"Ok":400767750},"read_failed_nanos_avg":{"Err":"no data"},"roundtrip_nanos_avg":{"Ok":317749}}]}
```

[tokio]: https://tokio.rs/
