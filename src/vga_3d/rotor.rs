#![allow(unused_imports)]
#![allow(dead_code)]

use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};
use forward_ref::forward_ref_binop;

use libm::{acosf, cosf, sinf, sqrtf};

use super::{
    bivector::VGA3DBivector, multivector::VGA3DMultivector, trivector::VGA3DTrivector,
    vector::VGA3DVector, VGA3DOps, VGA3DOpsRef,
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct VGA3DRotor {
    scalar: f32,
    bivector: VGA3DBivector,
}

impl VGA3DRotor {
    pub fn zero() -> Self {
        Self {
            scalar: 0.0,
            bivector: VGA3DBivector::zero(),
        }
    }

    pub fn new(angle: f32, rotation_plane: VGA3DBivector) -> Self {
        let scalar = cosf(angle / 2.0);
        let sin = sinf(angle / 2.0);
        let norm = rotation_plane.norm();
        let bi_pre = sin * norm;

        let bivector = bi_pre * rotation_plane;
        Self { scalar, bivector }
    }

    pub fn scalar(&self) -> f32 {
        self.scalar
    }

    pub fn bivector(&self) -> VGA3DBivector {
        self.bivector
    }

    pub fn angle(&self) -> f32 {
        acosf(self.scalar())
    }

    pub fn rotatino_plane(&self) -> VGA3DBivector {
        let sin = sinf(self.angle());
        self.bivector * (1.0 / sin)
    }
}

// Addition
impl Add for VGA3DRotor {
    type Output = VGA3DMultivector;

    fn add(self: VGA3DRotor, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = self.scalar + b.scalar;
        let bivector = self.bivector + b.bivector;
        VGA3DMultivector::new(
            scalar,
            VGA3DVector::zero(),
            bivector,
            VGA3DTrivector::zero(),
        )
    }
}

// Subtration
impl Sub for VGA3DRotor {
    type Output = VGA3DMultivector;

    fn sub(self: VGA3DRotor, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = self.scalar - b.scalar;
        let bivector = self.bivector - b.bivector;
        VGA3DMultivector::new(
            scalar,
            VGA3DVector::zero(),
            bivector,
            VGA3DTrivector::zero(),
        )
    }
}

// Geometric Product
// \[ R_1 R_2 \]
impl Mul for VGA3DRotor {
    type Output = VGA3DRotor;

    fn mul(self: VGA3DRotor, b: VGA3DRotor) -> VGA3DRotor {
        let a = self.bivector * b.bivector;
        let scalar = self.scalar * b.scalar + a.scalar();
        let bivector = self.scalar * b.bivector + self.bivector * b.scalar + a.bivector();
        VGA3DRotor::new(scalar, bivector)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DRotor, VGA3DRotor);

// #[cfg(test)]
// mod rotor_geo {
//     use super::*;
//     use approx::assert_relative_eq;
//     #[test]
//     fn rotor_rotor_geo() {
//         let rotor1 = VGA3DRotor::new(3.0, 5.0, 4.0);
//         let rotor2 = VGA3DRotor::new(2.0, 1.0, 6.0);
//         let scalar = vector1 | vector2;
//         assert_relative_eq!(scalar, 35.0, max_relative = 0.000001);
//     }
// }

// Exterior Product
// \[ R_1 \wedge R_2\]
impl BitXor for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DRotor, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = (self.scalar() * b.scalar()) + (self.bivector() ^ b.bivector());
        let vector = VGA3DVector::zero();
        let bivector = (self.scalar() ^ b.bivector()) + (self.bivector() ^ b.scalar());
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Inner Product
// \[ R_1 \cdot R_2\]
impl BitOr for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DRotor, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = (self.scalar() * b.scalar()) + (self.bivector() | b.bivector());
        let vector = VGA3DVector::zero();
        let bivector = (self.scalar() | b.bivector()) + (self.bivector() | b.scalar());
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

impl VGA3DOps for VGA3DRotor {
    fn norm(self) -> f32 {
        sqrtf((self.reverse() * self).scalar())
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(self) -> VGA3DRotor {
        let a = 1.0 / (self * self.reverse()).scalar();
        VGA3DRotor {
            scalar: self.reverse().scalar * a,
            bivector: self.reverse().bivector * a,
        }
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(self) -> VGA3DRotor {
        VGA3DRotor {
            scalar: self.scalar,
            bivector: -self.bivector,
        }
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(self) -> VGA3DRotor {
        VGA3DRotor {
            scalar: self.scalar,
            bivector: -self.bivector,
        }
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(self) -> Self {
        self
    }
}

impl VGA3DOpsRef for VGA3DRotor {
    fn norm(&self) -> f32 {
        sqrtf((self.reverse() * self).scalar())
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(&self) -> VGA3DRotor {
        let a = 1.0 / (self * self.reverse()).scalar();
        VGA3DRotor {
            scalar: self.reverse().scalar * a,
            bivector: self.reverse().bivector * a,
        }
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(&self) -> VGA3DRotor {
        VGA3DRotor {
            scalar: self.scalar,
            bivector: -self.bivector,
        }
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(&self) -> VGA3DRotor {
        VGA3DRotor {
            scalar: self.scalar,
            bivector: -self.bivector,
        }
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(&self) -> Self {
        VGA3DRotor {
            scalar: self.scalar,
            bivector: self.bivector,
        }
    }
}
