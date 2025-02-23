#![allow(unused_imports)]
#![allow(dead_code)]

use super::{
    bivector::{self, VGA3DBivector},
    multivector::VGA3DMultivector,
    rotor::VGA3DRotor,
    trivector::{self, VGA3DTrivector},
    vector::VGA3DVector,
    VGA3DOpsRef,
};

// Functions
// Rotation
pub trait Rotatable {
    type Output;

    fn rotate(self, rotor: &VGA3DRotor) -> Self::Output;
}
impl Rotatable for VGA3DVector {
    type Output = VGA3DVector;

    fn rotate(self, rotor: &VGA3DRotor) -> Self::Output {
        (rotor.reverse() * self * rotor).vector()
    }
}
impl Rotatable for VGA3DBivector {
    type Output = VGA3DBivector;

    fn rotate(self, rotor: &VGA3DRotor) -> Self::Output {
        (rotor.reverse() * self * rotor).bivector()
    }
}
impl Rotatable for VGA3DTrivector {
    type Output = VGA3DTrivector;

    fn rotate(self, rotor: &VGA3DRotor) -> Self::Output {
        (rotor.reverse() * self * rotor).trivector()
    }
}
impl Rotatable for VGA3DMultivector {
    type Output = VGA3DMultivector;

    fn rotate(self, rotor: &VGA3DRotor) -> Self::Output {
        rotor.reverse() * self * rotor
    }
}

// Projection
// Rejection
// Reflection
