//! # primitives-yew
//!
//! Yew rendering layer for the UI component library. This crate provides
//! Yew components that use `primitives-core` for shared enums, class
//! constants, and error types.
//!
//! ## Usage
//!
//! ```rust
//! use primitives_yew::prelude::*;
//! ```

pub mod components;

pub use components::*;

pub mod prelude {
    pub use super::*;
}
