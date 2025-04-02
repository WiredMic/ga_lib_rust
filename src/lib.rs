// ga_lib is a rust library that implements different geometric algbras.
// Copyright (C) 2025 Rasmus Enevoldsen
//
// This file is part of ga_lib.
//
// ga_lib is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// ga_lib is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more
// details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with ga_lib. If not, see <https://www.gnu.org/licenses/>.

#![no_std]

//! > Geometry without algebra is dumb! Algebra without geometry is blind!
//!
//! >\- David Hestenes
//!
//! This is a create the expolore the world of geometric algebra.
//! It was build to be the backend of my rotation donut project.
//! I also wanted to see if I could use GA in the attitude control of a satellite.
//!
//! # Products
//! This library implements different geometric algebra products using Rust operations
//!
//! ## Addition
//! Addition is implemented with the use of the `+` (Add) operator.
//!
//! ## Subtraction
//! Subtraction is implemented with the use of the `-` (Sub) operator.
//!
//! ## Inner Product
//! The inner product $\cdot$ is implemented with the use of the `|` (BitOr) Operator.
//!
//! ## Exterior Product
//! The exterior product $\wedge$ is implemented with the use of the `^` (BitXor) Operator.
//!
//! ## Geometric Product
//! The geometric product is implemented with the use of the `*` (Mul) Operator.
//!
//! ## Regressive Product
//! The regressive product is implemented with the use of the `&` (BitAnd) Operator.
//!
//! TODO Not implemented
//!
//! # Operations
//! There is implemented different geometric operations.
//! They apply to all grades. Rotation only take rotors. The others take all grades except rotors.
//!
//! ## Rotation
//!
//!
//! ## Projection
//!
//! ## Rejection
//!
//! ## Reflection

/// Vector Geometric Algebra $\text{Cl}(3,0,0)$
pub mod vga3d;

#[macro_use]
pub(crate) mod macros;
