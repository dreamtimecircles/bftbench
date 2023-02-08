# `bftbench`

BFT libraries and platforms are frequently written in C++, Rust, Go or a JVM language and
it is often more convenient and performant to integrate them on the same platform,
especially when they don't offer a IPC or platform-independent wire format.

This is an exercise in using modern programming languages to write a benchmark framework
for BFT libraries and platforms.

At the moment, only a [Rust version ](./rust) of the framework and a short-circuited binding
are available, but at least Go and Kotlin versions are planned as well.
