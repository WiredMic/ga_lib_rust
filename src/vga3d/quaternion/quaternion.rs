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

use crate::forward_ref_binop;
#[cfg(feature = "defmt")]
use defmt::Format;

use num_traits::{Float, Zero};

use super::UnitQuaternion;
use crate::vga3d::{
    bivector::Bivector, multivector::Multivector, scalar::Scalar, trivector::Trivector,
    vector::Vector, Rotor, VGA3DOps, VGA3DOpsRef,
};

use core::ops::{Add, Div, Mul, Sub};

use libm::{acosf, cosf, sinf, sqrtf};

/// # Quaternions
///
/// Quaternions are not part of Geometric Algebra
/// They are subsumed.
///
/// The vector part in a quarernion is a negation of the defintion of a GA bivector.
///
/// $$q = a + b \mathrm{i} + c \mathrm{j} + d \mathrm{k}$$
/// $$i^2 = j ^2 = k^2 = ijk = -1$$
/// $$ij = -ji = k$$
/// $$jk = -kj = i$$
/// $$ki = -ik = j$$
///
/// "Biquaternion" in GA with bivectors are defined going in the oppisite direction
/// $$ \mathrm{i} \to \mathrm{e}_3\mathrm{e}_2 = -\mathrm{e}_2\mathrm{e}_3$$
/// $$ \mathrm{j} \to \mathrm{e}_1\mathrm{e}_3 = -\mathrm{e}_3\mathrm{e}_1$$
/// $$ \mathrm{k} \to \mathrm{e}_2\mathrm{e}_1 = -\mathrm{e}_1\mathrm{e}_2$$
///
/// $$M = a + b \mathrm{e}_1\mathrm{e}_2 + c \mathrm{e}_3\mathrm{e}_1 + d \mathrm{e}_2\mathrm{e}_3$$
/// $$(\mathrm{e}_1\mathrm{e}_2)^2 = (\mathrm{e}_3\mathrm{e}_1) ^2 = (\mathrm{e}_2\mathrm{e}_3)^2 = (\mathrm{e}_1\mathrm{e}_2)(\mathrm{e}_3\mathrm{e}_1)(\mathrm{e}_2\mathrm{e}_3) = -1$$
/// $$(\mathrm{e}_2\mathrm{e}_3)(\mathrm{e}_3\mathrm{e}_1) = -(\mathrm{e}_3\mathrm{e}_1)(\mathrm{e}_2\mathrm{e}_3) = -(\mathrm{e}_1\mathrm{e}_2)$$
/// $$(\mathrm{e}_3\mathrm{e}_1)(\mathrm{e}_1\mathrm{e}_2) = -(\mathrm{e}_1\mathrm{e}_2)(\mathrm{e}_3\mathrm{e}_1) = -(\mathrm{e}_2\mathrm{e}_3 )$$
/// $$(\mathrm{e}_1\mathrm{e}_2)(\mathrm{e}_2\mathrm{e}_3) = -(\mathrm{e}_2\mathrm{e}_3)(\mathrm{e}_1\mathrm{e}_2) = -(\mathrm{e}_3\mathrm{e}_1)$$
///
/// This is and implementation of the normal Quaterions
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Quaternion<F: Float> {
    scalar: Scalar<F>,
    vector: Vector<F>,
}

#[cfg(feature = "std")]
impl<F: Float + fmt::Display> fmt::Display for Quaternion<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}e12, {}e31, {}e23", self.e12, self.e31, self.e23)

        write!(f, "Quaternion {{\n")?;
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
impl<F: Float + defmt::Format> defmt::Format for Quaternion<F> {
    fn format(&self, fmt: defmt::Formatter) {
        // the defmt version - similar structure but using defmt macros
        defmt::write!(fmt, "Quaternion {{\n");
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

impl<F: Float> Quaternion<F> {
    /// New Quaternion with scalar and imaginary vector parts
    pub fn new(scalar: F, vector: Vector<F>) -> Self {
        Self {
            scalar: Scalar(scalar),
            vector,
        }
    }

    /// The zero quaternion
    pub fn zero() -> Self {
        Self {
            scalar: Scalar(F::zero()),
            vector: Vector::zero(),
        }
    }

    /// "Biquaternion" in GA with bivectors are defined going in the oppisite direction
    /// $$ \mathrm{i} \to \mathrm{e}_3\mathrm{e}_2 = -\mathrm{e}_2\mathrm{e}_3$$
    /// $$ \mathrm{j} \to \mathrm{e}_1\mathrm{e}_3 = -\mathrm{e}_3\mathrm{e}_1$$
    /// $$ \mathrm{k} \to \mathrm{e}_2\mathrm{e}_1 = -\mathrm{e}_1\mathrm{e}_2$$
    pub fn new_from_scalar_bivector(scalar: F, bivector: Bivector<F>) -> Self {
        Self {
            scalar: Scalar(scalar),
            vector: Vector::new(-bivector.e23(), -bivector.e31(), -bivector.e12()),
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
    ///
    /// This function normizises the Quaterion.
    /// It treats the scalar part as $\cos(\theta/2)$.
    /// And the vector part as $\sin(\theta/2)\hat{n}$.
    /// Where $\hat{n}$ is the axes of rotation
    pub fn to_rotor(&self) -> Rotor<F> {
        let norm = (self.scalar * self.scalar
            + self.vector.e1() * self.vector.e1()
            + self.vector.e2() * self.vector.e2()
            + self.vector.e3() * self.vector.e3())
        .sqrt();

        Rotor {
            scalar: Scalar(self.scalar.0 / norm),
            bivector: Bivector::new(
                self.vector.e3() / norm,
                self.vector.e2() / norm,
                self.vector.e1() / norm,
            ),
        }
    }

    /// This function normizises the Quaterion.
    /// It treats the scalar part as $\cos(\theta/2)$.
    /// And the vector part as $\sin(\theta/2)\hat{n}$.
    /// Where $\hat{n}$ is the axes of rotation
    pub fn to_unit_quaternion(self) -> UnitQuaternion<F> {
        let norm = (self.scalar * self.scalar
            + self.vector.e1() * self.vector.e1()
            + self.vector.e2() * self.vector.e2()
            + self.vector.e3() * self.vector.e3())
        .sqrt();

        UnitQuaternion {
            scalar: Scalar(self.scalar.0 / norm),
            vector: Vector::new(
                self.vector.e1() / norm,
                self.vector.e2() / norm,
                self.vector.e3() / norm,
            ),
        }
    }

    /// The complex conjugate of a quaternion is another quaterion with the vector part negated.
    pub fn conjugate(self) -> Quaternion<F> {
        Quaternion::new(
            self.scalar.0,
            Vector::new(-self.vector.e1(), -self.vector.e2(), -self.vector.e3()),
        )
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

        // sqrtf((self * self.conjugate()).scalar)
    }

    /// The invese of a quaternion is a quaternion
    ///
    /// $$ q^{-1} = \frac{q*}{qq*} = \frac{q*}{|q|^2} $$
    ///
    /// The norm of the quaternion can be 0.
    ///
    /// This will give an error.
    pub fn try_inverse(self) -> Option<Quaternion<F>> {
        let norm_squared = (self * self.conjugate()).scalar();

        match norm_squared {
            norm_squared if norm_squared.is_zero() => None,
            _ => return Some(self.conjugate() * (F::one() / norm_squared)),
        }
    }
}

impl<F: Float> Mul for Quaternion<F> {
    type Output = Quaternion<F>;
    /// Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    fn mul(self, b: Quaternion<F>) -> Quaternion<F> {
        Quaternion::new(
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
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Quaternion<F>, Quaternion<F>);

impl<F: Float> Mul<F> for Quaternion<F> {
    type Output = Quaternion<F>;
    /// Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    fn mul(self, b: F) -> Quaternion<F> {
        Quaternion::new(self.scalar() * b, self.vector() * Scalar(b))
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Quaternion<F>, F);

impl<F: Float> Mul<Quaternion<F>> for Scalar<F> {
    type Output = Quaternion<F>;
    /// Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    fn mul(self, b: Quaternion<F>) -> Quaternion<F> {
        Quaternion::new(self * Scalar(b.scalar()), self * b.vector())
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Scalar<F>, Quaternion<F>);

impl<F: Float> Div<Scalar<F>> for Quaternion<F> {
    // The division of rational numbers is a closed operation.
    type Output = Quaternion<F>;
    fn div(self, b: Scalar<F>) -> Quaternion<F> {
        if b.0 == F::zero() {
            panic!("Cannot divide by zero-valued `Rational`!");
        }

        Quaternion::new(
            self.scalar.0,
            Vector::new(
                self.vector().e1() / b.0,
                self.vector().e2() / b.0,
                self.vector().e3() / b.0,
            ),
        )
    }
}
forward_ref_binop!(impl<F:Float> Div, div for Quaternion<F>, Scalar<F>);

impl<F: Float> Add for Quaternion<F> {
    type Output = Quaternion<F>;
    /// Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    fn add(self, b: Quaternion<F>) -> Quaternion<F> {
        Quaternion::new(
            self.scalar + b.scalar,
            Vector::new(
                self.vector.e1() + b.vector.e1(),
                self.vector.e2() + b.vector.e2(),
                self.vector.e3() + b.vector.e3(),
            ),
        )
    }
}
forward_ref_binop!(impl<F:Float> Add, add for Quaternion<F>, Quaternion<F>);

impl<F: Float> Sub for Quaternion<F> {
    type Output = Quaternion<F>;
    /// Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    fn sub(self, b: Quaternion<F>) -> Quaternion<F> {
        Quaternion::new(
            self.scalar.0 - b.scalar.0,
            Vector::new(
                self.vector.e1() - b.vector.e1(),
                self.vector.e2() - b.vector.e2(),
                self.vector.e3() - b.vector.e3(),
            ),
        )
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Quaternion<F>, Quaternion<F>);

#[cfg(test)]
mod quaternion {
    use core::f32::consts::TAU;

    use crate::vga3d::Rotatable;

    use super::*;
    use approx::assert_relative_eq;
    use nalgebra::{Quaternion, Unit, UnitQuaternion, Vector3};

    #[test]
    fn quaternion_norm() {
        let q1 = super::Quaternion::new(2.0, Vector::new(4.0, -3.0, 7.0));
        let q2: Quaternion<f32> = nalgebra::Quaternion::new(2.0, 4.0, -3.0, 7.0);

        let norm1 = (q1.conjugate() * q1).scalar().sqrt();
        let norm2 = (q1 * q1.conjugate()).scalar().sqrt();

        assert_relative_eq!(q1.norm().scalar(), q2.norm(), max_relative = 0.000001);

        assert_relative_eq!(q1.norm().scalar(), norm1, max_relative = 0.000001);
        assert_relative_eq!(q1.norm().scalar(), norm2, max_relative = 0.000001);
    }

    #[test]
    fn quaternion_to_rotor() {
        let angle = 2.3;
        let rotation_axis = Vector::new(4.0, -3.0, 7.0);

        let q = super::Quaternion::new(
            (angle / 2.0).cos(),
            rotation_axis * Scalar((angle / 2.0).sin() / rotation_axis.norm().scalar()),
        );
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
    fn quaternion_quaternion_mul() {
        let p_vga = super::Quaternion::new(2.0, Vector::new(4.0, -3.0, 7.0));
        let q_vga = super::Quaternion::new(6.0, Vector::new(-4.0, -2.0, -3.0));
        let res_vga_1 = p_vga * q_vga;
        let res_vga_2 = q_vga * p_vga;

        let p_nalgebra: Quaternion<f32> = nalgebra::Quaternion::new(2.0, 4.0, -3.0, 7.0);
        let q_nalgebra = nalgebra::Quaternion::new(6.0, -4.0, -2.0, -3.0);
        let res_nalgebra_1 = p_nalgebra * q_nalgebra;
        let res_nalgebra_2 = q_nalgebra * p_nalgebra;

        assert_relative_eq!(
            res_vga_1.scalar(),
            res_nalgebra_1.w,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            res_vga_1.vector.e1(),
            res_nalgebra_1.i,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            res_vga_1.vector.e2(),
            res_nalgebra_1.j,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            res_vga_1.vector.e3(),
            res_nalgebra_1.k,
            max_relative = 0.000001
        );

        assert_relative_eq!(
            res_vga_2.scalar(),
            res_nalgebra_2.w,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            res_vga_2.vector.e1(),
            res_nalgebra_2.i,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            res_vga_2.vector.e2(),
            res_nalgebra_2.j,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            res_vga_2.vector.e3(),
            res_nalgebra_2.k,
            max_relative = 0.000001
        );
    }

    #[test]
    fn quaternion_conjugate() {
        let q_vga = super::Quaternion::new(2.0, Vector::new(4.0, -3.0, 7.0));
        let res_vga = q_vga.conjugate();

        let q_nalgebra: Quaternion<f32> = nalgebra::Quaternion::new(2.0, 4.0, -3.0, 7.0);
        let res_nalgebra = q_nalgebra.conjugate();

        assert_relative_eq!(res_vga.scalar(), res_nalgebra.w, max_relative = 0.000001);
        assert_relative_eq!(res_vga.vector.e1(), res_nalgebra.i, max_relative = 0.000001);
        assert_relative_eq!(res_vga.vector.e2(), res_nalgebra.j, max_relative = 0.000001);
        assert_relative_eq!(res_vga.vector.e3(), res_nalgebra.k, max_relative = 0.000001);
    }

    #[test]
    fn quaternion_to_unit_quaternion() {
        let q_vga = super::Quaternion::new(2.0, Vector::new(4.0, -3.0, 7.0));
        let res_vga = q_vga.to_unit_quaternion();

        let q_nalgebra: Quaternion<f32> = nalgebra::Quaternion::new(2.0, 4.0, -3.0, 7.0);
        let res_nalgebra = UnitQuaternion::from_quaternion(q_nalgebra);

        assert_relative_eq!(res_vga.scalar(), res_nalgebra.w, max_relative = 0.000001);
        assert_relative_eq!(res_vga.vector.e1(), res_nalgebra.i, max_relative = 0.000001);
        assert_relative_eq!(res_vga.vector.e2(), res_nalgebra.j, max_relative = 0.000001);
        assert_relative_eq!(res_vga.vector.e3(), res_nalgebra.k, max_relative = 0.000001);
    }

    #[test]
    fn quaternion_rotation_mul() {
        let angle = 3.0 / 4.0 * TAU;
        let rotation_axis = super::Vector::new(3.0, 4.0, 5.0);
        let scalar = (angle / 2.0).cos();
        let vector = rotation_axis * Scalar((angle / 2.0).sin() / rotation_axis.norm().scalar());

        let unit1 = super::Quaternion::new(scalar, vector);
        let unit2 = super::UnitQuaternion::new(angle / 2.0, rotation_axis);

        assert_relative_eq!(unit1.scalar(), unit2.scalar(), max_relative = 0.000001);
        assert_relative_eq!(unit1.e1(), unit2.e1(), max_relative = 0.000001);
        assert_relative_eq!(unit1.e2(), unit2.e2(), max_relative = 0.000001);
        assert_relative_eq!(unit1.e3(), unit2.e3(), max_relative = 0.000001);

        let axis = Unit::new_normalize(Vector3::new(3.0, 4.0, 5.0));
        // takes the full rotation angle
        let unit3 = nalgebra::UnitQuaternion::from_axis_angle(&axis, angle);

        assert_relative_eq!(unit2.scalar(), unit3.w, max_relative = 0.000001);
        assert_relative_eq!(unit2.e1(), unit3.i, max_relative = 0.000001);
        assert_relative_eq!(unit2.e2(), unit3.j, max_relative = 0.000001);
        assert_relative_eq!(unit2.e3(), unit3.k, max_relative = 0.000001);

        let vector_vga = Vector::new(3.0, 1.0, 6.0);
        let q_vector_vga = super::Quaternion::new(0.0, vector_vga);
        let vector_nalgebra = nalgebra::Vector3::new(3.0, 1.0, 6.0);

        let res1 = (unit2 * q_vector_vga * unit2.conjugate()).vector();
        let res2 = unit3.transform_vector(&vector_nalgebra);

        assert_relative_eq!(res1.e1(), res2.x, max_relative = 0.000001);
        assert_relative_eq!(res1.e2(), res2.y, max_relative = 0.000001);
        assert_relative_eq!(res1.e3(), res2.z, max_relative = 0.000001);
    }

    // #[test]
    // fn quaternion_vector_rot() {
    //     let angle = TAU * 4.0 / 7.0;
    //     let rotation_plane = Bivector::new(0.5, 5.2, -3.0);
    //     let rotation_axis = -rotation_plane.dual();

    //     // let rotation_axis = Vector::new(-3.0, 5.2, 0.5);
    //     // let rotation_plane = rotation_axis.dual();

    //     let input_vector = Vector::new(6.4, -4.5, 3.3);
    //     let q_vector = Quaternion::new(0.0, input_vector);

    //     let scalar = cosf(angle / 2.0);
    //     let sin = sinf(angle / 2.0);
    //     let norm = rotation_axis.norm();
    //     let bi_pre = sin / norm;
    //     let vector = bi_pre * rotation_axis;
    //     let unit = Quaternion { scalar, vector };

    //     let res_unit = (unit * q_vector * unit.conjugate()).vector();

    //     let rotor = Rotor::new(angle / 2.0, rotation_plane);
    //     // let rotor = unit.to_rotor();
    //     let res_rotor = input_vector.rotate(rotor);

    //     // -0.72899,-8.04355,-2.62105
    //     assert_relative_eq!(res_rotor.e1(), -0.72897816, max_relative = 0.000001);
    //     assert_relative_eq!(res_rotor.e2(), -8.043535, max_relative = 0.000001);
    //     assert_relative_eq!(res_rotor.e3(), -2.6210902, max_relative = 0.000001);

    //     assert_relative_eq!(rotor.scalar(), unit.scalar(), max_relative = 0.000001);
    //     assert_relative_eq!(rotor.e12(), unit.e3(), max_relative = 0.000001);
    //     assert_relative_eq!(rotor.e31(), unit.e2(), max_relative = 0.000001);
    //     assert_relative_eq!(rotor.e23(), unit.e1(), max_relative = 0.000001);

    //     assert_relative_eq!(res_unit.e1(), -0.72897816, max_relative = 0.000001);
    //     assert_relative_eq!(res_unit.e2(), -8.043535, max_relative = 0.000001);
    //     assert_relative_eq!(res_unit.e3(), -2.6210902, max_relative = 0.000001);
    // }

    // #[test]
    // fn quaternion_quaternion_add() {
    //     let p = Quaternion::new(2.0, Vector::new(4.0, -3.0, 7.0));
    //     let q = Quaternion::new(6.0, Vector::new(-4.0, -2.0, -3.0));
    //     let res = p + q;

    //     assert_relative_eq!(res.scalar(), 8.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.vector.e1(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.vector.e2(), -5.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.vector.e3(), 4.0, max_relative = 0.000001);
    // }

    // #[test]
    // fn quaternion_quaternion_sub() {
    //     let p = Quaternion::new(2.0, Vector::new(4.0, -3.0, 7.0));
    //     let q = Quaternion::new(6.0, Vector::new(-4.0, -2.0, -3.0));
    //     let res = p - q;

    //     assert_relative_eq!(res.scalar(), -4.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.vector.e1(), 8.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.vector.e2(), -1.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.vector.e3(), 10.0, max_relative = 0.000001);
    // }
}
