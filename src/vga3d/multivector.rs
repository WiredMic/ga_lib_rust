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

#![allow(unused_imports)]
#![allow(dead_code)]
// #![warn(missing_docs)]

use crate::forward_ref_binop;

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use num_traits::Float;

use super::{
    bivector::Bivector, trivector::Trivector, vector::Vector, Scalar, VGA3DOps, VGA3DOpsRef,
};

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "defmt")]
use defmt::Format;

/// # 3D Vector Geometric Algebra Multivector
/// Geometric algebra is the studie of multivectors
///
/// A multivector is a sum of all grades in the algebra
/// $$ M = \text{scalar} + \vec{v} + \overset\Rightarrow{b} + \overset\Rrightarrow{t} $$
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Multivector<F: Float> {
    pub(super) scalar: Scalar<F>,
    pub(super) vector: Vector<F>,
    pub(super) bivector: Bivector<F>,
    pub(super) trivector: Trivector<F>,
}

#[cfg(feature = "std")]
impl<F: Float + fmt::Display> fmt::Display for Multivector<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Multivector {{\n")?;
        write!(f, "\tscalar: {}\n", self.scalar.0,)?;
        write!(f, "\tvector: {}\n", self.vector,)?;
        write!(f, "\tbivector: {}\n", self.bivector,)?;
        write!(f, "\ttrivector: {}\n", self.trivector,)?;
        write!(f, "}}")
    }
}

#[cfg(feature = "defmt")]
impl<F: Float + defmt::Format> defmt::Format for Multivector<F> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Multivector {{\n");
        defmt::write!(f, "\tscalar: {}\n", self.scalar.scalar(),);
        defmt::write!(f, "\tvector: {}\n", self.vector,);
        defmt::write!(f, "\tbivector: {}\n", self.bivector,);
        defmt::write!(f, "\ttrivector: {}\n", self.trivector,);
        defmt::write!(f, "}}")
    }
}

impl<F: Float> Multivector<F> {
    pub fn zero() -> Self {
        Self {
            scalar: Scalar::zero(),
            vector: Vector::zero(),
            bivector: Bivector::zero(),
            trivector: Trivector::zero(),
        }
    }

    pub fn new(
        scalar: F,
        vector: Vector<F>,
        bivector: Bivector<F>,
        trivector: Trivector<F>,
    ) -> Self {
        Self {
            scalar: Scalar(scalar),
            vector,
            bivector,
            trivector,
        }
    }

    pub fn new_components(scalar: F, e1: F, e2: F, e3: F, e12: F, e31: F, e23: F, e123: F) -> Self {
        let vector = Vector::new(e1, e2, e3);
        let bivector = Bivector::new(e12, e31, e23);
        let trivector = Trivector::new(e123);
        Self {
            scalar: Scalar(scalar),
            vector,
            bivector,
            trivector,
        }
    }

    pub fn multivector(self) -> Self {
        self
    }

    // Scalar component
    pub fn scalar(&self) -> F {
        self.scalar.0
    }

    // Vector components
    pub fn vector(&self) -> Vector<F> {
        self.vector
    }

    pub fn e1(&self) -> F {
        self.vector.e1()
    }

    pub fn e2(&self) -> F {
        self.vector.e2()
    }

    pub fn e3(&self) -> F {
        self.vector.e3()
    }

    // Bivector components
    pub fn bivector(&self) -> Bivector<F> {
        self.bivector
    }

    pub fn e12(&self) -> F {
        self.bivector.e12()
    }

    pub fn e23(&self) -> F {
        self.bivector.e23()
    }

    pub fn e31(&self) -> F {
        self.bivector.e31()
    }

    // Trivector component
    pub fn trivector(&self) -> Trivector<F> {
        self.trivector
    }

    pub fn e123(&self) -> F {
        self.trivector.e123()
    }
}

impl<F: Float> Neg for Multivector<F> {
    type Output = Multivector<F>;
    fn neg(self) -> Multivector<F> {
        Multivector {
            scalar: Scalar(-self.scalar.0),
            vector: -self.vector,
            bivector: -self.bivector,
            trivector: -self.trivector,
        }
    }
}

impl<F: Float> Div<F> for Multivector<F> {
    // The division of rational numbers is a closed operation.
    type Output = Multivector<F>;

    fn div(self, b: F) -> Multivector<F> {
        if b == F::zero() {
            panic!("Cannot divide by zero-valued `Rational`!");
        }

        Multivector::new(
            self.scalar.0 / b,
            self.vector / b,
            self.bivector / b,
            self.trivector / b,
        )
    }
}
forward_ref_binop!(impl<F: Float> Div, div for Multivector<F>, F);

// // Cross Product
// // It does not make sence to take the cross product of two multvectors
// // impl Trivector {
// //     pub fn cross(self, _b: Trivector) -> f32 {
// //         0.0
// //     }
// // }

impl<F: Float> VGA3DOps<F> for Multivector<F> {
    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(self) -> Self {
        let scalar = self.scalar;
        let vector = self.vector;
        let bivector = -self.bivector;
        let trivector = -self.trivector;
        Multivector {
            scalar,
            vector,
            bivector,
            trivector,
        }
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(self) -> Self {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = -self.bivector;
        let trivector = self.trivector;
        Multivector {
            scalar,
            vector,
            bivector,
            trivector,
        }
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(self) -> Self {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = self.bivector;
        let trivector = -self.trivector;
        Multivector {
            scalar,
            vector,
            bivector,
            trivector,
        }
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(self) -> Self {
        self.reverse() / (self * self.reverse()).scalar.0
    }

    fn norm(self) -> F {
        (self.reverse() * self).scalar().sqrt()
    }
}

impl<F: Float> VGA3DOpsRef<F> for Multivector<F> {
    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(&self) -> Self {
        let scalar = self.scalar;
        let vector = self.vector;
        let bivector = -self.bivector;
        let trivector = -self.trivector;
        Multivector {
            scalar,
            vector,
            bivector,
            trivector,
        }
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(&self) -> Self {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = -self.bivector;
        let trivector = self.trivector;
        Multivector {
            scalar,
            vector,
            bivector,
            trivector,
        }
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(&self) -> Self {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = self.bivector;
        let trivector = -self.trivector;
        Multivector {
            scalar,
            vector,
            bivector,
            trivector,
        }
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(&self) -> Self {
        self.reverse() / (self * self.reverse()).scalar.0
    }

    // the norm of a multivector |A|
    // \[|A|^2=\left< A\^dag A \right>_0\]
    fn norm(&self) -> F {
        (self.reverse() * self).scalar().sqrt()
    }
}

// Dual
// In VGA 3D, the dual is the pseudoscalar
// \[ \text{scalar},\mathrm{e}_1,\,\mathrm{e}_2,\,\mathrm{e}_3,\,\mathrm{e}_3\star,\,\mathrm{e}_2\star,\,\mathrm{e}_1\star,\, \text{scalar} \star \]
impl<F: Float> Multivector<F> {
    pub fn dual(self) -> Multivector<F> {
        let scalar = self.trivector().dual();
        let vector = self.bivector().dual();
        let bivector = self.vector().dual();
        let trivector = Trivector::new(self.scalar());
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
mod quaternion {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn multivector_norm() {
        // 3e12+5e31+4e23
        let multivector = Multivector::new_components(2.0, -1.0, 4.0, 6.0, 4.0, 3.0, 2.2, -3.3);
        let multivector_reverse = multivector.reverse();

        assert_relative_eq!(
            multivector.norm(),
            9.885848471426213,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            (&multivector).norm(),
            9.885848471426213,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            multivector_reverse.norm(),
            9.885848471426213,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            (&multivector_reverse).norm(),
            9.885848471426213,
            max_relative = 0.000001
        );
    }
}
