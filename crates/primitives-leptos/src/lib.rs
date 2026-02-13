//! # primitives-leptos
//!
//! Leptos rendering layer for the UI component library. This crate provides
//! Leptos components that use `primitives-core` for shared enums, class
//! constants, and error types.
//!
//! ## Usage
//!
//! ```rust
//! use primitives_leptos::prelude::*;
//! ```

pub mod components;

pub use components::*;

pub mod prelude {
    pub use super::*;
}
