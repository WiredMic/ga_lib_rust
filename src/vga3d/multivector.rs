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

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use libm::sqrtf;

use super::{
    bivector::Bivector,
    trivector::{self, Trivector},
    vector::Vector,
    VGA3DOps, VGA3DOpsRef,
};

/// # 3D Vector Geometric Algebra Multivector
/// Geometric algebra is the studie of multivectors
///
/// A multivector is a sum of all grades in the algebra
/// $$ M = \text{scalar} + \vec{v} + \overset\Rightarrow{b} + \overset\Rrightarrow{t} $$
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Multivector {
    scalar: f32,
    vector: Vector,
    bivector: Bivector,
    trivector: Trivector,
}

impl Multivector {
    pub fn zero() -> Self {
        Self {
            scalar: 0.0,
            vector: Vector::zero(),
            bivector: Bivector::zero(),
            trivector: Trivector::zero(),
        }
    }

    pub fn new(scalar: f32, vector: Vector, bivector: Bivector, trivector: Trivector) -> Self {
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
        let vector = Vector::new(e1, e2, e3);
        let bivector = Bivector::new(e12, e31, e23);
        let trivector = Trivector::new(e123);
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
    pub fn vector(&self) -> Vector {
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
    pub fn bivector(&self) -> Bivector {
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
    pub fn trivector(&self) -> Trivector {
        self.trivector
    }

    pub fn e123(&self) -> f32 {
        self.trivector.e123()
    }
}

impl Neg for Multivector {
    type Output = Multivector;
    fn neg(self) -> Multivector {
        Multivector::new(-self.scalar, -self.vector, -self.bivector, -self.trivector)
    }
}

// // Cross Product
// // It does not make sence to take the cross product of two multvectors
// // impl Trivector {
// //     pub fn cross(self, _b: Trivector) -> f32 {
// //         0.0
// //     }
// // }

impl VGA3DOps for Multivector {
    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(self) -> Multivector {
        let scalar = self.scalar;
        let vector = self.vector;
        let bivector = -self.bivector;
        let trivector = -self.trivector;
        Multivector::new(scalar, vector, bivector, trivector)
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(self) -> Multivector {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = -self.bivector;
        let trivector = self.trivector;
        Multivector::new(scalar, vector, bivector, trivector)
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(self) -> Multivector {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = self.bivector;
        let trivector = -self.trivector;
        Multivector::new(scalar, vector, bivector, trivector)
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(self) -> Multivector {
        self.reverse() * (1.0 / (self * self.reverse()).scalar)
    }

    fn norm(self) -> f32 {
        sqrtf((self.reverse() * self).scalar())
    }
}

impl VGA3DOpsRef for Multivector {
    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(&self) -> Multivector {
        let scalar = self.scalar;
        let vector = self.vector;
        let bivector = -self.bivector;
        let trivector = -self.trivector;
        Multivector::new(scalar, vector, bivector, trivector)
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(&self) -> Multivector {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = -self.bivector;
        let trivector = self.trivector;
        Multivector::new(scalar, vector, bivector, trivector)
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(&self) -> Multivector {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = self.bivector;
        let trivector = -self.trivector;
        Multivector::new(scalar, vector, bivector, trivector)
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(&self) -> Multivector {
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
impl Multivector {
    pub fn dual(self) -> Multivector {
        let scalar = self.trivector().dual();
        let vector = self.bivector().dual();
        let bivector = self.vector().dual();
        let trivector = Trivector::new(self.scalar());
        Multivector::new(scalar, vector, bivector, trivector)
    }
}

// // Regressive Product
// // \[ (A \vee B)\star = ( A\star  \wedge B\star ) \]
// impl Trivector {
//     pub fn regressive(self) -> Trivector {
//         // TODO
//         self
//     }
// }
