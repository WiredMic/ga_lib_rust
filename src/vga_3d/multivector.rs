#![allow(unused_imports)]
#![allow(dead_code)]

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use libm::sqrtf;

use super::bivector::{self, VGA3DBivector};
use super::trivector::{self, VGA3DTrivector};
use super::vector::{self, VGA3DVector};

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

// Geometric Product
impl Mul for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DMultivector, b: VGA3DMultivector) -> VGA3DMultivector {
        let scalar_multi = self.scalar * b;
        let vec_multi = self.vector * b;
        let bivec_multi = self.bivector * b;
        let trivec_multi = self.trivector * b;

        scalar_multi + vec_multi + bivec_multi + trivec_multi
    }
}

#[cfg(test)]
mod mvec_mvec_mul {
    // https://bivector.net/tools.html?p=3&q=0&r=0
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn mvec_mvec_mul() {
        let mvec1 = VGA3DMultivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        let mvec2 = VGA3DMultivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        let mvec_res = mvec1 * mvec2;
        // 94+126e1​−5e2​−65e3​+23e12​−131e13​+158e23​+236e123
        assert_relative_eq!(mvec_res.scalar(), 94.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), 126.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), -5.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), -65.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 23.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 131.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 158.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 236.0, max_relative = 0.000001);
    }
    #[test]
    fn negetive_mvec_mvec_mul() {
        // let mvec1 = GaMultivector::new_mvec(-6.0, -8.0, -4.0, -1.0, -6.0, -4.0, -8.0, -5.0);
        let mvec1 =
            VGA3DMultivector::new_components(-4.0, -1.0, -3.0, -2.0, -9.0, -6.0, -3.0, -10.0);
        let mvec2 =
            VGA3DMultivector::new_components(-4.0, -2.0, -4.0, -9.0, -2.0, -1.0, -7.0, -1.0);
        let mvec_res = mvec1 * mvec2;
        // −7−83e1​+9e2​+35e3​+173e12​−9e13​+77e23​+169e123
        assert_relative_eq!(mvec_res.scalar(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), -83.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), 9.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), 35.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 173.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 9.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 77.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 169.0, max_relative = 0.000001);
    }
}

// // Inner Product / Dot Product
// // In 3D there the geometric product of two trivectors is there inner product
// // \[ A \cdot B = \left <A B \right>_{|a-b|} \]
// impl VGA3DMultivector {
//     pub fn inner(self, b: VGA3DMultivector) -> VGA3DMultivector {
//         self
//     }
// }
// impl BitOr for VGA3DMultivector {
//     type Output = VGA3DMultivector;

//     fn bitor(self: VGA3DMultivector, b: VGA3DMultivector) -> VGA3DMultivector {
//         self.inner(b)
//     }
// }

// // Exterior Product / Wedge Product
// // \[ A \wedge B = \left <A B \right>_{a+b} \]
// impl VGA3DMultivector {
//     pub fn exterior(self, _b: VGA3DMultivector) -> VGA3DMultivector {
//         self
//     }
// }
// impl BitXor for VGA3DMultivector {
//     type Output = VGA3DMultivector;

//     fn bitxor(self: VGA3DMultivector, b: VGA3DMultivector) -> VGA3DMultivector {
//         self.exterior(b)
//     }
// }

// // Cross Product
// // It does not make sence to take the cross product of two multvectors
// // impl VGA3DTrivector {
// //     pub fn cross(self, _b: VGA3DTrivector) -> f32 {
// //         0.0
// //     }
// // }

// Others
impl VGA3DMultivector {
    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    pub fn reverse(self) -> VGA3DMultivector {
        let scalar = self.scalar;
        let vector = self.vector;
        let bivector = -self.bivector;
        let trivector = -self.trivector;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    pub fn conjugate(self) -> VGA3DMultivector {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = -self.bivector;
        let trivector = self.trivector;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    pub fn involution(self) -> VGA3DMultivector {
        let scalar = self.scalar;
        let vector = -self.vector;
        let bivector = self.bivector;
        let trivector = -self.trivector;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// // Dual
// // In VGA 3D, the dual is the pseudoscalar
// // \[ \overset\Rightarrow{b} \overset\Rrightarrow{i} = -\vec{v} \]
// // vector and bivectors in 3D VGA follows this pattern. Going up, going down
// // \[ \mathrm{e}_1,\,\mathrm{e}_2,\,\mathrm{e}_3,\,\mathrm{e}_3\star,\,\mathrm{e}_2\star,\,\mathrm{e}_1\star,\, \]
// impl VGA3DTrivector {
//     pub fn dual(self) -> f32 {
//         -self.e123
//     }
// }
// Inverse
// \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
impl VGA3DMultivector {
    pub fn inverse(self) -> VGA3DMultivector {
        self.reverse() * (1.0 / (self * self.reverse()).scalar)
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

// the norm of a multivector |A|
// \[|A|^2=\left< A\^dag A \right>_0\]
impl VGA3DMultivector {
    pub fn norm(self) -> f32 {
        sqrtf((self.reverse() * self).scalar())
    }
}
