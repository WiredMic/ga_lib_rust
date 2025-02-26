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
// #![warn(missing_docs)]

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use libm::sqrtf;

use super::{
    bivector::VGA3DBivector,
    trivector::{self, VGA3DTrivector},
    vector::VGA3DVector,
    VGA3DOps, VGA3DOpsRef,
};

/// # 3D Vector Geometric Algebra Multivector
/// Geometric algebra is the studie of multivectors
///
/// A multivector is a sum of all grades in the algebra
/// $$ M = \text{scalar} + \vec{v} + \overset\Rightarrow{b} + \overset\Rrightarrow{t} $$
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct VGA3DMultivector {
    scalar: f32,
    vector: VGA3DVector,
    bivector: VGA3DBivector,
    trivector: VGA3DTrivector,
}

impl VGA3DMultivector {
    pub fn zero() -> Self {
        Self {
            scalar: 0.0,
            vector: VGA3DVector::zero(),
            bivector: VGA3DBivector::zero(),
            trivector: VGA3DTrivector::zero(),
        }
    }

    pub fn new(
        scalar: f32,
        vector: VGA3DVector,
        bivector: VGA3DBivector,
        trivector: VGA3DTrivector,
    ) -> Self {
        Self {
            scalar,
            vector,
            bivector,
            trivector,
        }
    }

    pub fn new_components(
        scalar: f32,
        e1: f32,
        e2: f32,
        e3: f32,
        e12: f32,
        e31: f32,
        e23: f32,
        e123: f32,
    ) -> Self {
        let vector = VGA3DVector::new(e1, e2, e3);
        let bivector = VGA3DBivector::new(e12, e31, e23);
        let trivector = VGA3DTrivector::new(e123);
        Self {
            scalar,
            vector,
            bivector,
            trivector,
        }
    }

    pub fn multivector(self) -> Self {
        self
    }

    // Scalar component
    pub fn scalar(&self) -> f32 {
        self.scalar
    }

    // Vector components
    pub fn vector(&self) -> VGA3DVector {
        self.vector
    }

    pub fn e1(&self) -> f32 {
        self.vector.e1()
    }

    pub fn e2(&self) -> f32 {
        self.vector.e2()
    }

    pub fn e3(&self) -> f32 {
        self.vector.e3()
    }

    // Bivector components
    pub fn bivector(&self) -> VGA3DBivector {
        self.bivector
    }

    pub fn e12(&self) -> f32 {
        self.bivector.e12()
    }

    pub fn e23(&self) -> f32 {
        self.bivector.e23()
    }

    pub fn e31(&self) -> f32 {
        self.bivector.e31()
    }

    // Trivector component
    pub fn trivector(&self) -> VGA3DTrivector {
        self.trivector
    }

    pub fn e123(&self) -> f32 {
        self.trivector.e123()
    }
}

impl Neg for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn neg(self) -> VGA3DMultivector {
        VGA3DMultivector::new(-self.scalar, -self.vector, -self.bivector, -self.trivector)
    }
}

// // Cross Product
// // It does not make sence to take the cross product of two multvectors
// // impl VGA3DTrivector {
// //     pub fn cross(self, _b: VGA3DTrivector) -> f32 {
// //         0.0
// //     }
// // }

impl VGA3DOps for VGA3DMultivector {
    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(self) -> VGA3DMultivector {
        let scalar = self.scalar;
        let vector = self.vector;
        let bivector = -self.bivector;
        let trivector = -self.trivector;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(self) -> VGA3DMultivector {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = -self.bivector;
        let trivector = self.trivector;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(self) -> VGA3DMultivector {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = self.bivector;
        let trivector = -self.trivector;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(self) -> VGA3DMultivector {
        self.reverse() * (1.0 / (self * self.reverse()).scalar)
    }

    fn norm(self) -> f32 {
        sqrtf((self.reverse() * self).scalar())
    }
}

impl VGA3DOpsRef for VGA3DMultivector {
    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(&self) -> VGA3DMultivector {
        let scalar = self.scalar;
        let vector = self.vector;
        let bivector = -self.bivector;
        let trivector = -self.trivector;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(&self) -> VGA3DMultivector {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = -self.bivector;
        let trivector = self.trivector;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(&self) -> VGA3DMultivector {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = self.bivector;
        let trivector = -self.trivector;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(&self) -> VGA3DMultivector {
        self.reverse() * (1.0 / (self * self.reverse()).scalar)
    }

    // the norm of a multivector |A|
    // \[|A|^2=\left< A\^dag A \right>_0\]
    fn norm(&self) -> f32 {
        sqrtf((self.reverse() * self).scalar())
    }
}

// Dual
// In VGA 3D, the dual is the pseudoscalar
// \[ \text{scalar},\mathrm{e}_1,\,\mathrm{e}_2,\,\mathrm{e}_3,\,\mathrm{e}_3\star,\,\mathrm{e}_2\star,\,\mathrm{e}_1\star,\, \text{scalar} \star \]
impl VGA3DMultivector {
    pub fn dual(self) -> VGA3DMultivector {
        let scalar = self.trivector().dual();
        let vector = self.bivector().dual();
        let bivector = self.vector().dual();
        let trivector = VGA3DTrivector::new(self.scalar());
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// // Regressive Product
// // \[ (A \vee B)\star = ( A\star  \wedge B\star ) \]
// impl VGA3DTrivector {
//     pub fn regressive(self) -> VGA3DTrivector {
//         // TODO
//         self
//     }
// }
