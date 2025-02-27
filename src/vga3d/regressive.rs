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

#![allow(unused_imports)]
#![allow(dead_code)]

use super::{
    bivector::{self, Bivector},
    multivector::Multivector,
    rotor::VGA3DRotor,
    trivector::{self, Trivector},
    vector::Vector,
};

use crate::forward_ref_binop;

/// # Regressive Product
/// $$ (A \vee B)\star = ( A\star  \wedge B\star ) $$
/// NOT IMPLEMENTED
impl Vector {
    pub fn regressive(self) -> Vector {
        // TODO
        self
    }
}
