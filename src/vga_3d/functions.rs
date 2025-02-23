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
    use crate::vga_3d::rotor;

    use super::*;
    use approx::assert_relative_eq;
    use core::f32::consts::TAU;

    #[test]
    fn vec_rot_quarter_e1e2() {
        let vector = VGA3DVector::new(3.0, 0.0, 0.0);
        let angle = TAU / 4.0;
        let bivector = VGA3DBivector::new(1.0, 0.0, 0.0);
        let rotor = VGA3DRotor::new(angle, bivector);
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
        let rotor = VGA3DRotor::new(angle, rotation_plane);
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

// pub trait Projectable {
//     type Output;
//     fn project<T, P>(self, target: T) -> Self::Output
//     where
//         T: VGA3DOps + Copy,
//         Self: core::ops::BitOr<T, Output = P>,
//         P: core::ops::Mul<T::Output>;
// }

// // Macro for implementing Projectable
// macro_rules! impl_projectable {
//     ($type:ty, $extract:ident) => {
//         impl Projectable for $type {
//             type Output = $type;

//             fn project<T, P>(self, target: T) -> Self::Output
//             where
//                 T: VGA3DOps + Copy,
//                 Self: core::ops::BitOr<T, Output = P>,
//                 P: core::ops::Mul<T::Output>,
//             {
//                 ((self | target) * target.inverse()).$extract()
//             }
//         }
//     };
// }

// // Implement for each type that can be projected
// impl_projectable!(VGA3DVector, vector);
// impl_projectable!(VGA3DBivector, bivector);

// pub trait Projectable {
//     type Output;
//     fn project_line(self, vector: VGA3DVector) -> Self::Output;
//     fn project_plane(self, vector: VGA3DBivector) -> Self::Output;
//     fn project_volume(self, vector: VGA3DTrivector) -> Self::Output;
//     fn project_multi(self, vector: VGA3DMultivector) -> Self::Output;
// }

// impl Projectable for VGA3DVector {
//     type Output = VGA3DVector;
//     fn project_line(self, b: VGA3DVector) -> Self::Output {
//         ((self | b) * (b.inverse())).vector()
//     }
//     fn project_plane(self, b: VGA3DBivector) -> Self::Output {
//         ((self | b) * (b.inverse())).vector()
//     }
//     fn project_volume(self, b: VGA3DTrivector) -> Self::Output {
//         ((self | b) * (b.inverse())).vector()
//     }
//     fn project_multi(self, b: VGA3DMultivector) -> Self::Output {
//         ((self | b) * (b.inverse())).vector()
//     }
// }
// impl Projectable for VGA3DBivector {
//     type Output = VGA3DBivector;
//     fn project_line(self, b: VGA3DVector) -> Self::Output {
//         ((self | b) * (b.inverse())).bivector()
//     }
//     fn project_plane(self, b: VGA3DBivector) -> Self::Output {
//         ((self | b) * (b.inverse())).bivector()
//     }
//     fn project_volume(self, b: VGA3DTrivector) -> Self::Output {
//         ((self | b) * (b.inverse())).bivector()
//     }
//     fn project_multi(self, b: VGA3DMultivector) -> Self::Output {
//         ((self | b) * (b.inverse())).bivector()
//     }
// }

// Rejection
// Reflection
