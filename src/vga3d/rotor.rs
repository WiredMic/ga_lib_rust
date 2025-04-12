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

use crate::forward_ref_binop;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use num_traits::Float;

// use libm::{acosf, cosf, sinf, sqrtf};

use super::{
    bivector::Bivector, multivector::Multivector, scalar::Scalar, trivector::Trivector,
    vector::Vector, VGA3DOps, VGA3DOpsRef,
};

/// # 3D Vector Geometric Algebra Rotor
///
/// The rotor is the rotation object in Geometric Algebra
/// $$ R\left (\frac{\theta}{2},\overset\Rightarrow{b} \right ) = \mathrm{e}^{ \overset\Rightarrow{b} \frac{\theta}{2}} = \cos \left( \frac{\theta}{2}  \right) + \sin \left( \frac{\theta}{2} \right)(b_1 \mathrm{e}_1\mathrm{e}_2 + b_2 \mathrm{e}_3\mathrm{e}_1 + b_3 \mathrm{e}_2\mathrm{e}_3) $$
/// The norm of a rotor is always 1
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Rotor<F: Float> {
    pub(super) scalar: Scalar<F>,
    pub(super) bivector: Bivector<F>,
}

#[cfg(feature = "std")]
impl<F: Float + fmt::Display> fmt::Display for Rotor<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}e12, {}e31, {}e23", self.e12, self.e31, self.e23)

        write!(f, "Rotor {{\n")?;
        write!(f, "\tscalar: {}\n", self.scalar())?;
        write!(f, "\tbivector: {}\n", self.bivector)?;
        write!(f, "}}")?;

        Ok(())
    }
}

#[cfg(feature = "defmt")]
impl<F: Float + defmt::Format> defmt::Format for Rotor<F> {
    fn format(&self, f: defmt::Formatter) {
        // write!(f, "{}e12, {}e31, {}e23", self.e12, self.e31, self.e23)

        defmt::write!(f, "Rotor {{\n");
        defmt::write!(f, "\tscalar: {}\n", self.scalar());
        defmt::write!(f, "\tbivector: {}\n", self.bivector);
        defmt::write!(f, "}}");
    }
}

impl<F: Float> Rotor<F> {
    /// Creates new rotor from an angle bivector $\overset\Rightarrow{\theta}$.
    /// The angle must be in radians and must be half the rotational angle.
    pub fn new(half_angle_bivector: Bivector<F>) -> Self {
        let half_angle = half_angle_bivector.norm().scalar();

        // The only time a angle bivectors norm is zero is when the angle is zero
        // cos(0.0) = 1.0
        // sin(0.0) = 0.0
        if half_angle == F::zero() {
            return Rotor {
                scalar: Scalar(F::one()),
                bivector: Bivector::new(F::zero(), F::zero(), F::zero()),
            };
        }
        // Normilize the rotation plane
        // multiply by the sin of the half angle
        let bi_pre = Scalar(half_angle.sin() / half_angle);
        Rotor {
            scalar: Scalar(half_angle.cos()),
            bivector: half_angle_bivector * bi_pre,
        }
    }

    /// Tries to creates new rotor from plane of rotation and angle of rotationen
    /// The plane of rotation is a bivector
    /// The direction of rotation is given by the orientation of the bivector
    /// The angle is half of the rotation angle
    pub fn try_new_from_half_angle_plane(
        half_angle: F,
        rotation_plane: Bivector<F>,
    ) -> Option<Self> {
        // test if the bivector is the unit bivector
        let bivector_norm = rotation_plane.norm().scalar();
        match bivector_norm {
            bivector_norm if bivector_norm.is_zero() => return None,
            _ => {
                let bi_pre = Scalar(half_angle.sin() / bivector_norm);
                return Some(Rotor {
                    scalar: Scalar(half_angle.cos()),
                    bivector: rotation_plane * bi_pre,
                });
            }
        }
    }

    /// This is the identity Rotor.
    /// Anything rotated with this (0.0 rad) will return it self.
    /// cos(0.0) = 1.0
    /// sin(0.0) = 0.0
    pub fn identity() -> Self {
        Rotor {
            scalar: Scalar(F::one()),
            bivector: Bivector::zero(),
        }
    }

    /// Get the scalar grade of the rotor
    pub fn scalar(&self) -> F {
        self.scalar.0
    }

    /// Get the bivector grade of the rotor
    pub fn bivector(&self) -> Bivector<F> {
        self.bivector
    }

    /// Get the $\mathrm{e}_1\mathrm{e}_2$ part of the bivector grade of the rotor
    pub fn e12(&self) -> F {
        self.bivector.e12()
    }

    /// Get the $\mathrm{e}_3\mathrm{e}_1$ part of the bivector grade of the rotor
    pub fn e31(&self) -> F {
        self.bivector.e31()
    }

    /// Get the $\mathrm{e}_2\mathrm{e}_3$ part of the bivector grade of the rotor
    pub fn e23(&self) -> F {
        self.bivector.e23()
    }

    /// Get the angle of the rotor
    pub fn get_half_angle(&self) -> Scalar<F> {
        Scalar(self.scalar().acos())
    }

    /// Get the plane of rotation of the rotor
    /// This is not always posible
    /// sin(0) = sin(tau/2) = 0
    pub fn try_get_rotation_plane(&self) -> Bivector<F> {
        let sin = Scalar(self.get_half_angle().scalar().sin());
        match sin.try_inverse() {
            None => Bivector::zero(),
            Some(sin_inverse) => self.bivector * sin_inverse,
        }
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
        let rotor = match Rotor::try_new_from_half_angle_plane(angle / 2.0, rotation_plane) {
            Some(rotor) => rotor,
            None => Rotor::identity(),
        };

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
        let rotor = match Rotor::try_new_from_half_angle_plane(rotation_angle / 2.0, rotation_plane)
        {
            Some(rotor) => rotor,
            None => Rotor::identity(),
        };

        assert_relative_eq!(
            rotor.get_half_angle().scalar(),
            rotation_angle / 2.0,
            max_relative = 0.000001
        );
    }

    #[test]
    fn get_plane_of_rotation() {
        let angle = TAU / 4.0;
        let rotation_plane = Bivector::new(4.0, 2.0, -3.0);
        let norm = rotation_plane.norm();
        let rotor = match Rotor::try_new_from_half_angle_plane(angle / 2.0, rotation_plane) {
            Some(rotor) => rotor,
            None => Rotor::identity(),
        };
        let rotor_rotation_plane = rotor.try_get_rotation_plane();

        assert_relative_eq!(
            rotor_rotation_plane.e12() * norm.scalar(),
            rotation_plane.e12(),
            max_relative = 0.000001
        );
        assert_relative_eq!(
            rotor_rotation_plane.e31() * norm.scalar(),
            rotation_plane.e31(),
            max_relative = 0.000001
        );
        assert_relative_eq!(
            rotor_rotation_plane.e23() * norm.scalar(),
            rotation_plane.e23(),
            max_relative = 0.000001
        );
    }
}

/// # Geometric Product
/// The geometric product of two rotors is another rotor
/// $$ R_1 R_2 = R_3$$
impl<F: Float> Mul for Rotor<F> {
    type Output = Rotor<F>;

    fn mul(self: Rotor<F>, b: Rotor<F>) -> Rotor<F> {
        let a = self.bivector * b.bivector;
        let scalar = self.scalar * b.scalar + a.scalar();
        let bivector = self.scalar * b.bivector + self.bivector * b.scalar + a.bivector();

        // Normelize
        let norm = (scalar * scalar + (bivector * bivector.reverse()).scalar()).sqrt();
        Rotor {
            scalar: Scalar(scalar * norm),
            bivector: bivector * Scalar(norm),
        }
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Rotor<F>, Rotor<F>);

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
        let rotor1 = match Rotor::try_new_from_half_angle_plane(angle1 / 2.0, rotation_plane1) {
            Some(rotor) => rotor,
            None => Rotor::identity(),
        };
        let angle2 = TAU / 4.0;
        let rotation_plane2 = Bivector::new(0.0, 1.0, 0.0);
        let rotor2 = match Rotor::try_new_from_half_angle_plane(angle2 / 2.0, rotation_plane2) {
            Some(rotor) => rotor,
            None => Rotor::identity(),
        };

        let res_rotor = rotor1 * rotor2;
        assert_relative_eq!(res_rotor.scalar(), 0.5, max_relative = 0.000001);
        assert_relative_eq!(res_rotor.e12(), 0.5, max_relative = 0.000001);
        assert_relative_eq!(res_rotor.e31(), 0.5, max_relative = 0.000001);
        assert_relative_eq!(res_rotor.e23(), 0.5, max_relative = 0.000001);
    }
}

impl<F: Float> VGA3DOps<F> for Rotor<F> {
    // \[ |R|^2=\left< R^\dag A \right>_0 \]
    fn norm(self) -> Scalar<F> {
        Scalar(
            ((self.scalar() * self.scalar())
                + (self.bivector() * self.bivector().reverse()).scalar())
            .sqrt(),
        )
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn try_inverse(self) -> Option<Rotor<F>> {
        let norm_squared = (self * self.reverse()).scalar();
        match Scalar(norm_squared).try_inverse() {
            None => None,
            Some(scalar_inverse) => Some(Rotor {
                scalar: Scalar(self.reverse().scalar.0 / norm_squared),
                bivector: self.reverse().bivector * scalar_inverse,
            }),
        }
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(self) -> Self {
        Rotor {
            scalar: self.scalar,
            bivector: -self.bivector,
        }
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(self) -> Self {
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

impl<F: Float> VGA3DOpsRef<F> for Rotor<F> {
    fn norm(&self) -> Scalar<F> {
        Scalar(
            ((self.scalar() * self.scalar())
                + (self.bivector() * self.bivector().reverse()).scalar())
            .sqrt(),
        )
    }

    // Inverse
    // \[A^{-1}=\frac{A^\dag}{\left< A A^\dag \right>}\]
    fn try_inverse(&self) -> Option<Rotor<F>> {
        let norm_squared = (self * self.reverse()).scalar();
        match Scalar(norm_squared).try_inverse() {
            None => None,
            Some(scalar_inverse) => Some(Rotor {
                scalar: Scalar(self.reverse().scalar.0 / norm_squared),
                bivector: self.reverse().bivector * scalar_inverse,
            }),
        }
    }

    // Reverse
    // It follows the patten (Each is a grade)
    // \[+ + - - + + - - \dots (-1)^{k(k-1)/2}\]
    fn reverse(&self) -> Self {
        Rotor {
            scalar: self.scalar,
            bivector: -self.bivector,
        }
    }
    // Clifford Conjugation
    // It follows the patten (Each is a grade)
    // \[+--+--+\dots(-1)^{k(k+1)/2}\]

    fn conjugate(&self) -> Self {
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
    fn rotor_norm() {
        let angle = TAU / 4.0;
        let rotation_plane = Bivector::new(4.0, 2.0, -3.0);
        let rotor = match Rotor::try_new_from_half_angle_plane(angle / 2.0, rotation_plane) {
            Some(rotor) => rotor,
            None => Rotor::identity(),
        };

        // 0.70710677+0.52522576e12+0.26261288e31-0.3939193e23
        assert_relative_eq!(rotor.norm().scalar(), 1.0, max_relative = 0.000001);
        assert_relative_eq!((&rotor).norm().scalar(), 1.0, max_relative = 0.000001);
    }

    fn rotor_rotor_reverse() {
        let angle1 = TAU / 4.0;
        let rotation_plane1 = Bivector::new(3.0, 2.0, 10.0);
        let rotor1 = match Rotor::try_new_from_half_angle_plane(angle1, rotation_plane1) {
            Some(rotor) => rotor,
            None => Rotor::identity(),
        };

        let angle2 = TAU / 2.0;
        let rotation_plane2 = Bivector::new(2.0, -3.0, -1.0);
        let rotor2 = match Rotor::try_new_from_half_angle_plane(angle2, rotation_plane2) {
            Some(rotor) => rotor,
            None => Rotor::identity(),
        };

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
