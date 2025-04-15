# `bftbench`

A benchmark framework for BFT libraries and platforms using modern programming languages.

BFT libraries, platforms and frameworks like [`bftgrid`] are often written in Rust,
Go, C++ or a JVM language, and it is often more convenient and performant
to integrate them in the same ecosystem, especially when they don't offer IPC
nor a platform-independent wire format.

A lean integration is even more important for benchmarks, that should not introduce
overhead and should thus avoid bridges.

At the moment, only a [Rust version](./rust) of the framework (with examples) is available.

[`bftgrid`]: https://github.com/dreamtimecircles/bftgrid

## Docs

The `rustdoc` reference including an index can be found at the [GitHub Pages site];
look for the relevant modules, at the moment [`bft-bench-core`], [`bft-bench-shortcircuit`],
[`bft-bench-echo`] and [`echo-grpc-server`].

[GitHub Pages site]: https://dreamtimecircles.github.io/bftbench
[`bft-bench-core`]: https://dreamtimecircles.github.io/bftbench/bft_bench_core
[`bft-bench-shortcircuit`]: https://dreamtimecircles.github.io/bftbench/bft_bench_shortcircuit
[`bft-bench-echo`]: https://dreamtimecircles.github.io/bftbench/bft_bench_echo
[`echo-grpc-server`]: https://dreamtimecircles.github.io/bftbench/echo_grpc_server

## Development

This repository uses [Nix] flakes and [direnv] to provide a reproducible development environment.

[Nix]: https://nixos.org
[direnv]: https://direnv.net
