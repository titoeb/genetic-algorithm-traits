#![deny(rustdoc::missing_doc_code_examples)]
#![deny(missing_docs)]
//! # Genetic algorithms for solving TSPs.
//!
//! This crates contains utitlities to run genetic algorithms and solve Traveling Salesman Problems.

/// The individual is the trait for your single unit of the genetic algorithm. I can mutated, be crossovered
/// with other individuals and compute their fitness based on data.
mod individual;
/// The `Population`  trait is the container for a set of individuals and it can evolve and maintain your population.
/// You can select fittest individuals and fittest supopulation. It has a default implementation for the genetic algorithm
/// as well.
mod population;
// The `utils`-module contains utility that are used throughout the rest of the code base. The underlying `ordered_crossover`-
/// function is implemented here.
mod utils;

// Expose only `Individual`  as well as `Population` trait outside.
pub use individual::Individual;
pub use population::Population;
