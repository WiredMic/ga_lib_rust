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

#![allow(dead_code)]
#![allow(unused_imports)]

use num_traits::Float;

mod scalar;
pub use scalar::Scalar;
//
mod vector;
pub use vector::Vector;
//
mod bivector;
pub use bivector::Bivector;
//
mod trivector;
pub use trivector::Trivector;
//
mod multivector;
pub use multivector::Multivector;
//
mod rotor;
pub use rotor::Rotor;
//
//
#[cfg(feature = "quaternion")]
mod quaternion;
#[cfg(feature = "quaternion")]
pub use quaternion::Quaternion;
//
#[cfg(feature = "quaternion")]
pub use quaternion::UnitQuaternion;

// Interactions
mod addition;
mod exterior;
mod geometric;
mod inner;
mod regressive;
mod subtraction;
// Functions
mod functions;
pub use functions::{Projectable, Reflectable, Rejectable, Rotatable};

pub trait VGA3DOps<F: Float> {
    fn reverse(self) -> Self;
    // fn dual(self) -> Self;
    fn conjugate(self) -> Self;
    fn involute(self) -> Self;
    fn norm(self) -> F;
    fn try_inverse(self) -> Option<Self>
    where
        Self: Sized;
}

pub trait VGA3DOpsRef<F: Float> {
    fn reverse(&self) -> Self;
    // fn dual(&&self) -> Self;
    fn conjugate(&self) -> Self;
    fn involute(&self) -> Self;
    fn norm(&self) -> F;
    fn try_inverse(&self) -> Option<Self>
    where
        Self: Sized;
}
