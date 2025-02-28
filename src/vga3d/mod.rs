// ga_lib is a rust library that implements different geometric algbras.
// Copyright (C) 2025 Rasmus Enevoldsen

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![allow(dead_code)]
#![allow(unused_imports)]

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

pub trait VGA3DOps {
    fn reverse(self) -> Self;
    // fn dual(&self) -> Self;
    fn conjugate(self) -> Self;
    fn involute(self) -> Self;
    fn norm(self) -> f32;
    fn inverse(self) -> Self;
}

pub trait VGA3DOpsRef {
    fn reverse(&self) -> Self;
    // fn dual(&&self) -> Self;
    fn conjugate(&self) -> Self;
    fn involute(&self) -> Self;
    fn norm(&self) -> f32;
    fn inverse(&self) -> Self;
}
