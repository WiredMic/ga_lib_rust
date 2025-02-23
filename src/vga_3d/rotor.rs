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
        let bi_pre = sin / norm;

        let bivector = bi_pre * rotation_plane;
        Self { scalar, bivector }
    }

    pub fn scalar(&self) -> f32 {
        self.scalar
    }

    pub fn bivector(&self) -> VGA3DBivector {
        self.bivector
    }

    pub fn e12(&self) -> f32 {
        self.bivector.e12()
    }

    pub fn e31(&self) -> f32 {
        self.bivector.e31()
    }

    pub fn e23(&self) -> f32 {
        self.bivector.e23()
    }

    pub fn angle(&self) -> f32 {
        acosf(self.scalar())
    }

    pub fn rotatino_plane(&self) -> VGA3DBivector {
        let sin = sinf(self.angle());
        self.bivector * (1.0 / sin)
    }
}

#[cfg(test)]
mod rotor {
    use super::*;
    use approx::assert_relative_eq;
    use core::f32::consts::TAU;

    #[test]
    fn new() {
        let angle = TAU / 4.0;
        let rotation_plane = VGA3DBivector::new(4.0, 2.0, -3.0);
        let rotor = VGA3DRotor::new(angle, rotation_plane);
        // 0.70710677+0.52522576e12+0.26261288e31-0.3939193e23
        assert_relative_eq!(rotor.scalar(), 0.70710677, max_relative = 0.000001);
        assert_relative_eq!(rotor.e12(), 0.52522576, max_relative = 0.000001);
        assert_relative_eq!(rotor.e31(), 0.26261288, max_relative = 0.000001);
        assert_relative_eq!(rotor.e23(), -0.3939193, max_relative = 0.000001);
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

        // Normelize
        let norm = sqrtf(scalar * scalar + (bivector * bivector.reverse()).scalar());
        VGA3DRotor {
            scalar: scalar * norm,
            bivector: bivector * norm,
        }
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DRotor, VGA3DRotor);

#[cfg(test)]
mod rotor_geo {
    use core::f32::consts::TAU;

    use crate::vga_3d::bivector;

    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn rotor_rotor_geo() {
        let angle1 = TAU / 4.0;
        let rotation_plane1 = VGA3DBivector::new(1.0, 0.0, 0.0);
        let rotor1 = VGA3DRotor::new(angle1, rotation_plane1);
        let angle2 = TAU / 4.0;
        let rotation_plane2 = VGA3DBivector::new(0.0, 1.0, 0.0);
        let rotor2 = VGA3DRotor::new(angle2, rotation_plane2);

        let res_rotor = rotor1 * rotor2;
        assert_relative_eq!(res_rotor.scalar(), 0.5, max_relative = 0.000001);
        assert_relative_eq!(res_rotor.e12(), 0.5, max_relative = 0.000001);
        assert_relative_eq!(res_rotor.e31(), 0.5, max_relative = 0.000001);
        assert_relative_eq!(res_rotor.e23(), 0.5, max_relative = 0.000001);
    }
}

impl VGA3DOps for VGA3DRotor {
    // \[ |R|^2=\left< R^\dag A \right>_0 \]
    fn norm(self) -> f32 {
        sqrtf(
            (self.scalar() * self.scalar())
                + (self.bivector() * self.bivector().reverse()).scalar(),
        )
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
        sqrtf(
            (self.scalar() * self.scalar())
                + (self.bivector() * self.bivector().reverse()).scalar(),
        )
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

#[cfg(test)]
mod rotor_reverse {
    use super::*;
    use approx::assert_relative_eq;
    use core::f32::consts::TAU;
    // The reverse of the geometric product of to rotors is the geometric product of the reverse rotors flipped
    // \[ (R_1R_2)^\dag = R_2^\dag R_1^\dag\]
    #[test]
    fn rotor_rotor_reverse() {
        let angle1 = TAU / 4.0;
        let rotation_plane1 = VGA3DBivector::new(3.0, 2.0, 10.0);
        let rotor1 = VGA3DRotor::new(angle1, rotation_plane1);

        let angle2 = TAU / 2.0;
        let rotation_plane2 = VGA3DBivector::new(2.0, -3.0, -1.0);
        let rotor2 = VGA3DRotor::new(angle2, rotation_plane2);

        let rotor_reverse = (rotor1 * rotor2).reverse();
        let reverse_rotor = rotor2.reverse() * rotor1.reverse();

        assert_relative_eq!(
            rotor_reverse.scalar(),
            reverse_rotor.scalar(),
            max_relative = 0.000001
        );
        assert_relative_eq!(
            rotor_reverse.e12(),
            reverse_rotor.e12(),
            max_relative = 0.000001
        );
        assert_relative_eq!(
            rotor_reverse.e31(),
            reverse_rotor.e31(),
            max_relative = 0.000001
        );
        assert_relative_eq!(
            rotor_reverse.e23(),
            reverse_rotor.e23(),
            max_relative = 0.000001
        );
    }
}
