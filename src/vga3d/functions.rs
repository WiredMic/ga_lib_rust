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

#![allow(unused_imports)]
#![allow(dead_code)]

use super::{
    bivector::Bivector, multivector::Multivector, rotor::Rotor, scalar::Scalar,
    trivector::Trivector, vector::Vector, VGA3DOps, VGA3DOpsRef,
};

use num_traits::Float;

// Functions
// Rotation
pub trait Rotatable<R> {
    type Output;
    fn rotate(self, rotor: R) -> Self::Output;
}

macro_rules! impl_rotatable {
    ($vec:ty, $output:ty, $extract:ident) => {
        // Owned vector, owned rotor
        impl<F: Float> Rotatable<Rotor<F>> for $vec {
            type Output = $output;
            fn rotate(self, rotor: Rotor<F>) -> Self::Output {
                (rotor.reverse() * self * rotor).$extract()
            }
        }

        // Owned vector, reference rotor
        impl<'r, F: Float> Rotatable<&'r Rotor<F>> for $vec {
            type Output = $output;
            fn rotate(self, rotor: &'r Rotor<F>) -> Self::Output {
                (rotor.reverse() * self * *rotor).$extract()
            }
        }

        // Reference vector, owned rotor
        impl<'v, F: Float> Rotatable<Rotor<F>> for &'v $vec {
            type Output = $output;
            fn rotate(self, rotor: Rotor<F>) -> Self::Output {
                (rotor.reverse() * *self * rotor).$extract()
            }
        }

        // Reference vector, reference rotor
        impl<'v, 'r, F: Float> Rotatable<&'r Rotor<F>> for &'v $vec {
            type Output = $output;
            fn rotate(self, rotor: &'r Rotor<F>) -> Self::Output {
                (rotor.reverse() * *self * *rotor).$extract()
            }
        }
    };
}

// Usage:
impl_rotatable!(Vector<F>, Vector<F>, vector);
impl_rotatable!(Bivector<F>, Bivector<F>, bivector);
impl_rotatable!(Trivector<F>, Trivector<F>, trivector);
impl_rotatable!(Multivector<F>, Multivector<F>, multivector);

#[cfg(test)]
mod rotation {
    use crate::vga3d::rotor;

    use super::*;
    use approx::assert_relative_eq;
    use core::f32::consts::TAU;

    #[test]
    fn vec_rot_quarter_e1e2() {
        let vector = Vector::new(3.0, 0.0, 0.0);
        let angle = TAU / 4.0;
        let bivector = Bivector::new(1.0, 0.0, 0.0);
        let rotor = Rotor::try_new_from_half_angle_plane(angle / 2.0, bivector);
        let vector_rot_ref1 = match rotor {
            Some(rotor) => (&vector).rotate(&rotor),
            None => vector,
        };
        let vector_rot_ref2 = match rotor {
            Some(rotor) => (&vector).rotate(rotor),
            None => vector,
        };
        let vector_rot_ref3 = match rotor {
            Some(rotor) => (vector).rotate(&rotor),
            None => vector,
        };
        let vector_rot = match rotor {
            Some(rotor) => (vector).rotate(rotor),
            None => vector,
        };

        assert_relative_eq!(vector_rot_ref1.e1(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(vector_rot_ref1.e2(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(vector_rot_ref1.e3(), 0.0, max_relative = 0.000001);

        assert_relative_eq!(vector_rot_ref2.e1(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(vector_rot_ref2.e2(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(vector_rot_ref2.e3(), 0.0, max_relative = 0.000001);

        assert_relative_eq!(vector_rot_ref3.e1(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(vector_rot_ref3.e2(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(vector_rot_ref3.e3(), 0.0, max_relative = 0.000001);

        assert_relative_eq!(vector_rot.e1(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(vector_rot.e2(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(vector_rot.e3(), 0.0, max_relative = 0.000001);
    }

    #[test]
    fn vector() {
        let angle = TAU / 4.0;
        let rotation_plane = Bivector::new(0.5, 5.2, -3.0);
        let rotor = Rotor::try_new_from_half_angle_plane(angle / 2.0, rotation_plane);
        let vector = Vector::new(6.4, -4.5, 3.3);
        let res = match rotor {
            Some(rotor) => vector.rotate(rotor),
            None => vector,
        };

        assert_relative_eq!(res.e1(), 6.6072783, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), -3.6931403, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), -3.847674, max_relative = 0.000001);
    }

    #[test]
    fn vector_2() {
        let angle = TAU * 4.0 / 7.0;
        let rotation_plane = Bivector::new(0.5, 5.2, -3.0);
        let rotor = Rotor::try_new_from_half_angle_plane(angle / 2.0, rotation_plane);
        let vector = Vector::new(6.4, -4.5, 3.3);
        let res = match rotor {
            Some(rotor) => vector.rotate(rotor),
            None => vector,
        };

        // -0.72899,-8.04355,-2.62105
        assert_relative_eq!(res.e1(), -0.72897816, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), -8.043535, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), -2.6210902, max_relative = 0.000001);
    }

    #[test]
    fn bivector() {
        let angle = TAU / 4.0;
        let rotation_plane = Bivector::new(0.5, 5.2, -3.0);
        let rotor = Rotor::try_new_from_half_angle_plane(angle / 2.0, rotation_plane);
        let bivector = Bivector::new(6.4, -4.5, 3.3);
        let bivector_rot = match rotor {
            Some(rotor) => bivector.rotate(rotor),
            None => bivector,
        };

        assert_relative_eq!(bivector_rot.e12(), -1.0222723, max_relative = 0.000001);
        assert_relative_eq!(bivector_rot.e31(), -0.8519465, max_relative = 0.000001);
        assert_relative_eq!(bivector_rot.e23(), 8.386248, max_relative = 0.000001);
    }

    // #[test]
    // fn vector() {
    //     let angle = TAU / 4;
    //     let rotation_plane = Bivector::new(1.0, 0.0, 0.0);
    //     let vector = Vector::new(e1, e2, e3)
    //     assert_eq!(trivec.e123, 2.0);
    // }
}

// Projection
// \[A_\parallel = (A\cdot B)B^{-1}\]
pub trait HasVector<F: Float> {
    fn vector(&self) -> Vector<F>;
}

impl<F: Float> HasVector<F> for Vector<F> {
    fn vector(&self) -> Vector<F> {
        *self
    }
}

impl<F: Float> HasVector<F> for Multivector<F> {
    fn vector(&self) -> Vector<F> {
        self.vector()
    }
}

pub trait HasBivector<F: Float> {
    fn bivector(&self) -> Bivector<F>;
}

impl<F: Float> HasBivector<F> for Bivector<F> {
    fn bivector(&self) -> Bivector<F> {
        *self
    }
}

impl<F: Float> HasBivector<F> for Multivector<F> {
    fn bivector(&self) -> Bivector<F> {
        self.bivector() // Or however you currently implement vector()
    }
}

pub trait HasTrivector<F: Float> {
    fn trivector(&self) -> Trivector<F>;
}

impl<F: Float> HasTrivector<F> for Trivector<F> {
    fn trivector(&self) -> Trivector<F> {
        *self
    }
}

impl<F: Float> HasTrivector<F> for Multivector<F> {
    fn trivector(&self) -> Trivector<F> {
        self.trivector() // Or however you currently implement vector()
    }
}

pub trait HasMultivector<F: Float> {
    fn multivector(&self) -> Multivector<F>;
}

impl<F: Float> HasMultivector<F> for Multivector<F> {
    fn multivector(&self) -> Multivector<F> {
        *self
    }
}

pub trait Projectable<T, F: Float> {
    type Output;

    fn try_project(self, target: T) -> Option<Self::Output>
    where
        T: VGA3DOps<F> + Copy;
}

// For vectors
impl<T, F: Float> Projectable<T, F> for Vector<F>
where
    T: VGA3DOps<F> + Copy,
    Vector<F>: core::ops::BitOr<T>,
    <Vector<F> as core::ops::BitOr<T>>::Output: core::ops::Mul<T>,
    <<Vector<F> as core::ops::BitOr<T>>::Output as core::ops::Mul<T>>::Output: HasVector<F>,
{
    type Output = Vector<F>;
    fn try_project(self, b: T) -> Option<Self::Output> {
        match b.try_inverse() {
            None => None,
            Some(b_inverse) => Some(((self | b) * (b_inverse)).vector()),
        }
    }
}

impl<T, F: Float> Projectable<T, F> for Bivector<F>
where
    T: VGA3DOps<F> + Copy,
    Bivector<F>: core::ops::BitOr<T>,
    <Bivector<F> as core::ops::BitOr<T>>::Output: core::ops::Mul<T>,
    <<Bivector<F> as core::ops::BitOr<T>>::Output as core::ops::Mul<T>>::Output: HasBivector<F>,
{
    type Output = Bivector<F>;
    fn try_project(self, b: T) -> Option<Self::Output> {
        match b.try_inverse() {
            None => None,
            Some(b_inverse) => Some(((self | b) * (b_inverse)).bivector()),
        }
    }
}

impl<T, F: Float> Projectable<T, F> for Trivector<F>
where
    T: VGA3DOps<F> + Copy,
    Trivector<F>: core::ops::BitOr<T>,
    <Trivector<F> as core::ops::BitOr<T>>::Output: core::ops::Mul<T>,
    <<Trivector<F> as core::ops::BitOr<T>>::Output as core::ops::Mul<T>>::Output: HasTrivector<F>,
{
    type Output = Trivector<F>;
    fn try_project(self, b: T) -> Option<Self::Output> {
        match b.try_inverse() {
            None => None,
            Some(b_inverse) => Some(((self | b) * (b_inverse)).trivector()),
        }
    }
}

impl<T, F: Float> Projectable<T, F> for Multivector<F>
where
    T: VGA3DOps<F> + Copy,
    Multivector<F>: core::ops::BitOr<T>,
    <Multivector<F> as core::ops::BitOr<T>>::Output: core::ops::Mul<T>,
    <<Multivector<F> as core::ops::BitOr<T>>::Output as core::ops::Mul<T>>::Output:
        HasMultivector<F>,
{
    type Output = Multivector<F>;
    fn try_project(self, b: T) -> Option<Self::Output> {
        match b.try_inverse() {
            None => None,
            Some(b_inverse) => Some(((self | b) * (b_inverse)).multivector()),
        }
    }
}

#[cfg(test)]
mod projection {
    use super::*;
    use approx::assert_relative_eq;
    use core::f32::consts::TAU;

    #[test]
    fn vector_vector() {
        let vector1 = Vector::new(2.0, 0.0, 3.0);
        let vector2 = Vector::new(-2.0, 0.0, 4.0);
        let res = match vector1.try_project(vector2) {
            None => vector1,
            Some(vec_res) => vec_res,
        };
        assert_relative_eq!(res.e1(), -0.8, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 1.6, max_relative = 0.000001);
    }

    #[test]
    fn vector_bivector() {
        // 2e1+3e3
        let vector = Vector::new(2.0, 0.0, 3.0);
        // -2e12+4e23
        let bivector = Bivector::new(-2.0, 0.0, 4.0);
        // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
        // 1.5999999046e1​+3.1999998093e3
        let res = match vector.try_project(bivector) {
            None => vector,
            Some(vec_res) => vec_res,
        };

        assert_relative_eq!(res.e1(), 1.6, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 3.2, max_relative = 0.000001);
    }

    #[test]
    fn vector_trivector() {
        // 2e1+3e3
        let vector = Vector::new(2.0, 0.0, 3.0);
        // -2e12+4e23
        let trivector = Trivector::new(-2.0);
        // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
        // 1.5999999046e1​+3.1999998093e3
        let res = match vector.try_project(trivector) {
            None => vector,
            Some(vector_res) => vector_res,
        };

        assert_relative_eq!(res.e1(), 2.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 3.0, max_relative = 0.000001);
    }
    #[test]
    fn bivector_vector() {
        // The projected bivector is orthogonal to the dual of the vector it is projected onto
        let bivector = Bivector::new(-2.0, 5.0, -4.0);
        let vector = Vector::new(3.0, 4.0, 0.0);
        let res = match bivector.try_project(vector) {
            None => bivector,
            Some(bivector_res) => bivector_res,
        };
        let dual = vector.dual();
        let inner = dual | res;
        assert_relative_eq!(inner.0, 0.0, max_relative = 0.0001);
    }

    // #[test]
    // fn bivector_bivector() {
    //     let bivector1 = Vector::new(2.0, 0.0, 3.0);
    //     let bivector2 = Bivector::new(-2.0, 0.0, 4.0);
    //     let res = bivector1.project(bivector2);
    //     assert_relative_eq!(res.e1(), 1.6, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.2, max_relative = 0.000001);
    // }

    // #[test]
    // fn bivector_trivector() {
    //     // 2e1+3e3
    //     let vector = Vector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let trivector = Trivector::new(-2.0);
    //     // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
    //     // 1.5999999046e1​+3.1999998093e3
    //     let res = vector.project(trivector);
    //     assert_relative_eq!(res.e1(), 2.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.0, max_relative = 0.000001);
    // }
}

// Rejection
pub trait Rejectable<T, F: Float> {
    type Output;

    fn try_reject(self, target: T) -> Option<Self::Output>
    where
        T: VGA3DOps<F> + Copy;
}

// For vectors
impl<T, F: Float> Rejectable<T, F> for Vector<F>
where
    T: VGA3DOps<F> + Copy,
    Vector<F>: core::ops::BitXor<T>,
    <Vector<F> as core::ops::BitXor<T>>::Output: core::ops::Mul<T>,
    <<Vector<F> as core::ops::BitXor<T>>::Output as core::ops::Mul<T>>::Output: HasVector<F>,
{
    type Output = Vector<F>;
    fn try_reject(self, b: T) -> Option<Self::Output> {
        match b.try_inverse() {
            None => None,
            Some(b_inverse) => Some(((self ^ b) * (b_inverse)).vector()),
        }
    }
}

impl<T, F: Float> Rejectable<T, F> for Bivector<F>
where
    T: VGA3DOps<F> + Copy,
    Bivector<F>: core::ops::BitXor<T>,
    <Bivector<F> as core::ops::BitXor<T>>::Output: core::ops::Mul<T>,
    <<Bivector<F> as core::ops::BitXor<T>>::Output as core::ops::Mul<T>>::Output: HasBivector<F>,
{
    type Output = Bivector<F>;
    fn try_reject(self, b: T) -> Option<Self::Output> {
        match b.try_inverse() {
            None => None,
            Some(b_inverse) => Some(((self ^ b) * b_inverse).bivector()),
        }
    }
}

impl<T, F: Float> Rejectable<T, F> for Trivector<F>
where
    T: VGA3DOps<F> + Copy,
    Trivector<F>: core::ops::BitXor<T>,
    <Trivector<F> as core::ops::BitXor<T>>::Output: core::ops::Mul<T>,
    <<Trivector<F> as core::ops::BitXor<T>>::Output as core::ops::Mul<T>>::Output: HasTrivector<F>,
{
    type Output = Trivector<F>;
    fn try_reject(self, b: T) -> Option<Self::Output> {
        match b.try_inverse() {
            None => None,
            Some(b_inverse) => Some(((self ^ b) * b_inverse).trivector()),
        }
    }
}

impl<T, F: Float> Rejectable<T, F> for Multivector<F>
where
    T: VGA3DOps<F> + Copy,
    Multivector<F>: core::ops::BitXor<T>,
    <Multivector<F> as core::ops::BitXor<T>>::Output: core::ops::Mul<T>,
    <<Multivector<F> as core::ops::BitXor<T>>::Output as core::ops::Mul<T>>::Output:
        HasMultivector<F>,
{
    type Output = Multivector<F>;
    fn try_reject(self, b: T) -> Option<Self::Output> {
        match b.try_inverse() {
            None => None,
            Some(b_inverse) => Some(((self ^ b) * b_inverse).multivector()),
        }
    }
}

#[cfg(test)]
mod rejection {
    use super::*;
    use approx::assert_relative_eq;
    use core::f32::consts::TAU;

    // #[test]
    // fn vector_vector() {
    //     let vector1 = Vector::new(2.0, 0.0, 3.0);
    //     let vector2 = Vector::new(-2.0, 0.0, 4.0);
    //     let res = vector1.project(vector2);
    //     assert_relative_eq!(res.e1(), -0.8, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 1.6, max_relative = 0.000001);
    // }

    // #[test]
    // fn vector_bivector() {
    //     // 2e1+3e3
    //     let vector = Vector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let bivector = Bivector::new(-2.0, 0.0, 4.0);
    //     // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
    //     // 1.5999999046e1​+3.1999998093e3
    //     let res = vector.project(bivector);
    //     assert_relative_eq!(res.e1(), 1.6, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.2, max_relative = 0.000001);
    // }

    // #[test]
    // fn vector_trivector() {
    //     // 2e1+3e3
    //     let vector = Vector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let trivector = Trivector::new(-2.0);
    //     // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
    //     // 1.5999999046e1​+3.1999998093e3
    //     let res = vector.project(trivector);
    //     assert_relative_eq!(res.e1(), 2.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.0, max_relative = 0.000001);
    // }
    // #[test]
    // fn bivector_vector() {
    //     // The projected bivector is orthogonal to the dual of the vector it is projected onto
    //     let bivector = Bivector::new(-2.0, 5.0, -4.0);
    //     let vector = Vector::new(3.0, 4.0, 0.0);
    //     let res = bivector.project(vector);
    //     let dual = vector.dual();
    //     let inner = dual | res;
    //     assert_relative_eq!(inner, 0.0, max_relative = 0.0001);
    // }

    // #[test]
    // fn bivector_bivector() {
    //     let bivector1 = Vector::new(2.0, 0.0, 3.0);
    //     let bivector2 = Bivector::new(-2.0, 0.0, 4.0);
    //     let res = bivector1.project(bivector2);
    //     assert_relative_eq!(res.e1(), 1.6, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.2, max_relative = 0.000001);
    // }

    // #[test]
    // fn bivector_trivector() {
    //     // 2e1+3e3
    //     let vector = Vector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let trivector = Trivector::new(-2.0);
    //     // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
    //     // 1.5999999046e1​+3.1999998093e3
    //     let res = vector.project(trivector);
    //     assert_relative_eq!(res.e1(), 2.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.0, max_relative = 0.000001);
    // }
}

// Reflection
// \[A' = B^{-1 }A B\]
pub trait Reflectable<T, F: Float> {
    type Output;

    fn try_reflect(self, target: T) -> Option<Self::Output>
    where
        T: VGA3DOps<F> + Copy;
}

impl<T, F: Float> Reflectable<T, F> for Vector<F>
where
    T: VGA3DOps<F> + Copy,
    T: core::ops::Mul<Vector<F>>, // T can multiply a vector
    <T as core::ops::Mul<Vector<F>>>::Output: core::ops::Mul<T>, // Result can multiply T
    <<T as core::ops::Mul<Vector<F>>>::Output as core::ops::Mul<T>>::Output: HasVector<F>, // Result has vector method
{
    type Output = Vector<F>;
    fn try_reflect(self, b: T) -> Option<Self::Output> {
        match b.try_inverse() {
            None => None,
            Some(b_inverse) => Some(((b_inverse * self) * b).vector()),
        }
    }
}

impl<T, F: Float> Reflectable<T, F> for Bivector<F>
where
    T: VGA3DOps<F> + Copy,
    T: core::ops::Mul<Bivector<F>>, // T can multiply a vector
    <T as core::ops::Mul<Bivector<F>>>::Output: core::ops::Mul<T>, // Result can multiply T
    <<T as core::ops::Mul<Bivector<F>>>::Output as core::ops::Mul<T>>::Output: HasBivector<F>, // Result has vector method
{
    type Output = Bivector<F>;
    fn try_reflect(self, b: T) -> Option<Self::Output> {
        match b.try_inverse() {
            None => None,
            Some(b_inverse) => Some(((b_inverse * self) * b).bivector()),
        }
    }
}

impl<T, F: Float> Reflectable<T, F> for Trivector<F>
where
    T: VGA3DOps<F> + Copy,
    T: core::ops::Mul<Trivector<F>>, // T can multiply a vector
    <T as core::ops::Mul<Trivector<F>>>::Output: core::ops::Mul<T>, // Result can multiply T
    <<T as core::ops::Mul<Trivector<F>>>::Output as core::ops::Mul<T>>::Output: HasTrivector<F>, // Result has vector method
{
    type Output = Trivector<F>;
    fn try_reflect(self, b: T) -> Option<Self::Output> {
        match b.try_inverse() {
            None => None,
            Some(b_inverse) => Some(((b_inverse * self) * b).trivector()),
        }
    }
}

impl<T, F: Float> Reflectable<T, F> for Multivector<F>
where
    T: VGA3DOps<F> + Copy,
    T: core::ops::Mul<Multivector<F>>, // T can multiply a vector
    <T as core::ops::Mul<Multivector<F>>>::Output: core::ops::Mul<T>, // Result can multiply T
    <<T as core::ops::Mul<Multivector<F>>>::Output as core::ops::Mul<T>>::Output: HasMultivector<F>, // Result has vector method
{
    type Output = Multivector<F>;
    fn try_reflect(self, b: T) -> Option<Self::Output> {
        match b.try_inverse() {
            None => None,
            Some(b_inverse) => Some(((b_inverse * self) * b).multivector()),
        }
    }
}

#[cfg(test)]
mod reflection {
    use super::*;
    use approx::assert_relative_eq;
    use core::f32::consts::TAU;

    #[test]
    fn vector_vector() {
        // A vector reflection is the projection minus a rejction
        // 2e1 + 3e3
        let vector1 = Vector::new(2.0, 0.0, 3.0);
        // -2e1 + 4e3
        let vector2 = Vector::new(-2.0, 0.0, 4.0);
        // −72e1​+4e3
        let res = match vector1.try_reflect(vector2) {
            None => vector1,
            Some(vector_res) => vector_res,
        };
        let test = match vector1.try_project(vector2) {
            None => vector1,
            Some(vector_res1) => match vector1.try_reject(vector2) {
                None => vector_res1 - vector1,
                Some(vector_res2) => vector_res1 - vector_res2,
            },
        };
        assert_relative_eq!(res.e1(), test.e1(), max_relative = 0.000001);
        assert_relative_eq!(res.e2(), test.e2(), max_relative = 0.000001);
        assert_relative_eq!(res.e3(), test.e3(), max_relative = 0.000001);
    }

    // #[test]
    // fn vector_bivector() {
    //     // 2e1+3e3
    //     let vector = Vector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let bivector = Bivector::new(-2.0, 0.0, 4.0);
    //     // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
    //     // 1.5999999046e1​+3.1999998093e3
    //     let res = vector.project(bivector);
    //     assert_relative_eq!(res.e1(), 1.6, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.2, max_relative = 0.000001);
    // }

    // #[test]
    // fn vector_trivector() {
    //     // 2e1+3e3
    //     let vector = Vector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let trivector = Trivector::new(-2.0);
    //     // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
    //     // 1.5999999046e1​+3.1999998093e3
    //     let res = vector.project(trivector);
    //     assert_relative_eq!(res.e1(), 2.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.0, max_relative = 0.000001);
    // }
    // #[test]
    // fn bivector_vector() {
    //     // The projected bivector is orthogonal to the dual of the vector it is projected onto
    //     let bivector = Bivector::new(-2.0, 5.0, -4.0);
    //     let vector = Vector::new(3.0, 4.0, 0.0);
    //     let res = bivector.project(vector);
    //     let dual = vector.dual();
    //     let inner = dual | res;
    //     assert_relative_eq!(inner, 0.0, max_relative = 0.0001);
    // }

    // #[test]
    // fn bivector_bivector() {
    //     let bivector1 = Vector::new(2.0, 0.0, 3.0);
    //     let bivector2 = Bivector::new(-2.0, 0.0, 4.0);
    //     let res = bivector1.project(bivector2);
    //     assert_relative_eq!(res.e1(), 1.6, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.2, max_relative = 0.000001);
    // }

    // #[test]
    // fn bivector_trivector() {
    //     // 2e1+3e3
    //     let vector = Vector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let trivector = Trivector::new(-2.0);
    //     // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
    //     // 1.5999999046e1​+3.1999998093e3
    //     let res = vector.project(trivector);
    //     assert_relative_eq!(res.e1(), 2.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.0, max_relative = 0.000001);
    // }
}
