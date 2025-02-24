#![allow(unused_imports)]
#![allow(dead_code)]

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use libm::{fabsf, sqrtf};

use super::{bivector::VGA3DBivector, vector::VGA3DVector, VGA3DOps, VGA3DOpsRef};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct VGA3DTrivector {
    e123: f32,
}

impl VGA3DTrivector {
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
        let trivec = VGA3DTrivector::new(2.0);
        assert_eq!(trivec.e123, 2.0);
    }
}

impl Neg for VGA3DTrivector {
    type Output = VGA3DTrivector;
    fn neg(self) -> VGA3DTrivector {
        VGA3DTrivector::new(-self.e123)
    }
}

// Cross Product
// \[ \overset\Rightarrow{a} \times \overset\Rightarrow{b} = \left <\overset\Rightarrow{a} \overset\Rightarrow{b} \right>_2 \]
// The cross product for two trivectors gives the bivector orthogonal to both.
// This does not exist in 3D
impl VGA3DTrivector {
    pub fn cross(self, _b: VGA3DTrivector) -> f32 {
        0.0
    }
}

#[cfg(test)]
mod trivector_cross {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn trivec_trivec_cross() {
        let trivector1 = VGA3DTrivector::new(3.0);
        let trivector2 = VGA3DTrivector::new(6.0);
        let res = trivector1.cross(trivector2);
        assert_relative_eq!(res, 0.0, max_relative = 0.000001);
    }
}

// Dual
// In VGA 3D, the dual is the pseudoscalar
// \[ \overset\Rightarrow{b} \overset\Rrightarrow{i} = -\vec{v} \]
// vector and bivectors in 3D VGA follows this pattern. Going up, going down
// \[ \mathrm{e}_1,\,\mathrm{e}_2,\,\mathrm{e}_3,\,\mathrm{e}_3\star,\,\mathrm{e}_2\star,\,\mathrm{e}_1\star,\, \]
impl VGA3DTrivector {
    pub fn dual(self) -> f32 {
        -self.e123
    }
}

// Regressive Product
// \[ (A \vee B)\star = ( A\star  \wedge B\star ) \]
impl VGA3DTrivector {
    pub fn regressive(self, b: VGA3DTrivector) -> VGA3DTrivector {
        // The exterior product of two scalars is a simple multiplication
        // The dual of scalar is the trivector
        -VGA3DTrivector::new(self.dual() * b.dual())
    }
}

impl VGA3DOps for VGA3DTrivector {
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

impl VGA3DOpsRef for VGA3DTrivector {
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
