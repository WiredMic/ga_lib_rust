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
#![warn(missing_docs)]

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use libm::sqrtf;

//
use super::{
    bivector::Bivector, multivector::Multivector, trivector::Trivector, VGA3DOps,
    VGA3DOpsRef,
};

/// # 3D Vector Geometric Algebra Vector
/// This is the same vector as in $\mathbb{R}^3$
/// $$\vec{v}=v_1 \mathrm{e}_1 + v_2 \mathrm{e}_2 + v_3 \mathrm{e}_3$$
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    e1: f32,
    e2: f32,
    e3: f32,
}

impl Vector {
    /// The zero vector
    pub fn zero() -> Self {
        Self {
            e1: 0.0,
            e2: 0.0,
            e3: 0.0,
        }
    }

    /// Create new vector from linear combination of unit vectors
    pub fn new(e1: f32, e2: f32, e3: f32) -> Self {
        Self { e1, e2, e3 }
    }

    /// Get vector
    pub fn vector(self) -> Self {
        self
    }

    // Vector components
    /// Get unit scaling factor for $\mathrm{e}_1$
    pub fn e1(&self) -> f32 {
        self.e1
    }

    /// Get unit scaling factor for $\mathrm{e}_2$
    pub fn e2(&self) -> f32 {
        self.e2
    }

    /// Get unit scaling factor for $\mathrm{e}_3$
    pub fn e3(&self) -> f32 {
        self.e3
    }
}

#[cfg(test)]
mod vector_new {
    use super::*;

    #[test]
    fn vector_new() {
        let vec = Vector::new(2.0, 1.0, 1.0);
        assert_eq!(vec.e1, 2.0);
        assert_eq!(vec.e2, 1.0);
        assert_eq!(vec.e3, 1.0);
    }
}

// Negation
impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector::new(-self.e1, -self.e2, -self.e3)
    }
}

impl Vector {
    /// # Cross Product
    /// The cross product is the dual of the exterior product
    /// $$ \vec{v}\times\vec{u} = (\vec{v}\wedge\vec{u})\star $$
    pub fn cross(self, b: Vector) -> Vector {
        // -(self ^ b).dual()
        let e1 = self.e2() * b.e3() - self.e3() * b.e2();
        let e2 = self.e3() * b.e1() - self.e1() * b.e3();
        let e3 = self.e1() * b.e2() - self.e2() * b.e1();
        Vector::new(e1, e2, e3)
    }
}
#[cfg(test)]
mod vector_cross {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn vector_vector_cross() {
        // 3e1+5e2+4e3
        let vector1 = Vector::new(3.0, 5.0, 4.0);
        // 2e1+1e2+6e3
        let vector2 = Vector::new(2.0, 1.0, 6.0);
        let cross = vector1.cross(vector2);
        // −7e12​-10e31​+26e23
        assert_relative_eq!(cross.e1(), 26.0, max_relative = 0.000001);
        assert_relative_eq!(cross.e2(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(cross.e3(), -7.0, max_relative = 0.000001);
    }
}

impl Vector {
    /// # Dual
    /// In VGA 3D, the dual is the pseudoscalar
    /// $$ \vec{v} \overset\Rrightarrow{i} = \overset\Rightarrow{b} $$
    /// vector and bivectors in 3D VGA follows this pattern. Going up, going down
    /// $$\text{scalar}, \mathrm{e}_1,\mathrm{e}_2,\mathrm{e}_3,\mathrm{e}_3\star,\mathrm{e}_2\star,\mathrm{e}_1\star, \text{scalar}\star $$
    pub fn dual(self) -> Bivector {
        Bivector::new(self.e3, self.e2, self.e1)
    }
}

#[cfg(test)]
mod vector_dual {
    use super::*;
    #[test]
    fn vector_to_bivector() {
        let vector: Vector = Vector::new(1.0, 2.0, 3.0);
        let bivector: Bivector = vector.dual();
        assert_eq!(vector.e1(), bivector.e23());
        assert_eq!(vector.e2(), bivector.e31());
        assert_eq!(vector.e3(), bivector.e12());
    }
}

impl VGA3DOps for Vector {
    fn norm(self) -> f32 {
        sqrtf((self.e1() * self.e1()) + (self.e2() * self.e2()) + (self.e3() * self.e3()))
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(self) -> Vector {
        self.reverse() * (1.0 / (self * self.reverse()).scalar())
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(self) -> Vector {
        self
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(self) -> Vector {
        -self
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(self) -> Self {
        -self
    }
}

impl VGA3DOpsRef for Vector {
    fn norm(&self) -> f32 {
        // sqrtf((self.reverse() * self).scalar())
        sqrtf((self.e1() * self.e1()) + (self.e2() * self.e2()) + (self.e3() * self.e3()))
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(&self) -> Vector {
        self.reverse() * (1.0 / (self * self.reverse()).scalar())
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(&self) -> Vector {
        *self
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(&self) -> Vector {
        -(*self)
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(&self) -> Self {
        -(*self)
    }
}
