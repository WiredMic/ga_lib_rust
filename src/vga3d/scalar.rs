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
#![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "defmt")]
use defmt::Format;

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use num_traits::Float;
// use libm::sqrtf;

use crate::forward_ref_binop;

use super::{
    bivector::Bivector, multivector::Multivector, trivector::Trivector, VGA3DOps, VGA3DOpsRef,
};

/// # 3D Vector Geometric Algebra Scalar
/// Wrapper for the Generic Float type
#[derive(Default, Debug, Clone, Copy, PartialEq)]
/// Scalar struct
pub struct Scalar<F: Float>(pub(super) F);

impl<F: Float> Scalar<F> {
    /// The zero scalar
    pub fn zero() -> Self {
        Scalar(F::zero())
    }

    /// New scalar from scalar
    pub fn new(scalar: F) -> Self {
        Scalar(scalar)
    }

    ///  Get scalar
    pub fn scalar(self) -> F {
        self.0
    }

    /// try to inverse a scalar value
    pub fn try_inverse(self) -> Option<Scalar<F>> {
        let scalar = self.0;
        match scalar {
            scalar if scalar.is_zero() => None,
            _ => return Some(Scalar(F::one() / scalar)),
        }
    }
}

#[cfg(test)]
mod quaternion {

    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn scalar_inverse() {
        let scalar = Scalar(4.0);
        let scalar_inverse = match scalar.try_inverse() {
            Some(scalar) => scalar,
            None => Scalar(0.0),
        };

        assert_relative_eq!(scalar_inverse.scalar(), 1.0 / 4.0, max_relative = 0.000001);
    }
}
