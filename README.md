

# `bftbench`

An exercise in writing a benchmark framework for BFT libraries and platforms using modern programming languages.

BFT libraries and platforms are often written in Rust, Go, C++ or a JVM language, and it is often more convenient and performant to integrate them in the same ecosystem, especially when they don't offer IPC nor a platform-independent wire format.

A lean integration is even more important for benchmarks, that should not introduce overhead and should thus avoid bridges.

At the moment, only a [Rust version ](./rust) of the framework and a short-circuited binding are available.
