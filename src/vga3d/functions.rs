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

use super::{
    bivector::VGA3DBivector, multivector::VGA3DMultivector, rotor::VGA3DRotor,
    trivector::VGA3DTrivector, vector::VGA3DVector, VGA3DOps, VGA3DOpsRef,
};

// Functions
// Rotation
pub trait Rotatable<R = VGA3DRotor> {
    type Output;
    fn rotate(self, rotor: R) -> Self::Output;
}

macro_rules! impl_rotatable {
    ($vec:ty, $output:ty, $extract:ident) => {
        // Owned vector, owned rotor
        impl Rotatable for $vec {
            type Output = $output;
            fn rotate(self, rotor: VGA3DRotor) -> Self::Output {
                (rotor.reverse() * self * rotor).$extract()
            }
        }

        // Owned vector, reference rotor
        impl Rotatable<&VGA3DRotor> for $vec {
            type Output = $output;
            fn rotate(self, rotor: &VGA3DRotor) -> Self::Output {
                (rotor.reverse() * self * *rotor).$extract()
            }
        }

        // Reference vector, owned rotor
        impl<'a> Rotatable for &'a $vec {
            type Output = $output;
            fn rotate(self, rotor: VGA3DRotor) -> Self::Output {
                (rotor.reverse() * *self * rotor).$extract()
            }
        }

        // Reference vector, reference rotor
        impl<'a, 'b> Rotatable<&'b VGA3DRotor> for &'a $vec {
            type Output = $output;
            fn rotate(self, rotor: &VGA3DRotor) -> Self::Output {
                (rotor.reverse() * *self * *rotor).$extract()
            }
        }
    };
}

// Usage:
impl_rotatable!(VGA3DVector, VGA3DVector, vector);
impl_rotatable!(VGA3DBivector, VGA3DBivector, bivector);
impl_rotatable!(VGA3DTrivector, VGA3DTrivector, trivector);
impl_rotatable!(VGA3DMultivector, VGA3DMultivector, multivector);

#[cfg(test)]
mod rotation {
    use crate::vga3d::rotor;

    use super::*;
    use approx::assert_relative_eq;
    use core::f32::consts::TAU;

    #[test]
    fn vec_rot_quarter_e1e2() {
        let vector = VGA3DVector::new(3.0, 0.0, 0.0);
        let angle = TAU / 4.0;
        let bivector = VGA3DBivector::new(1.0, 0.0, 0.0);
        let rotor = VGA3DRotor::new(angle / 2.0, bivector);
        let vector_rot_ref1 = (&vector).rotate(&rotor);
        let vector_rot_ref2 = (&vector).rotate(rotor);
        let vector_rot_ref3 = vector.rotate(&rotor);
        let vector_rot = vector.rotate(rotor);

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
    fn bivector() {
        let angle = TAU / 4.0;
        let rotation_plane = VGA3DBivector::new(0.5, 5.2, -3.0);
        let rotor = VGA3DRotor::new(angle / 2.0, rotation_plane);
        let bivector = VGA3DBivector::new(6.4, -4.5, 3.3);
        let bivector_rot = bivector.rotate(rotor);

        assert_relative_eq!(bivector_rot.e12(), -1.0222723, max_relative = 0.000001);
        assert_relative_eq!(bivector_rot.e31(), -0.8519465, max_relative = 0.000001);
        assert_relative_eq!(bivector_rot.e23(), 8.386248, max_relative = 0.000001);
    }

    // #[test]
    // fn vector() {
    //     let angle = TAU / 4;
    //     let rotation_plane = VGA3DBivector::new(1.0, 0.0, 0.0);
    //     let vector = VGA3DVector::new(e1, e2, e3)
    //     assert_eq!(trivec.e123, 2.0);
    // }
}

// Projection
// \[A_\parallel = (A\cdot B)B^{-1}\]
pub trait HasVector {
    fn vector(&self) -> VGA3DVector;
}

impl HasVector for VGA3DVector {
    fn vector(&self) -> VGA3DVector {
        *self
    }
}

impl HasVector for VGA3DMultivector {
    fn vector(&self) -> VGA3DVector {
        self.vector() // Or however you currently implement vector()
    }
}

pub trait HasBivector {
    fn bivector(&self) -> VGA3DBivector;
}

impl HasBivector for VGA3DBivector {
    fn bivector(&self) -> VGA3DBivector {
        *self
    }
}

impl HasBivector for VGA3DMultivector {
    fn bivector(&self) -> VGA3DBivector {
        self.bivector() // Or however you currently implement vector()
    }
}

pub trait HasTrivector {
    fn trivector(&self) -> VGA3DTrivector;
}

impl HasTrivector for VGA3DTrivector {
    fn trivector(&self) -> VGA3DTrivector {
        *self
    }
}

impl HasTrivector for VGA3DMultivector {
    fn trivector(&self) -> VGA3DTrivector {
        self.trivector() // Or however you currently implement vector()
    }
}

pub trait HasMultivector {
    fn multivector(&self) -> VGA3DMultivector;
}

impl HasMultivector for VGA3DMultivector {
    fn multivector(&self) -> VGA3DMultivector {
        *self
    }
}

pub trait Projectable<T> {
    type Output;

    fn project(self, target: T) -> Self::Output
    where
        T: VGA3DOps + Copy;
}

// For vectors
impl<T> Projectable<T> for VGA3DVector
where
    T: VGA3DOps + Copy,
    VGA3DVector: core::ops::BitOr<T>,
    <VGA3DVector as core::ops::BitOr<T>>::Output: core::ops::Mul<T>,
    <<VGA3DVector as core::ops::BitOr<T>>::Output as core::ops::Mul<T>>::Output: HasVector,
{
    type Output = VGA3DVector;
    fn project(self, b: T) -> Self::Output {
        ((self | b) * (b.inverse())).vector()
    }
}

impl<T> Projectable<T> for VGA3DBivector
where
    T: VGA3DOps + Copy,
    VGA3DBivector: core::ops::BitOr<T>,
    <VGA3DBivector as core::ops::BitOr<T>>::Output: core::ops::Mul<T>,
    <<VGA3DBivector as core::ops::BitOr<T>>::Output as core::ops::Mul<T>>::Output: HasBivector,
{
    type Output = VGA3DBivector;

    fn project(self, b: T) -> Self::Output {
        ((self | b) * (b.inverse())).bivector()
    }
}

impl<T> Projectable<T> for VGA3DTrivector
where
    T: VGA3DOps + Copy,
    VGA3DTrivector: core::ops::BitOr<T>,
    <VGA3DTrivector as core::ops::BitOr<T>>::Output: core::ops::Mul<T>,
    <<VGA3DTrivector as core::ops::BitOr<T>>::Output as core::ops::Mul<T>>::Output: HasTrivector,
{
    type Output = VGA3DTrivector;

    fn project(self, b: T) -> Self::Output {
        ((self | b) * (b.inverse())).trivector()
    }
}

impl<T> Projectable<T> for VGA3DMultivector
where
    T: VGA3DOps + Copy,
    VGA3DMultivector: core::ops::BitOr<T>,
    <VGA3DMultivector as core::ops::BitOr<T>>::Output: core::ops::Mul<T>,
    <<VGA3DMultivector as core::ops::BitOr<T>>::Output as core::ops::Mul<T>>::Output:
        HasMultivector,
{
    type Output = VGA3DMultivector;

    fn project(self, b: T) -> Self::Output {
        ((self | b) * (b.inverse())).multivector()
    }
}

#[cfg(test)]
mod projection {
    use super::*;
    use approx::assert_relative_eq;
    use core::f32::consts::TAU;

    #[test]
    fn vector_vector() {
        let vector1 = VGA3DVector::new(2.0, 0.0, 3.0);
        let vector2 = VGA3DVector::new(-2.0, 0.0, 4.0);
        let res = vector1.project(vector2);
        assert_relative_eq!(res.e1(), -0.8, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 1.6, max_relative = 0.000001);
    }

    #[test]
    fn vector_bivector() {
        // 2e1+3e3
        let vector = VGA3DVector::new(2.0, 0.0, 3.0);
        // -2e12+4e23
        let bivector = VGA3DBivector::new(-2.0, 0.0, 4.0);
        // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
        // 1.5999999046e1​+3.1999998093e3
        let res = vector.project(bivector);
        assert_relative_eq!(res.e1(), 1.6, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 3.2, max_relative = 0.000001);
    }

    #[test]
    fn vector_trivector() {
        // 2e1+3e3
        let vector = VGA3DVector::new(2.0, 0.0, 3.0);
        // -2e12+4e23
        let trivector = VGA3DTrivector::new(-2.0);
        // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
        // 1.5999999046e1​+3.1999998093e3
        let res = vector.project(trivector);
        assert_relative_eq!(res.e1(), 2.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 3.0, max_relative = 0.000001);
    }
    #[test]
    fn bivector_vector() {
        // The projected bivector is orthogonal to the dual of the vector it is projected onto
        let bivector = VGA3DBivector::new(-2.0, 5.0, -4.0);
        let vector = VGA3DVector::new(3.0, 4.0, 0.0);
        let res = bivector.project(vector);
        let dual = vector.dual();
        let inner = dual | res;
        assert_relative_eq!(inner, 0.0, max_relative = 0.0001);
    }

    // #[test]
    // fn bivector_bivector() {
    //     let bivector1 = VGA3DVector::new(2.0, 0.0, 3.0);
    //     let bivector2 = VGA3DBivector::new(-2.0, 0.0, 4.0);
    //     let res = bivector1.project(bivector2);
    //     assert_relative_eq!(res.e1(), 1.6, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.2, max_relative = 0.000001);
    // }

    // #[test]
    // fn bivector_trivector() {
    //     // 2e1+3e3
    //     let vector = VGA3DVector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let trivector = VGA3DTrivector::new(-2.0);
    //     // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
    //     // 1.5999999046e1​+3.1999998093e3
    //     let res = vector.project(trivector);
    //     assert_relative_eq!(res.e1(), 2.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.0, max_relative = 0.000001);
    // }
}

// Rejection
pub trait Rejectable<T> {
    type Output;

    fn reject(self, target: T) -> Self::Output
    where
        T: VGA3DOps + Copy;
}

// For vectors
impl<T> Rejectable<T> for VGA3DVector
where
    T: VGA3DOps + Copy,
    VGA3DVector: core::ops::BitXor<T>,
    <VGA3DVector as core::ops::BitXor<T>>::Output: core::ops::Mul<T>,
    <<VGA3DVector as core::ops::BitXor<T>>::Output as core::ops::Mul<T>>::Output: HasVector,
{
    type Output = VGA3DVector;
    fn reject(self, b: T) -> Self::Output {
        ((self ^ b) * (b.inverse())).vector()
    }
}

impl<T> Rejectable<T> for VGA3DBivector
where
    T: VGA3DOps + Copy,
    VGA3DBivector: core::ops::BitXor<T>,
    <VGA3DBivector as core::ops::BitXor<T>>::Output: core::ops::Mul<T>,
    <<VGA3DBivector as core::ops::BitXor<T>>::Output as core::ops::Mul<T>>::Output: HasBivector,
{
    type Output = VGA3DBivector;

    fn reject(self, b: T) -> Self::Output {
        ((self ^ b) * (b.inverse())).bivector()
    }
}

impl<T> Rejectable<T> for VGA3DTrivector
where
    T: VGA3DOps + Copy,
    VGA3DTrivector: core::ops::BitXor<T>,
    <VGA3DTrivector as core::ops::BitXor<T>>::Output: core::ops::Mul<T>,
    <<VGA3DTrivector as core::ops::BitXor<T>>::Output as core::ops::Mul<T>>::Output: HasTrivector,
{
    type Output = VGA3DTrivector;

    fn reject(self, b: T) -> Self::Output {
        ((self ^ b) * (b.inverse())).trivector()
    }
}

impl<T> Rejectable<T> for VGA3DMultivector
where
    T: VGA3DOps + Copy,
    VGA3DMultivector: core::ops::BitXor<T>,
    <VGA3DMultivector as core::ops::BitXor<T>>::Output: core::ops::Mul<T>,
    <<VGA3DMultivector as core::ops::BitXor<T>>::Output as core::ops::Mul<T>>::Output:
        HasMultivector,
{
    type Output = VGA3DMultivector;

    fn reject(self, b: T) -> Self::Output {
        ((self ^ b) * (b.inverse())).multivector()
    }
}

#[cfg(test)]
mod rejection {
    use super::*;
    use approx::assert_relative_eq;
    use core::f32::consts::TAU;

    // #[test]
    // fn vector_vector() {
    //     let vector1 = VGA3DVector::new(2.0, 0.0, 3.0);
    //     let vector2 = VGA3DVector::new(-2.0, 0.0, 4.0);
    //     let res = vector1.project(vector2);
    //     assert_relative_eq!(res.e1(), -0.8, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 1.6, max_relative = 0.000001);
    // }

    // #[test]
    // fn vector_bivector() {
    //     // 2e1+3e3
    //     let vector = VGA3DVector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let bivector = VGA3DBivector::new(-2.0, 0.0, 4.0);
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
    //     let vector = VGA3DVector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let trivector = VGA3DTrivector::new(-2.0);
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
    //     let bivector = VGA3DBivector::new(-2.0, 5.0, -4.0);
    //     let vector = VGA3DVector::new(3.0, 4.0, 0.0);
    //     let res = bivector.project(vector);
    //     let dual = vector.dual();
    //     let inner = dual | res;
    //     assert_relative_eq!(inner, 0.0, max_relative = 0.0001);
    // }

    // #[test]
    // fn bivector_bivector() {
    //     let bivector1 = VGA3DVector::new(2.0, 0.0, 3.0);
    //     let bivector2 = VGA3DBivector::new(-2.0, 0.0, 4.0);
    //     let res = bivector1.project(bivector2);
    //     assert_relative_eq!(res.e1(), 1.6, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.2, max_relative = 0.000001);
    // }

    // #[test]
    // fn bivector_trivector() {
    //     // 2e1+3e3
    //     let vector = VGA3DVector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let trivector = VGA3DTrivector::new(-2.0);
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
pub trait Reflectable<T> {
    type Output;

    fn reflect(self, target: T) -> Self::Output
    where
        T: VGA3DOps + Copy;
}

impl<T> Reflectable<T> for VGA3DVector
where
    T: VGA3DOps + Copy,
    T: core::ops::Mul<VGA3DVector>, // T can multiply a vector
    <T as core::ops::Mul<VGA3DVector>>::Output: core::ops::Mul<T>, // Result can multiply T
    <<T as core::ops::Mul<VGA3DVector>>::Output as core::ops::Mul<T>>::Output: HasVector, // Result has vector method
{
    type Output = VGA3DVector;
    fn reflect(self, b: T) -> Self::Output {
        ((b.inverse() * self) * b).vector()
    }
}

impl<T> Reflectable<T> for VGA3DBivector
where
    T: VGA3DOps + Copy,
    T: core::ops::Mul<VGA3DBivector>, // T can multiply a vector
    <T as core::ops::Mul<VGA3DBivector>>::Output: core::ops::Mul<T>, // Result can multiply T
    <<T as core::ops::Mul<VGA3DBivector>>::Output as core::ops::Mul<T>>::Output: HasBivector, // Result has vector method
{
    type Output = VGA3DBivector;
    fn reflect(self, b: T) -> Self::Output {
        ((b.inverse() * self) * b).bivector()
    }
}

impl<T> Reflectable<T> for VGA3DTrivector
where
    T: VGA3DOps + Copy,
    T: core::ops::Mul<VGA3DTrivector>, // T can multiply a vector
    <T as core::ops::Mul<VGA3DTrivector>>::Output: core::ops::Mul<T>, // Result can multiply T
    <<T as core::ops::Mul<VGA3DTrivector>>::Output as core::ops::Mul<T>>::Output: HasTrivector, // Result has vector method
{
    type Output = VGA3DTrivector;
    fn reflect(self, b: T) -> Self::Output {
        ((b.inverse() * self) * b).trivector()
    }
}

impl<T> Reflectable<T> for VGA3DMultivector
where
    T: VGA3DOps + Copy,
    T: core::ops::Mul<VGA3DMultivector>, // T can multiply a vector
    <T as core::ops::Mul<VGA3DMultivector>>::Output: core::ops::Mul<T>, // Result can multiply T
    <<T as core::ops::Mul<VGA3DMultivector>>::Output as core::ops::Mul<T>>::Output: HasMultivector, // Result has vector method
{
    type Output = VGA3DMultivector;
    fn reflect(self, b: T) -> Self::Output {
        ((b.inverse() * self) * b).multivector()
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
        let vector1 = VGA3DVector::new(2.0, 0.0, 3.0);
        // -2e1 + 4e3
        let vector2 = VGA3DVector::new(-2.0, 0.0, 4.0);
        // −72e1​+4e3
        let res = vector1.reflect(vector2);
        let test = vector1.project(vector2) - vector1.reject(vector2);

        assert_relative_eq!(res.e1(), test.e1(), max_relative = 0.000001);
        assert_relative_eq!(res.e2(), test.e2(), max_relative = 0.000001);
        assert_relative_eq!(res.e3(), test.e3(), max_relative = 0.000001);
    }

    // #[test]
    // fn vector_bivector() {
    //     // 2e1+3e3
    //     let vector = VGA3DVector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let bivector = VGA3DBivector::new(-2.0, 0.0, 4.0);
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
    //     let vector = VGA3DVector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let trivector = VGA3DTrivector::new(-2.0);
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
    //     let bivector = VGA3DBivector::new(-2.0, 5.0, -4.0);
    //     let vector = VGA3DVector::new(3.0, 4.0, 0.0);
    //     let res = bivector.project(vector);
    //     let dual = vector.dual();
    //     let inner = dual | res;
    //     assert_relative_eq!(inner, 0.0, max_relative = 0.0001);
    // }

    // #[test]
    // fn bivector_bivector() {
    //     let bivector1 = VGA3DVector::new(2.0, 0.0, 3.0);
    //     let bivector2 = VGA3DBivector::new(-2.0, 0.0, 4.0);
    //     let res = bivector1.project(bivector2);
    //     assert_relative_eq!(res.e1(), 1.6, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.2, max_relative = 0.000001);
    // }

    // #[test]
    // fn bivector_trivector() {
    //     // 2e1+3e3
    //     let vector = VGA3DVector::new(2.0, 0.0, 3.0);
    //     // -2e12+4e23
    //     let trivector = VGA3DTrivector::new(-2.0);
    //     // ( (2e1+3e3) | (-2e12+4e23) ) * (( ~(-2e12+4e23) )/  ((-2e12+4e23) * ~(-2e12+4e23))  )
    //     // 1.5999999046e1​+3.1999998093e3
    //     let res = vector.project(trivector);
    //     assert_relative_eq!(res.e1(), 2.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
    //     assert_relative_eq!(res.e3(), 3.0, max_relative = 0.000001);
    // }
}
