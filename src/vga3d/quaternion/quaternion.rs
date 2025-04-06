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

use crate::forward_ref_binop;

use super::UnitQuaternion;
use crate::vga3d::{
    bivector::Bivector,
    multivector::Multivector,
    trivector::Trivector,
    vector::{self, Vector},
    Rotor, VGA3DOps, VGA3DOpsRef,
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
pub struct Quaternion {
    scalar: f32,
    vector: Vector,
}

impl Quaternion {
    pub fn new(scalar: f32, vector: Vector) -> Self {
        Self { scalar, vector }
    }

    /// "Biquaternion" in GA with bivectors are defined going in the oppisite direction
    /// $$ \mathrm{i} \to \mathrm{e}_3\mathrm{e}_2 = -\mathrm{e}_2\mathrm{e}_3$$
    /// $$ \mathrm{j} \to \mathrm{e}_1\mathrm{e}_3 = -\mathrm{e}_3\mathrm{e}_1$$
    /// $$ \mathrm{k} \to \mathrm{e}_2\mathrm{e}_1 = -\mathrm{e}_1\mathrm{e}_2$$
    pub fn new_from_scalar_bivector(scalar: f32, bivector: Bivector) -> Self {
        Self {
            scalar,
            vector: Vector::new(-bivector.e23(), -bivector.e31(), -bivector.e12()),
        }
    }

    /// Return scalar part of Quaternion
    pub fn scalar(&self) -> f32 {
        self.scalar
    }

    /// Return vector part of Quaternion
    pub fn vector(&self) -> Vector {
        self.vector
    }

    /// Return $\mathrm{e}_1$ vector part of Quaternion
    pub fn e1(&self) -> f32 {
        self.vector.e1()
    }
    /// Return $\mathrm{e}_2$ vector part of Quaternion
    pub fn e2(&self) -> f32 {
        self.vector.e2()
    }

    /// Return $\mathrm{e}_3$ vector part of Quaternion
    pub fn e3(&self) -> f32 {
        self.vector.e3()
    }

    /// Return bivector representation of vector part of Quaternion
    pub fn bivector(&self) -> Bivector {
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
    pub fn to_rotor(&self) -> Rotor {
        let norm = sqrtf(
            self.scalar * self.scalar
                + self.vector.e1() * self.vector.e1()
                + self.vector.e2() * self.vector.e2()
                + self.vector.e3() * self.vector.e3(),
        );

        Rotor {
            scalar: self.scalar / norm,
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
    pub fn to_unit_quaternion(self) -> UnitQuaternion {
        let norm = sqrtf(
            self.scalar * self.scalar
                + self.vector.e1() * self.vector.e1()
                + self.vector.e2() * self.vector.e2()
                + self.vector.e3() * self.vector.e3(),
        );

        UnitQuaternion {
            scalar: self.scalar / norm,
            vector: Vector::new(
                self.vector.e1() / norm,
                self.vector.e2() / norm,
                self.vector.e3() / norm,
            ),
        }
    }

    /// The complex conjugate of a quaternion is another quaterion with the vector part negated.
    pub fn conjugate(self) -> Quaternion {
        Quaternion::new(
            self.scalar,
            Vector::new(-self.vector.e1(), -self.vector.e2(), -self.vector.e3()),
        )
    }

    /// The norm of a quaternion is the square root of the product between the quaternion and its conjugate.
    /// $$\sqrt{q q*}$$
    pub fn norm(self) -> f32 {
        sqrtf(
            self.scalar * self.scalar
                + self.vector.e1() * self.vector.e1()
                + self.vector.e2() * self.vector.e2()
                + self.vector.e3() * self.vector.e3(),
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
    pub fn inverse(self) -> Result<Quaternion, &'static str> {
        let norm = self.norm();

        match norm {
            0.0 => Err("The length of the quaternion is 0"),
            _ => return Ok(self.conjugate() * (1.0 / (self * self.conjugate()).scalar())),
        }
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;
    /// Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    fn mul(self, b: Quaternion) -> Quaternion {
        Quaternion::new(
            (self.scalar * b.scalar)
                - (self.vector.e1() * b.vector.e1())
                - (self.vector.e2() * b.vector.e2())
                - (self.vector.e3() * b.vector.e3()),
            Vector::new(
                (self.scalar * b.vector.e1())
                    + (self.vector.e1() * b.scalar)
                    + self.vector.e2() * b.vector.e3()
                    - self.vector.e3() * b.vector.e2(),
                (self.scalar * b.vector.e2())
                    + (self.vector.e2() * b.scalar)
                    + self.vector.e3() * b.vector.e1()
                    - self.vector.e1() * b.vector.e3(),
                (self.scalar * b.vector.e3())
                    + (self.vector.e3() * b.scalar)
                    + self.vector.e1() * b.vector.e2()
                    - self.vector.e2() * b.vector.e1(),
            ),
        )
    }
}
forward_ref_binop!(impl Mul, mul for Quaternion, Quaternion);

impl Mul<f32> for Quaternion {
    type Output = Quaternion;
    /// Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    fn mul(self, b: f32) -> Quaternion {
        Quaternion::new(self.scalar() * b, self.vector() * b)
    }
}
forward_ref_binop!(impl Mul, mul for Quaternion, f32);

impl Mul<Quaternion> for f32 {
    type Output = Quaternion;
    /// Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    fn mul(self, b: Quaternion) -> Quaternion {
        Quaternion::new(self * b.scalar(), self * b.vector())
    }
}
forward_ref_binop!(impl Mul, mul for f32, Quaternion);

impl Div<f32> for Quaternion {
    // The division of rational numbers is a closed operation.
    type Output = Quaternion;

    fn div(self, b: f32) -> Quaternion {
        if b == 0.0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }

        Quaternion::new(
            self.scalar,
            Vector::new(
                self.vector().e1() / b,
                self.vector().e2() / b,
                self.vector().e3() / b,
            ),
        )
    }
}
forward_ref_binop!(impl Div, div for Quaternion, f32);

impl Add for Quaternion {
    type Output = Quaternion;
    /// Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    fn add(self, b: Quaternion) -> Quaternion {
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
forward_ref_binop!(impl Add, add for Quaternion, Quaternion);

impl Sub for Quaternion {
    type Output = Quaternion;
    /// Quaternion multiplication
    /// $$ pq = p_0 \cdot q_0 - \vec{p}\cdot \vec{q} +p_0\vec{q} + q_0\vec{p} + \vec{p}\cross\vec{q} $$
    fn sub(self, b: Quaternion) -> Quaternion {
        Quaternion::new(
            self.scalar - b.scalar,
            Vector::new(
                self.vector.e1() - b.vector.e1(),
                self.vector.e2() - b.vector.e2(),
                self.vector.e3() - b.vector.e3(),
            ),
        )
    }
}
forward_ref_binop!(impl Sub, sub for Quaternion, Quaternion);

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

        let norm1 = sqrtf((q1.conjugate() * q1).scalar);
        let norm2 = sqrtf((q1 * q1.conjugate()).scalar);

        assert_relative_eq!(q1.norm(), q2.norm(), max_relative = 0.000001);

        assert_relative_eq!(q1.norm(), norm1, max_relative = 0.000001);
        assert_relative_eq!(q1.norm(), norm2, max_relative = 0.000001);
    }

    #[test]
    fn quaternion_to_rotor() {
        let angle = 2.3;
        let rotation_axis = Vector::new(4.0, -3.0, 7.0);

        let q = super::Quaternion::new(
            cosf(angle / 2.0),
            sinf(angle / 2.0) * rotation_axis * (1.0 / rotation_axis.norm()),
        );
        let res = q.to_rotor();

        let rotor = Rotor::new(angle / 2.0, rotation_axis.dual());

        assert_relative_eq!(res.norm(), 1.0, max_relative = 0.000001);
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
        let scalar = cosf(angle / 2.0);
        let vector = rotation_axis * (sinf(angle / 2.0) / rotation_axis.norm());

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
