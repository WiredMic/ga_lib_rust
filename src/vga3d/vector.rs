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
    bivector::Bivector, multivector::Multivector, scalar::Scalar, trivector::Trivector, VGA3DOps,
    VGA3DOpsRef,
};

/// # 3D Vector Geometric Algebra Vector
/// This is the same vector as in $\mathbb{R}^3$
/// $$\vec{v}=v_1 \mathrm{e}_1 + v_2 \mathrm{e}_2 + v_3 \mathrm{e}_3$$
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vector<F: Float> {
    e1: F,
    e2: F,
    e3: F,
}

#[cfg(feature = "std")]
impl<F: Float + fmt::Display> fmt::Display for Vector<F> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}e1, {}e2, {}e3", self.e1, self.e2, self.e3)
        write!(f, "Vector {{")?;
        write!(f, " {}e1", self.e1)?;

        // For e2 component, add appropriate sign
        if self.e2 >= F::zero() {
            write!(f, " + {}e2", self.e2)?;
        } else {
            write!(f, " - {}e2", self.e2.abs())?;
        }

        // For e3 component, add appropriate sign
        if self.e3 >= F::zero() {
            write!(f, " + {}e3", self.e3)?;
        } else {
            write!(f, " - {}e3", self.e3.abs())?;
        }
        write!(f, " }}")?;

        Ok(())
    }
}

#[cfg(feature = "defmt")]
impl<F: Float + defmt::Format> defmt::Format for Vector<F> {
    fn format(&self, f: defmt::Formatter) {
        // write!(f, "{}e1, {}e2, {}e3", self.e1, self.e2, self.e3)
        defmt::write!(f, "Vector {{");
        defmt::write!(f, " {}e1", self.e1);

        // For e2 component, add appropriate sign
        if self.e2 >= F::zero() {
            defmt::write!(f, " + {}e2", self.e2);
        } else {
            defmt::write!(f, " - {}e2", self.e2.abs());
        }

        // For e3 component, add appropriate sign
        if self.e3 >= F::zero() {
            defmt::write!(f, " + {}e3", self.e3);
        } else {
            defmt::write!(f, " - {}e3", self.e3.abs());
        }
        defmt::write!(f, " }}");
    }
}

impl<F: Float> Vector<F> {
    /// The zero vector
    pub fn zero() -> Self {
        Self {
            e1: F::zero(),
            e2: F::zero(),
            e3: F::zero(),
        }
    }

    /// Create new vector from linear combination of unit vectors
    pub fn new(e1: F, e2: F, e3: F) -> Self {
        Self { e1, e2, e3 }
    }

    /// Get vector
    pub fn vector(self) -> Self {
        self
    }

    // Vector components
    /// Get unit scaling factor for $\mathrm{e}_1$
    pub fn e1(&self) -> F {
        self.e1
    }

    /// Get unit scaling factor for $\mathrm{e}_2$
    pub fn e2(&self) -> F {
        self.e2
    }

    /// Get unit scaling factor for $\mathrm{e}_3$
    pub fn e3(&self) -> F {
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
impl<F: Float> Neg for Vector<F> {
    type Output = Vector<F>;
    fn neg(self) -> Vector<F> {
        Vector::new(-self.e1, -self.e2, -self.e3)
    }
}

impl<F: Float> Vector<F> {
    /// # Cross Product
    /// The cross product is the dual of the exterior product
    /// $$ \vec{v}\times\vec{u} = (\vec{v}\wedge\vec{u})\star $$
    pub fn cross(self, b: Vector<F>) -> Vector<F> {
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

impl<F: Float> Vector<F> {
    /// # Dual
    /// In VGA 3D, the dual is the pseudoscalar
    /// $$ \vec{v} \overset\Rrightarrow{i} = \overset\Rightarrow{b} $$
    /// vector and bivectors in 3D VGA follows this pattern. Going up, going down
    /// $$\text{scalar}, \mathrm{e}_1,\mathrm{e}_2,\mathrm{e}_3,\mathrm{e}_3\star,\mathrm{e}_2\star,\mathrm{e}_1\star, \text{scalar}\star $$
    pub fn dual(self) -> Bivector<F> {
        Bivector::new(self.e3, self.e2, self.e1)
    }
}

#[cfg(test)]
mod vector_dual {
    use super::*;
    #[test]
    fn vector_to_bivector() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        let bivector = vector.dual();
        assert_eq!(vector.e1(), bivector.e23());
        assert_eq!(vector.e2(), bivector.e31());
        assert_eq!(vector.e3(), bivector.e12());
    }
}

impl<F: Float> VGA3DOps<F> for Vector<F> {
    fn norm(self) -> Scalar<F> {
        Scalar(((self.e1() * self.e1()) + (self.e2() * self.e2()) + (self.e3() * self.e3())).sqrt())
    }

    // Normilize
    // $$\frac{A}{|A|}$$
    fn try_normalize(self) -> Option<Self>
    where
        Self: Sized,
    {
        match self.norm().try_inverse() {
            None => None,
            Some(norm_inverse) => Some(self * norm_inverse),
        }
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn try_inverse(self) -> Option<Self> {
        let norm_squared = (self * self.reverse()).scalar();
        match Scalar(norm_squared).try_inverse() {
            None => None,
            Some(scalar_inverse) => Some(self.reverse() * scalar_inverse),
        }
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(self) -> Self {
        self
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(self) -> Self {
        -self
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(self) -> Self {
        -self
    }
}

impl<F: Float> VGA3DOpsRef<F> for Vector<F> {
    fn norm(&self) -> Scalar<F> {
        // ((self.reverse() * self).scalar()).sqrt()
        Scalar(((self.e1() * self.e1()) + (self.e2() * self.e2()) + (self.e3() * self.e3())).sqrt())
    }

    // Normilize
    // $$\frac{A}{|A|}$$
    fn try_normalize(&self) -> Option<Self>
    where
        Self: Sized,
    {
        match self.norm().try_inverse() {
            None => None,
            Some(norm_inverse) => Some(*self * norm_inverse),
        }
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn try_inverse(&self) -> Option<Vector<F>> {
        let norm_squared = (self * self.reverse()).scalar();
        match Scalar(norm_squared).try_inverse() {
            None => None,
            Some(scalar_inverse) => Some(self.reverse() * scalar_inverse),
        }
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(&self) -> Self {
        *self
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(&self) -> Self {
        -(*self)
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(&self) -> Self {
        -(*self)
    }
}

#[cfg(test)]
mod vector {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn vector_norm() {
        // 3e12+5e31+4e23
        let vector = Vector::new(3.0, 5.0, 4.0);
        let vector_reverse = vector.reverse();
        assert_relative_eq!(vector_reverse.e1(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(vector_reverse.e2(), 5.0, max_relative = 0.000001);
        assert_relative_eq!(vector_reverse.e3(), 4.0, max_relative = 0.000001);

        assert_relative_eq!(
            vector.norm().scalar(),
            7.0710678118654755,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            (&vector).norm().scalar(),
            7.0710678118654755,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            vector_reverse.norm().scalar(),
            7.0710678118654755,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            (&vector_reverse).norm().scalar(),
            7.0710678118654755,
            max_relative = 0.000001
        );
    }
}
