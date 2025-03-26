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
// #![warn(rustdoc::missing_doc_code_examples)]

use crate::forward_ref_binop;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use libm::{acosf, cosf, sinf, sqrtf};

use super::{
    bivector::Bivector, multivector::Multivector, trivector::Trivector, vector::Vector, VGA3DOps,
    VGA3DOpsRef,
};

/// # 3D Vector Geometric Algebra Rotor
///
/// The rotor is the rotation object in Geometric Algebra
/// $$ R\left (\frac{\theta}{2},\overset\Rightarrow{b} \right ) = \mathrm{e}^{ \overset\Rightarrow{b} \frac{\theta}{2}} = \cos \left( \frac{\theta}{2}  \right) + \sin \left( \frac{\theta}{2} \right)(b_1 \mathrm{e}_1\mathrm{e}_2 + b_2 \mathrm{e}_3\mathrm{e}_1 + b_3 \mathrm{e}_2\mathrm{e}_3) $$
/// The norm of a rotor is always 1
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Rotor {
    scalar: f32,
    bivector: Bivector,
}

impl Rotor {
    /// Creates new rotor from plane of rotation and angle of rotationen
    /// The plane of rotation is a bivector
    /// The direction of rotation is given by the orientation of the bivector
    /// The angle is half of the rotation angle
    pub fn new(angle: f32, rotation_plane: Bivector) -> Self {
        let scalar = cosf(angle);
        let sin = sinf(angle);
        let norm = rotation_plane.norm();
        let bi_pre = sin / norm;

        let bivector = bi_pre * rotation_plane;
        Self { scalar, bivector }
    }

    /// Creates new rotor from an angle bivector $\overset\Rightarrow{\theta}$.
    /// The angle must be in radians and must be half the rotational angle.
    pub fn new_angle_bivector(angle_bivector: Bivector) -> Self {
        let norm = angle_bivector.norm();
        let rotation_plane = angle_bivector * (1.0 / norm);
        Rotor::new(norm, rotation_plane)
    }

    /// Get the scalar grade of the rotor
    pub fn scalar(&self) -> f32 {
        self.scalar
    }

    /// Get the bivector grade of the rotor
    pub fn bivector(&self) -> Bivector {
        self.bivector
    }

    /// Get the $\mathrm{e}_1\mathrm{e}_2$ part of the bivector grade of the rotor
    pub fn e12(&self) -> f32 {
        self.bivector.e12()
    }

    /// Get the $\mathrm{e}_3\mathrm{e}_1$ part of the bivector grade of the rotor
    pub fn e31(&self) -> f32 {
        self.bivector.e31()
    }

    /// Get the $\mathrm{e}_2\mathrm{e}_3$ part of the bivector grade of the rotor
    pub fn e23(&self) -> f32 {
        self.bivector.e23()
    }

    /// Get the angle of the rotor
    pub fn angle(&self) -> f32 {
        acosf(self.scalar())
    }

    /// Get the plane of rotation of the rotor
    pub fn rotatino_plane(&self) -> Bivector {
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
        let rotation_plane = Bivector::new(4.0, 2.0, -3.0);
        let rotor = Rotor::new(angle / 2.0, rotation_plane);
        // 0.70710677+0.52522576e12+0.26261288e31-0.3939193e23
        assert_relative_eq!(rotor.scalar(), 0.70710677, max_relative = 0.000001);
        assert_relative_eq!(rotor.e12(), 0.52522576, max_relative = 0.000001);
        assert_relative_eq!(rotor.e31(), 0.26261288, max_relative = 0.000001);
        assert_relative_eq!(rotor.e23(), -0.3939193, max_relative = 0.000001);
    }

    #[test]
    fn get_angle() {
        let rotation_angle = TAU / 4.0;
        let rotation_plane = Bivector::new(4.0, 2.0, -3.0);
        let rotor = Rotor::new(rotation_angle / 2.0, rotation_plane);
        assert_relative_eq!(rotor.angle(), rotation_angle / 2.0, max_relative = 0.000001);
    }

    #[test]
    fn get_plane_of_rotation() {
        let angle = TAU / 4.0;
        let rotation_plane = Bivector::new(4.0, 2.0, -3.0);
        let norm = rotation_plane.norm();
        let rotor = Rotor::new(angle / 2.0, rotation_plane);
        assert_relative_eq!(
            rotor.rotatino_plane().e12() * norm,
            rotation_plane.e12(),
            max_relative = 0.000001
        );
        assert_relative_eq!(
            rotor.rotatino_plane().e31() * norm,
            rotation_plane.e31(),
            max_relative = 0.000001
        );
        assert_relative_eq!(
            rotor.rotatino_plane().e23() * norm,
            rotation_plane.e23(),
            max_relative = 0.000001
        );
    }
}

/// # Geometric Product
/// The geometric product of two rotors is another rotor
/// $$ R_1 R_2 = R_3$$
impl Mul for Rotor {
    type Output = Rotor;

    fn mul(self: Rotor, b: Rotor) -> Rotor {
        let a = self.bivector * b.bivector;
        let scalar = self.scalar * b.scalar + a.scalar();
        let bivector = self.scalar * b.bivector + self.bivector * b.scalar + a.bivector();

        // Normelize
        let norm = sqrtf(scalar * scalar + (bivector * bivector.reverse()).scalar());
        Rotor {
            scalar: scalar * norm,
            bivector: bivector * norm,
        }
    }
}
forward_ref_binop!(impl Mul, mul for Rotor, Rotor);

#[cfg(test)]
mod rotor_geo {
    use core::f32::consts::TAU;

    use crate::vga3d::bivector;

    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn rotor_rotor_geo() {
        let angle1 = TAU / 4.0;
        let rotation_plane1 = Bivector::new(1.0, 0.0, 0.0);
        let rotor1 = Rotor::new(angle1 / 2.0, rotation_plane1);
        let angle2 = TAU / 4.0;
        let rotation_plane2 = Bivector::new(0.0, 1.0, 0.0);
        let rotor2 = Rotor::new(angle2 / 2.0, rotation_plane2);

        let res_rotor = rotor1 * rotor2;
        assert_relative_eq!(res_rotor.scalar(), 0.5, max_relative = 0.000001);
        assert_relative_eq!(res_rotor.e12(), 0.5, max_relative = 0.000001);
        assert_relative_eq!(res_rotor.e31(), 0.5, max_relative = 0.000001);
        assert_relative_eq!(res_rotor.e23(), 0.5, max_relative = 0.000001);
    }
}

impl VGA3DOps for Rotor {
    // \[ |R|^2=\left< R^\dag A \right>_0 \]
    fn norm(self) -> f32 {
        sqrtf(
            (self.scalar() * self.scalar())
                + (self.bivector() * self.bivector().reverse()).scalar(),
        )
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(self) -> Rotor {
        let a = 1.0 / (self * self.reverse()).scalar();
        Rotor {
            scalar: self.reverse().scalar * a,
            bivector: self.reverse().bivector * a,
        }
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(self) -> Rotor {
        Rotor {
            scalar: self.scalar,
            bivector: -self.bivector,
        }
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(self) -> Rotor {
        Rotor {
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

impl VGA3DOpsRef for Rotor {
    fn norm(&self) -> f32 {
        sqrtf(
            (self.scalar() * self.scalar())
                + (self.bivector() * self.bivector().reverse()).scalar(),
        )
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn inverse(&self) -> Rotor {
        let a = 1.0 / (self * self.reverse()).scalar();
        Rotor {
            scalar: self.reverse().scalar * a,
            bivector: self.reverse().bivector * a,
        }
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(&self) -> Rotor {
        Rotor {
            scalar: self.scalar,
            bivector: -self.bivector,
        }
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(&self) -> Rotor {
        Rotor {
            scalar: self.scalar,
            bivector: -self.bivector,
        }
    }
    // Grade Involution
    // The follows this patten (Each is a grade)
    // \[+ - + - + -\dots (-1)^{k}\]

    fn involute(&self) -> Self {
        Rotor {
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
        let rotation_plane1 = Bivector::new(3.0, 2.0, 10.0);
        let rotor1 = Rotor::new(angle1, rotation_plane1);

        let angle2 = TAU / 2.0;
        let rotation_plane2 = Bivector::new(2.0, -3.0, -1.0);
        let rotor2 = Rotor::new(angle2, rotation_plane2);

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
