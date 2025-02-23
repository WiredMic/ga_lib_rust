#![allow(unused_imports)]
#![allow(dead_code)]

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use libm::sqrtf;

use super::{
    multivector::VGA3DMultivector,
    trivector::{self, VGA3DTrivector},
    vector::VGA3DVector,
    VGA3DOps, VGA3DOpsRef,
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct VGA3DBivector {
    e12: f32,
    e31: f32,
    e23: f32,
}

impl VGA3DBivector {
    pub fn zero() -> Self {
        Self {
            e12: 0.0,
            e31: 0.0,
            e23: 0.0,
        }
    }

    pub fn new(e12: f32, e31: f32, e23: f32) -> Self {
        Self { e12, e31, e23 }
    }

    pub fn bivector(self) -> Self {
        self
    }

    pub fn e12(&self) -> f32 {
        self.e12
    }

    pub fn e23(&self) -> f32 {
        self.e23
    }

    pub fn e31(&self) -> f32 {
        self.e31
    }
}

#[cfg(test)]
mod new {
    use super::*;

    #[test]
    fn new() {
        let bivec = VGA3DBivector::new(2.0, 1.0, 1.0);
        assert_eq!(bivec.e12, 2.0);
        assert_eq!(bivec.e31, 1.0);
        assert_eq!(bivec.e23, 1.0);
    }
}

// Negation
impl Neg for VGA3DBivector {
    type Output = VGA3DBivector;
    fn neg(self) -> VGA3DBivector {
        VGA3DBivector::new(-self.e12, -self.e31, -self.e23)
    }
}

// Cross Product
// The cross product for two bivectors gives the bivector orthogonal to both
// \[ \overset\Rightarrow{a} \times \overset\Rightarrow{b} = \left <\overset\Rightarrow{a} \overset\Rightarrow{b} \right>_2 \]
impl VGA3DBivector {
    pub fn cross(self, b: VGA3DBivector) -> VGA3DBivector {
        let e12 = self.e31 * b.e23 - self.e23 * b.e31;
        let e31 = self.e23 * b.e12 - self.e12 * b.e23;
        let e23 = self.e12 * b.e31 - self.e31 * b.e12;
        VGA3DBivector::new(e12, e31, e23)
    }
}

#[cfg(test)]
mod bivector_cross {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn bivector_bivector_cross() {
        // 3e1+5e2+4e3
        let bivector1 = VGA3DBivector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let bivector2 = VGA3DBivector::new(2.0, 1.0, 6.0);
        //
        let res_bivector = VGA3DBivector::new(26.0, -10.0, -7.0);
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

// Dual
// In VGA 3D, the dual is the pseudoscalar
// \[ \overset\Rightarrow{b} \overset\Rrightarrow{i} = -\vec{v} \]
// vector and bivectors in 3D VGA follows this pattern. Going up, going down
// \[ \mathrm{e}_1,\,\mathrm{e}_2,\,\mathrm{e}_3,\,\mathrm{e}_3\star,\,\mathrm{e}_2\star,\,\mathrm{e}_1\star,\, \]
impl VGA3DBivector {
    pub fn dual(self) -> VGA3DVector {
        VGA3DVector::new(-self.e23, -self.e31, -self.e12)
    }
}

// #[cfg(test)]
// mod vector_cross {
//     use super::*;
//     use approx::assert_relative_eq;
//     #[test]
//     fn vector_vector_cross() {
//     }}

// Inverse
// \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
impl VGA3DBivector {
    pub fn inverse(self) -> VGA3DBivector {
        self.reverse() * (1.0 / (self * self.reverse()).scalar())
    }
}

// Regressive Product
// \[ (A \vee B)\star = ( A\star  \wedge B\star ) \]
impl VGA3DBivector {
    pub fn regressive(self) -> VGA3DBivector {
        // TODO
        self
    }
}

impl VGA3DBivector {
    pub fn norm(self) -> f32 {
        sqrtf((self.reverse() * self).scalar())
    }
}

impl VGA3DOps for VGA3DBivector {
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

impl VGA3DOpsRef for VGA3DBivector {
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
