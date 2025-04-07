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

use libm::sqrtf;

use super::{
    multivector::Multivector,
    trivector::{self, Trivector},
    vector::Vector,
    VGA3DOps, VGA3DOpsRef,
};

/// # 3D Vector Geometric Algebra Bivector
/// This is the oriented area.
/// $$\overset\Rightarrow{b}=b_1 \mathrm{e}_1\mathrm{e}_2 + b_2 \mathrm{e}_3\mathrm{e}_1 + b_3 \mathrm{e}_2\mathrm{e}_3$$
///
/// This is the correct way to represent an axial vector.
/// The two are confused because the bivector is the dual of the vector.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Bivector {
    e12: f32,
    e31: f32,
    e23: f32,
}

#[cfg(feature = "std")]
impl fmt::Display for Bivector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}e12, {}e31, {}e23", self.e12, self.e31, self.e23)

        write!(f, "Bivector {{")?;
        write!(f, " {}e1e2", self.e12)?;

        // For e2 component, add appropriate sign
        if self.e31 >= 0.0 {
            write!(f, " + {}e31", self.e31)?;
        } else {
            write!(f, " - {}e31", self.e31.abs())?;
        }

        // For e3 component, add appropriate sign
        if self.e23 >= 0.0 {
            write!(f, " + {}e23", self.e23)?;
        } else {
            write!(f, " - {}e23", self.e23.abs())?;
        }
        write!(f, " }}")?;

        Ok(())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Bivector {
    fn format(&self, f: defmt::Formatter) {
        // defmt::write!(f, "{}e12, {}e31, {}e23", self.e12, self.e31, self.e23)

        defmt::write!(f, "Bivector {{");
        defmt::write!(f, " {}e1e2", self.e12);

        // For e2 component, add appropriate sign
        if self.e31 >= 0.0 {
            defmt::write!(f, " + {}e31", self.e31);
        } else {
            defmt::write!(f, " - {}e31", self.e31.abs());
        }

        // For e3 component, add appropriate sign
        if self.e23 >= 0.0 {
            defmt::write!(f, " + {}e23", self.e23);
        } else {
            defmt::write!(f, " - {}e23", self.e23.abs());
        }
        defmt::write!(f, " }}")?;
    }
}

impl Bivector {
    /// The zero bivector
    pub fn zero() -> Self {
        Self {
            e12: 0.0,
            e31: 0.0,
            e23: 0.0,
        }
    }

    /// Create new bivector from linear combination of unit bivector
    pub fn new(e12: f32, e31: f32, e23: f32) -> Self {
        Self { e12, e31, e23 }
    }

    /// Get bivector
    pub fn bivector(self) -> Self {
        self
    }

    /// The scaling factor for unit bivector $\mathrm{e}_1\mathrm{e}_2$
    pub fn e12(&self) -> f32 {
        self.e12
    }

    /// The scaling factor for unit bivector $\mathrm{e}_3\mathrm{e}_1$
    pub fn e23(&self) -> f32 {
        self.e23
    }

    /// The scaling factor for unit bivector $\mathrm{e}_2\mathrm{e}_3$
    pub fn e31(&self) -> f32 {
        self.e31
    }
}

#[cfg(test)]
mod new {
    use super::*;

    #[test]
    fn new() {
        let bivec = Bivector::new(2.0, 1.0, 1.0);
        assert_eq!(bivec.e12, 2.0);
        assert_eq!(bivec.e31, 1.0);
        assert_eq!(bivec.e23, 1.0);
    }
}

// Negation
impl Neg for Bivector {
    type Output = Bivector;
    fn neg(self) -> Bivector {
        Bivector::new(-self.e12, -self.e31, -self.e23)
    }
}

impl Bivector {
    /// # Cross Product
    /// The cross product for two bivectors gives the bivector orthogonal to both
    /// $$ \overset\Rightarrow{a} \times \overset\Rightarrow{b} = \left <\overset\Rightarrow{a} \overset\Rightarrow{b} \right>_2 $$
    pub fn cross(self, b: Bivector) -> Bivector {
        let e12 = self.e31 * b.e23 - self.e23 * b.e31;
        let e31 = self.e23 * b.e12 - self.e12 * b.e23;
        let e23 = self.e12 * b.e31 - self.e31 * b.e12;
        Bivector::new(e12, e31, e23)
    }
}

#[cfg(test)]
mod bivector_cross {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn bivector_bivector_cross() {
        // 3e1+5e2+4e3
        let bivector1 = Bivector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let bivector2 = Bivector::new(2.0, 1.0, 6.0);
        //
        let res_bivector = Bivector::new(26.0, -10.0, -7.0);
        assert_relative_eq!(
            bivector1.cross(bivector2).e12(),
            res_bivector.e12(),
            max_relative = 0.000001
        );
        assert_relative_eq!(
            bivector1.cross(bivector2).e31(),
            res_bivector.e31(),
            max_relative = 0.000001
        );
        assert_relative_eq!(
            bivector1.cross(bivector2).e23(),
            res_bivector.e23(),
            max_relative = 0.000001
        );
    }
}

impl Bivector {
    /// # Dual
    /// In VGA 3D, the dual is the unit pseudoscalar $\overset\Rrightarrow{i}$
    ///
    /// $$ \overset\Rightarrow{b} \overset\Rrightarrow{i} = -\vec{v} $$
    ///
    /// Vector and bivectors in 3D VGA follows this pattern. Going up, going down
    ///
    /// $$ \text{scalar}, \mathrm{e}_1,\mathrm{e}_2,\mathrm{e}_3,\mathrm{e}_3\star, \mathrm{e}_2\star, \mathrm{e}_1\star, \text{scalar}\star $$
    pub fn dual(self) -> Vector {
        Vector::new(-self.e23, -self.e31, -self.e12)
    }
}

// #[cfg(test)]
// mod vector_cross {
//     use super::*;
//     use approx::assert_relative_eq;
//     #[test]
//     fn vector_vector_cross() {
//     }}

impl Bivector {
    /// # Inverse
    /// $$ A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>} $$
    pub fn inverse(self) -> Bivector {
        self.reverse() * (1.0 / (self * self.reverse()).scalar())
    }
}

impl Bivector {
    /// Regressive Product
    /// $$ (A \vee B)\star = ( A\star  \wedge B\star ) $$
    /// NOT IMPEMENTED
    pub fn regressive(self) -> Bivector {
        // TODO
        self
    }
}

impl VGA3DOps for Bivector {
    fn norm(self) -> f32 {
        sqrtf((self.e12() * self.e12()) + (self.e31() * self.e31()) + (self.e23() * self.e23()))
        // sqrtf((self.reverse() * self).scalar())
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(self) -> Self {
        self.reverse() * (1.0 / (self * self.reverse()).scalar())
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
        -self
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(self) -> Self {
        self
    }
}

impl VGA3DOpsRef for Bivector {
    fn norm(&self) -> f32 {
        // sqrtf((self.reverse() * self).scalar())
        sqrtf((self.e12() * self.e12()) + (self.e31() * self.e31()) + (self.e23() * self.e23()))
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(&self) -> Self {
        self.reverse() * (1.0 / (self * self.reverse()).scalar())
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
