# `bftbench`

This is mostly an exercise in using modern programming languages to write a benchmark framework for BFT libraries and platforms, that are frequently written in C++, Rust, Go or a JVM language.

In addition it is often more convenient and performant to integrate them on the same platform, especially when they don't offer IPC nor platform-independent wire format. A lean integration is even more important for benchmarks, that should avoid introducing any overhead and should thus avoid bridge components.

At the moment, only a [Rust version ](./rust) of the framework and a short-circuited binding are available, but at least Go and Kotlin versions are planned as well.
