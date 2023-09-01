//! A library for efficient movement interpolation using different random walk models.
//! Below is an overview of all important types and functions, as well as a short introduction
//! on how to use the library.
//!
//! # Walk Models
//! This library implements different random walk models which can be used to generate
//! random walks with different characteristics. Below is a list of the different models
//! including short descriptions.
//!
//! - [Simple random walk](kernel::simple_rw::SimpleRwGenerator): Generates completely random walks
//! - [Biased random walk](kernel::biased_rw::BiasedRwGenerator): Generates random walks which tend
//! into a specific direction, i.e. they have a bias
//! - [Correlated random walk](kernel::correlated_rw::CorrelatedRwGenerator): Generates random walks
//! which have a higher probability of taking the same direction as in the last step
//! - [Biased and correlated random walk](kernel::biased_correlated_rw::BiasedCorrelatedRwGenerator)
//! : Combines the biased random walk with the correlated random walk
//! - [Lévy walk](kernel::levy_walk::LevyWalkGenerator): Generates random walks which sometimes jump
//! a few steps at once
//!
//! Walk models are implemented in this library as so-called
//! [`KernelGenerator`](kernel::generator::KernelGenerator)s. What they do is essentially to
//! generate a probability [`Kernel`](kernel::Kernel) that models the corresponding random walk
//! model. Alternatively, kernels can be creates by hand using some custom probability distribution.
//! This can be done the easiest using the [`kernel!`] macro.
//!
//! # Dynamic Programs
//!
//! There are two different types of dynamic programs which compute the random walk probabilities.
//! They are listed below together with short descriptions.
//!
//! - [Simple dynamic program](dp::simple::SimpleDynamicProgram): A dynamic program that uses a
//! single kernel to compute the probabilities.
//! - [Multi dynamic program](dp::multi::MultiDynamicProgram): A dynamic program that uses multiple
//! kernels to compute the probabilities. This is for example required when using correlated
//! random walks.
//!
//! Dynamic programs are wrapped into the [`DynamicProgram`](dp::DynamicProgram) enum and must
//! implement the [`DynamicPrograms`](dp::DynamicPrograms) trait. They can be initialized using the
//! [`DynamicProgramBuilder`](dp::builder::DynamicProgramBuilder).
//!
//! # Walkers
//!
//! # Dataset functionality
//!
//! # Examples
//!

pub mod dataset;
pub mod dp;
pub mod kernel;
pub mod walk_analyzer;
pub mod walker;
