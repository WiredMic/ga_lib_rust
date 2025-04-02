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

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use libm::{fabsf, sqrtf};

use super::{bivector::Bivector, vector::Vector, VGA3DOps, VGA3DOpsRef};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Trivector {
    e123: f32,
}

impl Trivector {
    pub fn zero() -> Self {
        Self { e123: 0.0 }
    }

    pub fn new(e123: f32) -> Self {
        Self { e123 }
    }

    pub fn trivector(self) -> Self {
        self
    }

    pub fn e123(&self) -> f32 {
        self.e123
    }
}

#[cfg(test)]
mod new {
    use super::*;

    #[test]
    fn new() {
        let trivec = Trivector::new(2.0);
        assert_eq!(trivec.e123, 2.0);
    }
}

impl Neg for Trivector {
    type Output = Trivector;
    fn neg(self) -> Trivector {
        Trivector::new(-self.e123)
    }
}

// Cross Product
// \[ \overset\Rightarrow{a} \times \overset\Rightarrow{b} = \left <\overset\Rightarrow{a} \overset\Rightarrow{b} \right>_2 \]
// The cross product for two trivectors gives the bivector orthogonal to both.
// This does not exist in 3D
impl Trivector {
    pub fn cross(self, _b: Trivector) -> f32 {
        0.0
    }
}

#[cfg(test)]
mod trivector_cross {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn trivec_trivec_cross() {
        let trivector1 = Trivector::new(3.0);
        let trivector2 = Trivector::new(6.0);
        let res = trivector1.cross(trivector2);
        assert_relative_eq!(res, 0.0, max_relative = 0.000001);
    }
}

// Dual
// In VGA 3D, the dual is the pseudoscalar
// \[ \overset\Rightarrow{b} \overset\Rrightarrow{i} = -\vec{v} \]
// vector and bivectors in 3D VGA follows this pattern. Going up, going down
// \[ \mathrm{e}_1,\,\mathrm{e}_2,\,\mathrm{e}_3,\,\mathrm{e}_3\star,\,\mathrm{e}_2\star,\,\mathrm{e}_1\star,\, \]
impl Trivector {
    pub fn dual(self) -> f32 {
        -self.e123
    }
}

// Regressive Product
// \[ (A \vee B)\star = ( A\star  \wedge B\star ) \]
impl Trivector {
    pub fn regressive(self, b: Trivector) -> Trivector {
        // The exterior product of two scalars is a simple multiplication
        // The dual of scalar is the trivector
        -Trivector::new(self.dual() * b.dual())
    }
}

impl VGA3DOps for Trivector {
    // There is only one element.
    // The norm is the absolute value of e1e2e3
    fn norm(self) -> f32 {
        fabsf(self.e123())
        // sqrtf((self.reverse() * self).scalar())
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(self) -> Self {
        self.reverse() * (1.0 / (self * self.reverse()))
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(self) -> Self {
        -self
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+ - - + - - +\dots(-1)^{k(k+1)/2}\]

    fn conjugate(self) -> Self {
        self
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(self) -> Self {
        -self
    }
}

impl VGA3DOpsRef for Trivector {
    fn norm(&self) -> f32 {
        // sqrtf((self.reverse() * self).scalar())
        fabsf(self.e123())
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(&self) -> Self {
        self.reverse() * (1.0 / (self * self.reverse()))
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(&self) -> Self {
        -(*self)
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+ - - + - - +\dots(-1)^{k(k+1)/2}\]

    fn conjugate(&self) -> Self {
        -(*self)
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(&self) -> Self {
        *self
    }
}
