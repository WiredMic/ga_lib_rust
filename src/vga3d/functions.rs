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
    bivector::Bivector, multivector::Multivector, rotor::VGA3DRotor,
    trivector::Trivector, vector::Vector, VGA3DOps, VGA3DOpsRef,
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
impl_rotatable!(Vector, Vector, vector);
impl_rotatable!(Bivector, Bivector, bivector);
impl_rotatable!(Trivector, Trivector, trivector);
impl_rotatable!(Multivector, Multivector, multivector);

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
        let rotation_plane = Bivector::new(0.5, 5.2, -3.0);
        let rotor = VGA3DRotor::new(angle / 2.0, rotation_plane);
        let bivector = Bivector::new(6.4, -4.5, 3.3);
        let bivector_rot = bivector.rotate(rotor);

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
pub trait HasVector {
    fn vector(&self) -> Vector;
}

impl HasVector for Vector {
    fn vector(&self) -> Vector {
        *self
    }
}

impl HasVector for Multivector {
    fn vector(&self) -> Vector {
        self.vector() // Or however you currently implement vector()
    }
}

pub trait HasBivector {
    fn bivector(&self) -> Bivector;
}

impl HasBivector for Bivector {
    fn bivector(&self) -> Bivector {
        *self
    }
}

impl HasBivector for Multivector {
    fn bivector(&self) -> Bivector {
        self.bivector() // Or however you currently implement vector()
    }
}

pub trait HasTrivector {
    fn trivector(&self) -> Trivector;
}

impl HasTrivector for Trivector {
    fn trivector(&self) -> Trivector {
        *self
    }
}

impl HasTrivector for Multivector {
    fn trivector(&self) -> Trivector {
        self.trivector() // Or however you currently implement vector()
    }
}

pub trait HasMultivector {
    fn multivector(&self) -> Multivector;
}

impl HasMultivector for Multivector {
    fn multivector(&self) -> Multivector {
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
impl<T> Projectable<T> for Vector
where
    T: VGA3DOps + Copy,
    Vector: core::ops::BitOr<T>,
    <Vector as core::ops::BitOr<T>>::Output: core::ops::Mul<T>,
    <<Vector as core::ops::BitOr<T>>::Output as core::ops::Mul<T>>::Output: HasVector,
{
    type Output = Vector;
    fn project(self, b: T) -> Self::Output {
        ((self | b) * (b.inverse())).vector()
    }
}

impl<T> Projectable<T> for Bivector
where
    T: VGA3DOps + Copy,
    Bivector: core::ops::BitOr<T>,
    <Bivector as core::ops::BitOr<T>>::Output: core::ops::Mul<T>,
    <<Bivector as core::ops::BitOr<T>>::Output as core::ops::Mul<T>>::Output: HasBivector,
{
    type Output = Bivector;

    fn project(self, b: T) -> Self::Output {
        ((self | b) * (b.inverse())).bivector()
    }
}

impl<T> Projectable<T> for Trivector
where
    T: VGA3DOps + Copy,
    Trivector: core::ops::BitOr<T>,
    <Trivector as core::ops::BitOr<T>>::Output: core::ops::Mul<T>,
    <<Trivector as core::ops::BitOr<T>>::Output as core::ops::Mul<T>>::Output: HasTrivector,
{
    type Output = Trivector;

    fn project(self, b: T) -> Self::Output {
        ((self | b) * (b.inverse())).trivector()
    }
}

impl<T> Projectable<T> for Multivector
where
    T: VGA3DOps + Copy,
    Multivector: core::ops::BitOr<T>,
    <Multivector as core::ops::BitOr<T>>::Output: core::ops::Mul<T>,
    <<Multivector as core::ops::BitOr<T>>::Output as core::ops::Mul<T>>::Output:
        HasMultivector,
{
    type Output = Multivector;

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
        let vector1 = Vector::new(2.0, 0.0, 3.0);
        let vector2 = Vector::new(-2.0, 0.0, 4.0);
        let res = vector1.project(vector2);
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
        let res = vector.project(bivector);
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
        let res = vector.project(trivector);
        assert_relative_eq!(res.e1(), 2.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 3.0, max_relative = 0.000001);
    }
    #[test]
    fn bivector_vector() {
        // The projected bivector is orthogonal to the dual of the vector it is projected onto
        let bivector = Bivector::new(-2.0, 5.0, -4.0);
        let vector = Vector::new(3.0, 4.0, 0.0);
        let res = bivector.project(vector);
        let dual = vector.dual();
        let inner = dual | res;
        assert_relative_eq!(inner, 0.0, max_relative = 0.0001);
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
pub trait Rejectable<T> {
    type Output;

    fn reject(self, target: T) -> Self::Output
    where
        T: VGA3DOps + Copy;
}

// For vectors
impl<T> Rejectable<T> for Vector
where
    T: VGA3DOps + Copy,
    Vector: core::ops::BitXor<T>,
    <Vector as core::ops::BitXor<T>>::Output: core::ops::Mul<T>,
    <<Vector as core::ops::BitXor<T>>::Output as core::ops::Mul<T>>::Output: HasVector,
{
    type Output = Vector;
    fn reject(self, b: T) -> Self::Output {
        ((self ^ b) * (b.inverse())).vector()
    }
}

impl<T> Rejectable<T> for Bivector
where
    T: VGA3DOps + Copy,
    Bivector: core::ops::BitXor<T>,
    <Bivector as core::ops::BitXor<T>>::Output: core::ops::Mul<T>,
    <<Bivector as core::ops::BitXor<T>>::Output as core::ops::Mul<T>>::Output: HasBivector,
{
    type Output = Bivector;

    fn reject(self, b: T) -> Self::Output {
        ((self ^ b) * (b.inverse())).bivector()
    }
}

impl<T> Rejectable<T> for Trivector
where
    T: VGA3DOps + Copy,
    Trivector: core::ops::BitXor<T>,
    <Trivector as core::ops::BitXor<T>>::Output: core::ops::Mul<T>,
    <<Trivector as core::ops::BitXor<T>>::Output as core::ops::Mul<T>>::Output: HasTrivector,
{
    type Output = Trivector;

    fn reject(self, b: T) -> Self::Output {
        ((self ^ b) * (b.inverse())).trivector()
    }
}

impl<T> Rejectable<T> for Multivector
where
    T: VGA3DOps + Copy,
    Multivector: core::ops::BitXor<T>,
    <Multivector as core::ops::BitXor<T>>::Output: core::ops::Mul<T>,
    <<Multivector as core::ops::BitXor<T>>::Output as core::ops::Mul<T>>::Output:
        HasMultivector,
{
    type Output = Multivector;

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
pub trait Reflectable<T> {
    type Output;

    fn reflect(self, target: T) -> Self::Output
    where
        T: VGA3DOps + Copy;
}

impl<T> Reflectable<T> for Vector
where
    T: VGA3DOps + Copy,
    T: core::ops::Mul<Vector>, // T can multiply a vector
    <T as core::ops::Mul<Vector>>::Output: core::ops::Mul<T>, // Result can multiply T
    <<T as core::ops::Mul<Vector>>::Output as core::ops::Mul<T>>::Output: HasVector, // Result has vector method
{
    type Output = Vector;
    fn reflect(self, b: T) -> Self::Output {
        ((b.inverse() * self) * b).vector()
    }
}

impl<T> Reflectable<T> for Bivector
where
    T: VGA3DOps + Copy,
    T: core::ops::Mul<Bivector>, // T can multiply a vector
    <T as core::ops::Mul<Bivector>>::Output: core::ops::Mul<T>, // Result can multiply T
    <<T as core::ops::Mul<Bivector>>::Output as core::ops::Mul<T>>::Output: HasBivector, // Result has vector method
{
    type Output = Bivector;
    fn reflect(self, b: T) -> Self::Output {
        ((b.inverse() * self) * b).bivector()
    }
}

impl<T> Reflectable<T> for Trivector
where
    T: VGA3DOps + Copy,
    T: core::ops::Mul<Trivector>, // T can multiply a vector
    <T as core::ops::Mul<Trivector>>::Output: core::ops::Mul<T>, // Result can multiply T
    <<T as core::ops::Mul<Trivector>>::Output as core::ops::Mul<T>>::Output: HasTrivector, // Result has vector method
{
    type Output = Trivector;
    fn reflect(self, b: T) -> Self::Output {
        ((b.inverse() * self) * b).trivector()
    }
}

impl<T> Reflectable<T> for Multivector
where
    T: VGA3DOps + Copy,
    T: core::ops::Mul<Multivector>, // T can multiply a vector
    <T as core::ops::Mul<Multivector>>::Output: core::ops::Mul<T>, // Result can multiply T
    <<T as core::ops::Mul<Multivector>>::Output as core::ops::Mul<T>>::Output: HasMultivector, // Result has vector method
{
    type Output = Multivector;
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
        let vector1 = Vector::new(2.0, 0.0, 3.0);
        // -2e1 + 4e3
        let vector2 = Vector::new(-2.0, 0.0, 4.0);
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
