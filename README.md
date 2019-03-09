# Rust Bio-Computation crate
This crate is a library that aims to provide some basic methods for bio-computation algorithms such GA, Ant colony optimizations.
The examples directory provides some basic examples showcasing each method.

## Examples
### Travelling salesman problem
This example aims to showcase the travelling salesman problem, there are 14 cities and the goal is to find the shorted path between them.
There are 14! possible routes, so we use a GA algorithm to find an approximate route in a reasonable amount of time.

To run use:
```bash
cargo run --example tsm
```