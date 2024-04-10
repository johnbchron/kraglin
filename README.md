# kraglin

Kraglin is planned to be a rust-based, lightweight, lightning-fast replacement for Redis.

## Architecture

The central trait is `Backend`, which defines the `execute()` method, taking a `Command` which holds key names and `Value`s. By defining tests and benchmarks generically on the `Backend` trait, we allow for highly exchangeable backend implementations. We intend to do the same for the frontend, but this is not built yet because the project is young.
