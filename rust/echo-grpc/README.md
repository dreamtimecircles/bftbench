# `echo-grpc`

An example that includes an echo gRPC streaming server and a minimal benchmark binding.

## Benchmark

In one terminal, run:

```bash
cargo run --release --bin echo-grpc-server -- --config echo-grpc/Server.example.toml
```

In a second terminal:

```bash
cargo run --release --bin bft-bench-echo -- --config echo-grpc/Benchmark.example.toml
```
