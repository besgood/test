// src/reputation/mod.rs

pub mod scoring;

pub use scoring::{calculate_reputation, ReputationScore};

pub struct ReputationEntry;
pub struct ReputationDB;
