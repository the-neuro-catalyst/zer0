//! # schema
//!
//! Core schema definitions and merging logic for the ZERO data pipeline.
//! This crate serves as the Single Source of Truth for data types across the stack.

pub mod ops;
pub mod types;

// Re-export core types for backward compatibility and ease of use

pub use ops::merger::{UnionValue, merge_data_types, merge_types};

pub use types::data_type::{DataType, Schema};

pub use types::passion::PassionTensor;

pub use types::schema::SchemaValue;
pub mod validation;
