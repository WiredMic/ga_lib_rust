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

use crate::forward_ref_binop;
use crate::vga3d::Scalar;
use crate::vga3d::{
    bivector::Bivector, multivector::Multivector, trivector::Trivector, vector::Vector, Rotor,
    VGA3DOps, VGA3DOpsRef,
};

use core::ops::Mul;

use num_traits::Float;

// use libm::{acosf, cosf, sinf, sqrtf};

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "defmt")]
use defmt::Format;

use super::Quaternion;

/// Unit Quaternion
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct UnitQuaternion<F: Float> {
    pub(super) scalar: Scalar<F>,
    pub(super) vector: Vector<F>,
}

#[cfg(feature = "std")]
impl<F: Float + fmt::Display> fmt::Display for UnitQuaternion<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}e12, {}e31, {}e23", self.e12, self.e31, self.e23)

        write!(f, "Unit Quaternion {{\n")?;
        write!(f, "\treal: {}\n", self.scalar.0)?;
        write!(f, "\timaginary: {}i", self.e1())?;

        // For j component, add appropriate sign
        if self.e2() >= F::zero() {
            write!(f, " + {}j", self.e2())?;
        } else {
            write!(f, " - {}j", self.e2().abs())?;
        }

        // For k component, add appropriate sign
        if self.e3() >= F::zero() {
            write!(f, " + {}k", self.e3())?;
        } else {
            write!(f, " - {}k\n", self.e3().abs())?;
        }
        write!(f, "}}")?;

        Ok(())
    }
}

#[cfg(feature = "defmt")]
impl<F: Float + defmt::Format> defmt::Format for UnitQuaternion<F> {
    fn format(&self, fmt: defmt::Formatter) {
        // the defmt version - similar structure but using defmt macros
        defmt::write!(fmt, "Unit Quaternion {{\n");
        defmt::write!(fmt, "\treal: {}\n", self.scalar.0);
        defmt::write!(fmt, "\timaginary: {}i", self.e1());

        // for j component, add appropriate sign
        if self.e2() >= F::zero() {
            defmt::write!(fmt, " + {}j", self.e2());
        } else {
            defmt::write!(fmt, " - {}j", self.e2().abs());
        }

        // for k component, add appropriate sign
        if self.e3() >= F::zero() {
            defmt::write!(fmt, " + {}k", self.e3());
        } else {
            defmt::write!(fmt, " - {}k", self.e3().abs());
        }
        defmt::write!(fmt, "\n}}");
    }
}

impl<F: Float> UnitQuaternion<F> {
    /// New Unit Quaternion from the halv of  the Rotation Angle and Axis of Rotation
    pub fn new(half_angle: F, rotation_axis: Vector<F>) -> Self {
        let scalar = half_angle.cos();
        let sin = half_angle.sin();
        let norm = rotation_axis.norm().scalar();
        let bi_pre = sin / norm;

        let vector = Scalar(bi_pre) * rotation_axis;
        Self {
            scalar: Scalar(scalar),
            vector,
        }
    }

    /// Return scalar part of Quaternion
    pub fn scalar(&self) -> F {
        self.scalar.0
    }

    /// Return vector part of Quaternion
    pub fn vector(&self) -> Vector<F> {
        self.vector
    }

    /// Return $\mathrm{e}_1$ vector part of Quaternion
    pub fn e1(&self) -> F {
        self.vector.e1()
    }
    /// Return $\mathrm{e}_2$ vector part of Quaternion
    pub fn e2(&self) -> F {
        self.vector.e2()
    }

    /// Return $\mathrm{e}_3$ vector part of Quaternion
    pub fn e3(&self) -> F {
        self.vector.e3()
    }

    /// Return bivector representation of vector part of Quaternion
    pub fn bivector(&self) -> Bivector<F> {
        Bivector::new(-self.vector.e3(), -self.vector.e2(), -self.vector.e1())
    }

    /// The way to rotation in Geomtric Algebra is with Rotors
    ///
    /// A unit Quaternion is almost a rotor.
    ///
    /// The imagniary part of the quaternion is a vector and the rotor need a bivector.
    /// The is done with the dual operator.
    ///
    /// $$ \vec{a}\star = \overset\Rightarrow{b}  $$
    pub fn to_rotor(self) -> Rotor<F> {
        Rotor {
            scalar: self.scalar,
            bivector: Bivector::new(self.vector.e3(), self.vector.e2(), self.vector.e1()),
        }
    }

    /// The complex conjugate of a quaternion is another quaterion with the vector part negated.
    pub fn conjugate(self) -> UnitQuaternion<F> {
        UnitQuaternion {
            scalar: self.scalar,
            vector: Vector::new(-self.vector.e1(), -self.vector.e2(), -self.vector.e3()),
        }
    }

    /// The norm of a quaternion is the square root of the product between the quaternion and its conjugate.
    /// $$\sqrt{q q*}$$
    pub fn norm(self) -> Scalar<F> {
        Scalar(
            (self.scalar * self.scalar
                + self.vector.e1() * self.vector.e1()
                + self.vector.e2() * self.vector.e2()
                + self.vector.e3() * self.vector.e3())
            .sqrt(),
        )
        // sqrtf((self * self.conjugate()).scalar())
    }
}

impl<F: Float> Rotor<F> {
    /// The way to rotation in Geomtric Algebra is with Rotors
    ///
    /// A unit Quaternion is almost a rotor.
    ///
    /// The rotor rotates around a plane. The unit quaternion around an axis.
    /// This means changing the plane (bivector) to an axis (vector).
    /// The is done with the dual operator.
    ///
    /// $$ -\overset\Rightarrow{a}\star = \vec{b}  $$
    pub fn to_unit_quaternion(self) -> UnitQuaternion<F> {
        UnitQuaternion {
            scalar: self.scalar,
            vector: Vector::new(self.e23(), self.e31(), self.e12()),
        }
    }
}

impl<F: Float> Mul for UnitQuaternion<F> {
    type Output = UnitQuaternion<F>;
    /// Unit Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    /// The hamiltonian product of two unit quaternions is a unit quaternion
    fn mul(self, b: UnitQuaternion<F>) -> UnitQuaternion<F> {
        let unit = UnitQuaternion::new(
            (self.scalar * b.scalar)
                - (self.vector.e1() * b.vector.e1())
                - (self.vector.e2() * b.vector.e2())
                - (self.vector.e3() * b.vector.e3()),
            Vector::new(
                (self.scalar.0 * b.vector.e1())
                    + (self.vector.e1() * b.scalar.0)
                    + self.vector.e2() * b.vector.e3()
                    - self.vector.e3() * b.vector.e2(),
                (self.scalar.0 * b.vector.e2())
                    + (self.vector.e2() * b.scalar.0)
                    + self.vector.e3() * b.vector.e1()
                    - self.vector.e1() * b.vector.e3(),
                (self.scalar.0 * b.vector.e3())
                    + (self.vector.e3() * b.scalar.0)
                    + self.vector.e1() * b.vector.e2()
                    - self.vector.e2() * b.vector.e1(),
            ),
        );

        unit
        // // normilize to get rid of float errors
        // let norm = unit.norm();
        // UnitQuaternion {
        //     scalar: unit.scalar / norm,
        //     vector: Vector::new(
        //         unit.vector.e1() / norm,
        //         unit.vector.e2() / norm,
        //         unit.vector.e3() / norm,
        //     ),
        // }
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for UnitQuaternion<F>, UnitQuaternion<F>);

impl<F: Float> Mul<Quaternion<F>> for UnitQuaternion<F> {
    type Output = Quaternion<F>;
    /// Unit Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    /// The hamiltonian product of two unit quaternions is a unit quaternion
    fn mul(self, b: Quaternion<F>) -> Quaternion<F> {
        Quaternion::new(
            (self.scalar() * b.scalar())
                - (self.vector().e1() * b.vector().e1())
                - (self.vector().e2() * b.vector().e2())
                - (self.vector().e3() * b.vector().e3()),
            Vector::new(
                (self.scalar() * b.vector().e1())
                    + (self.vector().e1() * b.scalar())
                    + self.vector().e2() * b.vector().e3()
                    - self.vector().e3() * b.vector().e2(),
                (self.scalar() * b.vector().e2())
                    + (self.vector().e2() * b.scalar())
                    + self.vector().e3() * b.vector().e1()
                    - self.vector().e1() * b.vector().e3(),
                (self.scalar() * b.vector().e3())
                    + (self.vector().e3() * b.scalar())
                    + self.vector().e1() * b.vector().e2()
                    - self.vector().e2() * b.vector().e1(),
            ),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for UnitQuaternion<F>, Quaternion<F>);

impl<F: Float> Mul<UnitQuaternion<F>> for Quaternion<F> {
    type Output = Quaternion<F>;
    /// Unit Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    /// The hamiltonian product of two unit quaternions is a unit quaternion
    fn mul(self, b: UnitQuaternion<F>) -> Quaternion<F> {
        Quaternion::new(
            (self.scalar() * b.scalar())
                - (self.vector().e1() * b.vector().e1())
                - (self.vector().e2() * b.vector().e2())
                - (self.vector().e3() * b.vector().e3()),
            Vector::new(
                (self.scalar() * b.vector().e1())
                    + (self.vector().e1() * b.scalar())
                    + self.vector().e2() * b.vector().e3()
                    - self.vector().e3() * b.vector().e2(),
                (self.scalar() * b.vector().e2())
                    + (self.vector().e2() * b.scalar())
                    + self.vector().e3() * b.vector().e1()
                    - self.vector().e1() * b.vector().e3(),
                (self.scalar() * b.vector().e3())
                    + (self.vector().e3() * b.scalar())
                    + self.vector().e1() * b.vector().e2()
                    - self.vector().e2() * b.vector().e1(),
            ),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Quaternion<F>, UnitQuaternion<F>);

pub trait RotatableQuaternion<F: Float, U = UnitQuaternion<F>> {
    type Output;
    fn rotate_with_quaternion(self, unit_quaternion: U) -> Self::Output;
}

impl<F: Float> RotatableQuaternion<F> for Quaternion<F> {
    type Output = Quaternion<F>;
    /// $$ q' = \hat{p} q \hat{p}* $$
    fn rotate_with_quaternion(self, unit_quaternion: UnitQuaternion<F>) -> Quaternion<F> {
        unit_quaternion * self * unit_quaternion.conjugate()
    }
}

impl<F: Float> RotatableQuaternion<F> for Vector<F> {
    type Output = Vector<F>;
    /// $$ q' = \hat{p} q \hat{p}* $$
    fn rotate_with_quaternion(self, unit_quaternion: UnitQuaternion<F>) -> Vector<F> {
        let q_vector = Quaternion::new(F::zero(), self);
        (unit_quaternion * q_vector * unit_quaternion.conjugate()).vector()
    }
}

#[cfg(test)]
mod unit_quaternion {
    use core::f32::consts::TAU;

    use crate::vga3d::{quaternion::Quaternion, Rotatable};

    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn unit_quaternion_unit_quaternion_mul() {
        let p = Quaternion::new(2.0, Vector::new(600.0, -3.0, 619.0));
        let q = Quaternion::new(6.0, Vector::new(-4.0, -2.0, -6488.0));
        let unit1 = p.to_unit_quaternion();
        let unit2 = q.to_unit_quaternion();
        let mut res = unit1 * unit2;
        for _i in 0..1_000_000 {
            res = unit1 * unit2;
        }

        assert_relative_eq!(unit1.norm().scalar(), 1.0, max_relative = 0.000001);
        assert_relative_eq!(unit2.norm().scalar(), 1.0, max_relative = 0.000001);
        assert_relative_eq!(res.norm().scalar(), 1.0, max_relative = 0.1);
    }

    #[test]
    fn unit_quaternion_quaternion_mul() {
        // https://www.omnicalculator.com/math/quaternion
        let p = Quaternion::new(2.0, Vector::new(4.0, -3.0, 7.0));
        let q = Quaternion::new(6.0, Vector::new(-4.0, -2.0, -3.0));
        let unit = q.to_unit_quaternion();
        let res = p * unit;
        // let res = unit.norm();

        assert_relative_eq!(unit.norm().scalar(), 1.0, max_relative = 0.000001);

        assert_relative_eq!(unit.scalar(), 0.7442084, max_relative = 0.000001);
        assert_relative_eq!(unit.e1(), -0.49613893, max_relative = 0.000001);
        assert_relative_eq!(unit.e2(), -0.24806947, max_relative = 0.000001);
        assert_relative_eq!(unit.e3(), -0.3721042, max_relative = 0.000001);

        // 5.33349351 + 4.83735463i - 4.71331985j + 1.98455573k
        assert_relative_eq!(res.scalar(), 5.33349351, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 4.83735463, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), -4.71331985, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 1.98455573, max_relative = 0.000001);
    }

    #[test]
    fn quaternion_unit_quaternion_mul() {
        // https://www.omnicalculator.com/math/quaternion
        let p = Quaternion::new(2.0, Vector::new(4.0, -3.0, 7.0));
        let q = Quaternion::new(6.0, Vector::new(-4.0, -2.0, -3.0));
        let unit = q.to_unit_quaternion();
        let res = unit * p;
        // let res = unit.norm();

        assert_relative_eq!(unit.norm().scalar(), 1.0, max_relative = 0.000001);

        assert_relative_eq!(unit.scalar(), 0.7442084, max_relative = 0.000001);
        assert_relative_eq!(unit.e1(), -0.49613893, max_relative = 0.000001);
        assert_relative_eq!(unit.e2(), -0.24806947, max_relative = 0.000001);
        assert_relative_eq!(unit.e3(), -0.3721042, max_relative = 0.000001);

        // 5.33349351 - 0.86824315i - 0.74420843j + 6.94594507k
        assert_relative_eq!(res.scalar(), 5.33349351, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), -0.86824315, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), -0.74420843, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 6.94594507, max_relative = 0.000001);
    }

    #[test]
    fn unit_quaternion_conjugate() {
        let q = Quaternion::new(2.0, Vector::new(4.0, -3.0, 7.0));
        let unit = q.to_unit_quaternion();
        let unit_conj = unit.conjugate();

        assert_relative_eq!(unit.scalar(), unit_conj.scalar(), max_relative = 0.000001);
        assert_relative_eq!(unit.e1(), -unit_conj.e1(), max_relative = 0.000001);
        assert_relative_eq!(unit.e2(), -unit_conj.e2(), max_relative = 0.000001);
        assert_relative_eq!(unit.e3(), -unit_conj.e3(), max_relative = 0.000001);
    }

    #[test]
    fn unit_quaternion_to_rotor() {
        let angle = 2.3;
        let rotation_axis = Vector::new(4.0, -3.0, 7.0);

        let q = super::UnitQuaternion::new(angle / 2.0, rotation_axis);
        let res = q.to_rotor();

        let rotor = match Rotor::try_new_from_half_angle_plane(angle / 2.0, rotation_axis.dual()) {
            Some(rotor) => rotor,
            None => Rotor::identity(),
        };

        assert_relative_eq!(res.norm().scalar(), 1.0, max_relative = 0.000001);
        assert_relative_eq!(res.scalar(), rotor.scalar(), max_relative = 0.000001);
        assert_relative_eq!(res.e12(), rotor.e12(), max_relative = 0.000001);
        assert_relative_eq!(res.e31(), rotor.e31(), max_relative = 0.000001);
        assert_relative_eq!(res.e23(), rotor.e23(), max_relative = 0.000001);
    }

    #[test]
    fn unit_quaternion_to_rotor_to_unit_quaternion() {
        let angle = 2.3;
        let rotation_axis = Vector::new(4.0, -3.0, 7.0);

        let unit = super::UnitQuaternion::new(angle / 2.0, rotation_axis);
        let rotor = unit.to_rotor();
        let res = rotor.to_unit_quaternion();

        assert_relative_eq!(res.scalar(), unit.scalar(), max_relative = 0.000001);
        assert_relative_eq!(res.e1(), unit.e1(), max_relative = 0.000001);
        assert_relative_eq!(res.e2(), unit.e2(), max_relative = 0.000001);
        assert_relative_eq!(res.e3(), unit.e3(), max_relative = 0.000001);
    }

    #[test]
    fn unit_quaternion_vector_rotation() {
        let angle = 3.0 / 7.0 * TAU;
        let rotation_axis = Vector::new(3.0, 4.0, 5.0);

        let unit = UnitQuaternion::new(angle / 2.0, rotation_axis);

        let vector = Vector::new(3.0, 1.0, 6.0);

        let rotor = unit.to_rotor();
        let res_rotor = vector.rotate(rotor);

        let res_unit = vector.rotate_with_quaternion(unit);

        assert_relative_eq!(res_unit.e1(), res_rotor.e1(), max_relative = 0.000001);
        assert_relative_eq!(res_unit.e2(), res_rotor.e2(), max_relative = 0.000001);
        assert_relative_eq!(res_unit.e3(), res_rotor.e3(), max_relative = 0.000001);
    }
}
