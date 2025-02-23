#![allow(unused_imports)]
#![allow(dead_code)]

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use libm::sqrtf;

use super::bivector::VGA3DBivector;
use super::vector::VGA3DVector;

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

// Geometric Product
// \[ \overset\Rrightarrow{a}\overset\Rrightarrow{b}= \overset\Rrightarrow{a} \cdot \overset\Rrightarrow{b}\]
impl Mul for VGA3DTrivector {
    type Output = f32;

    fn mul(self: VGA3DTrivector, b: VGA3DTrivector) -> f32 {
        -self.e123 * b.e123
    }
}
#[cfg(test)]
mod trivector_geo {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn trivec_trivec_geo() {
        let trivector1 = VGA3DTrivector::new(3.0);
        let trivector2 = VGA3DTrivector::new(6.0);
        let res = trivector1 * trivector2;
        assert_relative_eq!(res, -18.0, max_relative = 0.000001);
    }
}

// Inner Product / Dot Product
// In 3D there the geometric product of two trivectors is there inner product
// \[ \overset\Rrightarrow{a} \cdot \overset\Rrightarrow{b} = \left <\overset\Rrightarrow{a} \overset\Rrightarrow{b} \right>_0 = \overset\Rrightarrow{a} \overset\Rrightarrow{b} \]
impl VGA3DTrivector {
    pub fn inner(self, b: VGA3DTrivector) -> f32 {
        self * b
    }
}
impl BitOr for VGA3DTrivector {
    type Output = f32;

    fn bitor(self: VGA3DTrivector, b: VGA3DTrivector) -> f32 {
        self.inner(b)
    }
}

#[cfg(test)]
mod trivector_inner {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn trivec_trivec_inner() {
        let trivector1 = VGA3DTrivector::new(3.0);
        let trivector2 = VGA3DTrivector::new(6.0);
        let res = trivector1 | trivector2;
        assert_relative_eq!(res, -18.0, max_relative = 0.000001);
    }
}

// Exterior Product / Wedge Product
// \[ \overset\Rrightarrow{a} \wedge \overset\Rrightarrow{b} = \left <\overset\Rrightarrow{a} \overset\Rrightarrow{b} \right>_4 \]
// There is no object of grade 6 in 3D VGA
impl VGA3DTrivector {
    pub fn exterior(self, _b: VGA3DTrivector) -> f32 {
        0.0
    }
}
impl BitXor for VGA3DTrivector {
    type Output = f32;

    fn bitxor(self: VGA3DTrivector, b: VGA3DTrivector) -> f32 {
        self.exterior(b)
    }
}

#[cfg(test)]
mod trivector_exterior {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn trivec_trivec_exterior() {
        let trivector1 = VGA3DTrivector::new(3.0);
        let trivector2 = VGA3DTrivector::new(6.0);
        let res = trivector1 ^ trivector2;
        assert_relative_eq!(res, 0.0, max_relative = 0.000001);
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

// Others
impl VGA3DTrivector {
    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    pub fn reverse(self) -> VGA3DTrivector {
        -self
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    pub fn conjugate(self) -> VGA3DTrivector {
        self
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    pub fn involution(self) -> VGA3DTrivector {
        -self
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
// Inverse
// \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
impl VGA3DTrivector {
    pub fn inverse(self) -> VGA3DTrivector {
        self.reverse() * (1.0 / (self * self.reverse()))
    }
}

// Regressive Product
// \[ (A \vee B)\star = ( A\star  \wedge B\star ) \]
impl VGA3DTrivector {
    pub fn regressive(self) -> VGA3DTrivector {
        // TODO
        self
    }
}

impl VGA3DTrivector {
    pub fn norm(self) -> f32 {
        sqrtf(self.reverse() * self)
    }
}
