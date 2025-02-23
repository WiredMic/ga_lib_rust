#![allow(unused_imports)]
#![allow(dead_code)]

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use libm::sqrtf;

//
use super::{
    bivector::VGA3DBivector, multivector::VGA3DMultivector, trivector::VGA3DTrivector, VGA3DOps,
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct VGA3DVector {
    e1: f32,
    e2: f32,
    e3: f32,
}

impl VGA3DVector {
    pub fn zero() -> Self {
        Self {
            e1: 0.0,
            e2: 0.0,
            e3: 0.0,
        }
    }

    pub fn new(e1: f32, e2: f32, e3: f32) -> Self {
        Self { e1, e2, e3 }
    }

    // Vector components
    pub fn e1(&self) -> f32 {
        self.e1
    }

    pub fn e2(&self) -> f32 {
        self.e2
    }

    pub fn e3(&self) -> f32 {
        self.e3
    }
}

#[cfg(test)]
mod vector_new {
    use super::*;

    #[test]
    fn vector_new() {
        let vec = VGA3DVector::new(2.0, 1.0, 1.0);
        assert_eq!(vec.e1, 2.0);
        assert_eq!(vec.e2, 1.0);
        assert_eq!(vec.e3, 1.0);
    }
}

// Addition
impl Add for VGA3DVector {
    type Output = VGA3DVector;

    fn add(self: VGA3DVector, b: VGA3DVector) -> VGA3DVector {
        let mut res = VGA3DVector::zero();
        res.e1 = self.e1 + b.e1;
        res.e2 = self.e2 + b.e2;
        res.e3 = self.e3 + b.e3;
        res
    }
}

// Subtraction
impl Sub for VGA3DVector {
    type Output = VGA3DVector;

    fn sub(self: VGA3DVector, b: VGA3DVector) -> VGA3DVector {
        let mut res = VGA3DVector::zero();
        res.e1 = self.e1 - b.e1;
        res.e2 = self.e2 - b.e2;
        res.e3 = self.e3 - b.e3;
        res
    }
}

// Negation
impl Neg for VGA3DVector {
    type Output = VGA3DVector;
    fn neg(self) -> VGA3DVector {
        VGA3DVector::new(-self.e1, -self.e2, -self.e3)
    }
}

// Geometric Product

// Inner Product / Dot Product
// \[ \vec{u} \cdot \vec{v} = u_1 \cdot v_1 + u_2 \cdot v_2 + u_3 \cdot v_3 \]
impl VGA3DVector {
    // TODO This only works with vector. fix?
    pub fn inner(self, b: VGA3DVector) -> f32 {
        self.e1 * b.e1 + self.e2 * b.e2 + self.e3 * b.e3
    }
}
impl BitOr for VGA3DVector {
    type Output = f32;

    fn bitor(self: VGA3DVector, b: VGA3DVector) -> f32 {
        self.inner(b)
    }
}
#[cfg(test)]
mod vector_dot {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn vector_vector_dot() {
        // 3e1+5e2+4e3
        let vector1 = VGA3DVector::new(3.0, 5.0, 4.0);
        // 2e1+1e2+6e3
        let vector2 = VGA3DVector::new(2.0, 1.0, 6.0);
        let scalar = vector1 | vector2;
        assert_relative_eq!(scalar, 35.0, max_relative = 0.000001);
    }
}

// Exterior Product / Wedge Product
impl VGA3DVector {
    pub fn exterior(self, b: VGA3DVector) -> VGA3DBivector {
        let e12 = self.e1 * b.e2 - self.e2 * b.e1;
        let e31 = self.e3 * b.e1 - self.e1 * b.e3;
        let e23 = self.e2 * b.e3 - self.e3 * b.e2;
        VGA3DBivector::new(e12, e31, e23)
    }
}
impl BitXor for VGA3DVector {
    type Output = VGA3DBivector;

    fn bitxor(self: VGA3DVector, b: VGA3DVector) -> VGA3DBivector {
        self.exterior(b)
    }
}
#[cfg(test)]
mod vector_exterior {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn vector_vector_wedge() {
        // 3e1+5e2+4e3
        let vector1 = VGA3DVector::new(3.0, 5.0, 4.0);
        // 2e1+1e2+6e3
        let vector2 = VGA3DVector::new(2.0, 1.0, 6.0);
        let bivector = vector1 ^ vector2;
        // −7e12​-10e31​+26e23
        assert_relative_eq!(bivector.e12(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(bivector.e31(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(bivector.e23(), 26.0, max_relative = 0.000001);
    }
}

// Cross Product
// The cross product is the dual of the exterior product
// \[\vec{v}\times\vec{u} = (\vec{v}\wedge\vec{u})\star \]
impl VGA3DVector {
    pub fn cross(self, b: VGA3DVector) -> VGA3DVector {
        // -(self ^ b).dual()
        let e1 = self.e2() * b.e3() - self.e3() * b.e2();
        let e2 = self.e3() * b.e1() - self.e1() * b.e3();
        let e3 = self.e1() * b.e2() - self.e2() * b.e1();
        VGA3DVector::new(e1, e2, e3)
    }
}
#[cfg(test)]
mod vector_cross {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn vector_vector_cross() {
        // 3e1+5e2+4e3
        let vector1 = VGA3DVector::new(3.0, 5.0, 4.0);
        // 2e1+1e2+6e3
        let vector2 = VGA3DVector::new(2.0, 1.0, 6.0);
        let cross = vector1.cross(vector2);
        // −7e12​-10e31​+26e23
        assert_relative_eq!(cross.e1(), 26.0, max_relative = 0.000001);
        assert_relative_eq!(cross.e2(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(cross.e3(), -7.0, max_relative = 0.000001);
    }
}

// Dual
// In VGA 3D, the dual is the pseudoscalar
// In VGA 3D, the dual is the pseudoscalar
// \[ \vec{v} \overset\Rrightarrow{i} = \overset\Rightarrow{b} \]
// vector and bivectors in 3D VGA follows this pattern. Going up, going down
// \[ \mathrm{e}_1,\,\mathrm{e}_2,\,\mathrm{e}_3,\,\mathrm{e}_3\star,\,\mathrm{e}_2\star,\,\mathrm{e}_1\star,\, \]
impl VGA3DVector {
    pub fn dual(self) -> VGA3DBivector {
        VGA3DBivector::new(self.e3, self.e2, self.e1)
    }
}

#[cfg(test)]
mod vector_dual {
    use super::*;
    #[test]
    fn vector_to_bivector() {
        let vector: VGA3DVector = VGA3DVector::new(1.0, 2.0, 3.0);
        let bivector: VGA3DBivector = vector.dual();
        assert_eq!(vector.e1(), bivector.e23());
        assert_eq!(vector.e2(), bivector.e31());
        assert_eq!(vector.e3(), bivector.e12());
    }
}

// Regressive Product
// \[ (A \vee B)\star = ( A\star  \wedge B\star ) \]
impl VGA3DVector {
    pub fn regressive(self) -> VGA3DVector {
        // TODO
        self
    }
}

impl VGA3DOps for VGA3DVector {
    fn norm(self) -> f32 {
        sqrtf((self.reverse() * self).scalar())
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(self) -> VGA3DVector {
        self.reverse() * (1.0 / (self * self.reverse()).scalar())
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(self) -> VGA3DVector {
        self
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(self) -> VGA3DVector {
        -self
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(self) -> Self {
        -self
    }
}
