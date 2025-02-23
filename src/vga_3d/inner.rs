#![allow(unused_imports)]
#![allow(dead_code)]

use super::{
    bivector::{self, VGA3DBivector},
    multivector::VGA3DMultivector,
    rotor::VGA3DRotor,
    trivector::{self, VGA3DTrivector},
    vector::VGA3DVector,
};

use core::ops::BitOr;

// // Scalar-Scalar
// // \[ a \cdot b\]
// impl BitOr for f32 {
//     type Output = f32;
//     fn bitor(self: f32, b: VGA3DVector) -> f32 {
//         self * b
//     }
// }

// Scalar-Vector
// \[ a \cdot \vec{b}\]
impl BitOr<VGA3DVector> for f32 {
    type Output = VGA3DVector;
    fn bitor(self: f32, b: VGA3DVector) -> VGA3DVector {
        self * b
    }
}

// Vector-Scalar
// \[ \vec{a} \cdot b\]
impl BitOr<f32> for VGA3DVector {
    type Output = VGA3DVector;
    fn bitor(self: VGA3DVector, b: f32) -> VGA3DVector {
        self * b
    }
}

// Scalar-Bivector
// \[ s \cdot \overset\Rightarrow{b}\]
impl BitOr<VGA3DBivector> for f32 {
    type Output = VGA3DBivector;
    fn bitor(self: f32, b: VGA3DBivector) -> VGA3DBivector {
        self * b
    }
}

// Bivector-Scalar
// \[ \overset\Rightarrow{a} \cdot b\]
impl BitOr<f32> for VGA3DBivector {
    type Output = VGA3DBivector;
    fn bitor(self: VGA3DBivector, b: f32) -> VGA3DBivector {
        self * b
    }
}

// Scalar-Trivector
// \[ s \cdot \overset\Rrightarrow{b}\]
impl BitOr<VGA3DTrivector> for f32 {
    type Output = VGA3DTrivector;
    fn bitor(self: f32, b: VGA3DTrivector) -> VGA3DTrivector {
        self * b
    }
}

// Trivector-Scalar
// \[ \overset\Rrightarrow{a} \cdot b\]
impl BitOr<f32> for VGA3DTrivector {
    type Output = VGA3DTrivector;
    fn bitor(self: VGA3DTrivector, b: f32) -> VGA3DTrivector {
        self * b
    }
}

// Scalar-Multivector
// \[ s \cdot B\]
impl BitOr<VGA3DMultivector> for f32 {
    type Output = VGA3DMultivector;
    fn bitor(self: f32, b: VGA3DMultivector) -> VGA3DMultivector {
        self * b
    }
}

// Multivector-Scalar
// \[ B \cdot b\]
impl BitOr<f32> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DMultivector, b: f32) -> VGA3DMultivector {
        self * b
    }
}

// Scalar-Rotor
// \[ s \cdot R\]
impl BitOr<VGA3DRotor> for f32 {
    type Output = VGA3DMultivector;
    fn bitor(self: f32, b: VGA3DRotor) -> VGA3DMultivector {
        self * b
    }
}

//Rotor-Scalar
// \[ R \cdot b\]
impl BitOr<f32> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DRotor, b: f32) -> VGA3DMultivector {
        self * b
    }
}

// Vector-Bivector
// \[ \vec{a} \cdot \overset\Rightarrow{b}\]
impl BitOr<VGA3DBivector> for VGA3DVector {
    type Output = VGA3DVector;
    fn bitor(self: VGA3DVector, b: VGA3DBivector) -> VGA3DVector {
        let e1 = -self.e2() * b.e12() + self.e3() * b.e31();
        let e2 = self.e1() * b.e12() - self.e3() * b.e23();
        let e3 = -self.e1() * b.e31() + self.e2() * b.e23();
        VGA3DVector::new(e1, e2, e3)
    }
}

// Bivector-Vector
// \[ \overset\Rightarrow{b} \cdot \vec{b}\]
impl BitOr<VGA3DVector> for VGA3DBivector {
    type Output = VGA3DVector;
    fn bitor(self: VGA3DBivector, b: VGA3DVector) -> VGA3DVector {
        let e1 = self.e12() * b.e2() - self.e31() * b.e3();
        let e2 = -self.e12() * b.e1() + self.e23() * b.e3();
        let e3 = self.e31() * b.e1() - self.e23() * b.e2();
        VGA3DVector::new(e1, e2, e3)
    }
}

// Vector-Trivector
// \[ \vec{a} \cdot \overset\Rrightarrow{b}\]
impl BitOr<VGA3DTrivector> for VGA3DVector {
    type Output = VGA3DBivector;
    fn bitor(self: VGA3DVector, b: VGA3DTrivector) -> VGA3DBivector {
        self * b
    }
}

// Trivector-Vector
// \[ \overset\Rrightarrow{a} \cdot \vec{b}\]
impl BitOr<VGA3DVector> for VGA3DTrivector {
    type Output = VGA3DBivector;
    fn bitor(self: VGA3DTrivector, b: VGA3DVector) -> VGA3DBivector {
        self * b
    }
}

// Vector-Multivector
// \[ \vec{a} \cdot B\]
impl BitOr<VGA3DMultivector> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DVector, b: VGA3DMultivector) -> VGA3DMultivector {
        let scalar = self | b.vector();
        let vector = (self | b.bivector()) + (self | b.scalar());
        let bivector = self | b.trivector();
        let trivector = VGA3DTrivector::zero();

        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Multivector-Vector
// \[ A \cdot \vec{b}\]
impl BitOr<VGA3DVector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DMultivector, b: VGA3DVector) -> VGA3DMultivector {
        let scalar = self.vector() | b;
        let vector = (self.bivector() | b) + (self.scalar() | b);
        let bivector = self.trivector() | b;
        let trivector = VGA3DTrivector::zero();

        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Vector-Rotor
// \[ \vec{a} \cdot R\]
impl BitOr<VGA3DRotor> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DVector, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(
            0.0,
            (self | b.scalar()) + (self | b.bivector()),
            VGA3DBivector::zero(),
            VGA3DTrivector::zero(),
        )
    }
}

// Rotor-Vector
// \[ R \cdot \vec{b}\]
impl BitOr<VGA3DVector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DRotor, b: VGA3DVector) -> VGA3DMultivector {
        VGA3DMultivector::new(
            0.0,
            (self.scalar() | b) + (self.bivector() | b),
            VGA3DBivector::zero(),
            VGA3DTrivector::zero(),
        )
    }
}

// Bivector-Trivector
// \[ \overset\Rightarrow{a} \cdot \overset\Rrightarrow{b}\]
impl BitOr<VGA3DTrivector> for VGA3DBivector {
    type Output = VGA3DVector;
    fn bitor(self: VGA3DBivector, b: VGA3DTrivector) -> VGA3DVector {
        let e1 = -self.e23() * b.e123();
        let e2 = -self.e31() * b.e123();
        let e3 = -self.e12() * b.e123();
        VGA3DVector::new(e1, e2, e3)
    }
}

// Trivector-Bivector
// \[ \overset\Rrightarrow{a} \cdot \overset\Rightarrow{b}\]
impl BitOr<VGA3DBivector> for VGA3DTrivector {
    type Output = VGA3DVector;
    fn bitor(self: VGA3DTrivector, b: VGA3DBivector) -> VGA3DVector {
        let e1 = self.e123() * -b.e23();
        let e2 = self.e123() * -b.e31();
        let e3 = self.e123() * -b.e12();
        VGA3DVector::new(e1, e2, e3)
    }
}

// Bivector-Multivector
// \[ \overset\Rightarrow{a} \cdot B\]
impl BitOr<VGA3DMultivector> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DBivector, b: VGA3DMultivector) -> VGA3DMultivector {
        let scalar = self | b.bivector();
        let vector = (self | b.vector()) + (self | b.trivector());
        let bivector = self | b.scalar();
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Multivector-Bivector
// \[ A \cdot \overset\Rightarrow{b}\]
impl BitOr<VGA3DBivector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DMultivector, b: VGA3DBivector) -> VGA3DMultivector {
        let scalar = self.bivector() | b;
        let vector = (self.vector() | b) + (self.trivector() | b);
        let bivector = self.scalar() | b;
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Bivector-Rotor
// \[ \overset\Rightarrow{a} \cdot R\]
impl BitOr<VGA3DRotor> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DBivector, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = self | b.bivector();
        let vector = VGA3DVector::zero();
        let bivector = self | b.scalar();
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Rotor-Bivector
// \[ R \cdot \overset\Rightarrow{b}\]
impl BitOr<VGA3DBivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DRotor, b: VGA3DBivector) -> VGA3DMultivector {
        let scalar = self.bivector() | b;
        let vector = VGA3DVector::zero();
        let bivector = self.scalar() | b;
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Trivector-Multivector
// \[ \overset\Rrightarrow{a} \cdot B\]
impl BitOr<VGA3DMultivector> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DTrivector, b: VGA3DMultivector) -> VGA3DMultivector {
        let scalar = self | b.trivector();
        let vector = self | b.bivector();
        let bivector = self | b.vector();
        let trivector = self | b.scalar();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Multivector-Trivector
// \[ A \cdot \overset\Rrightarrow{b}\]
impl BitOr<VGA3DTrivector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DMultivector, b: VGA3DTrivector) -> VGA3DMultivector {
        let scalar = self.trivector() | b;
        let vector = self.bivector() | b;
        let bivector = self.vector() | b;
        let trivector = self.scalar() | b;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Trivector-Rotor
// \[ \overset\Rrightarrow{a} \cdot R\]
impl BitOr<VGA3DRotor> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DTrivector, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = 0.0;
        let vector = self | b.bivector();
        let bivector = VGA3DBivector::zero();
        let trivector = self | b.scalar();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Rotor-Trivector
// \[ R \cdot \overset\Rrightarrow{b}\]
impl BitOr<VGA3DTrivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DRotor, b: VGA3DTrivector) -> VGA3DMultivector {
        let scalar = 0.0;
        let vector = self.bivector() | b;
        let bivector = VGA3DBivector::zero();
        let trivector = self.scalar() | b;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Multivector-Rotor
// \[ A \cdot R\]
impl BitOr<VGA3DRotor> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DMultivector, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = (self.scalar() * b.scalar()) + (self.bivector() | b.bivector());
        let vector = (self.vector() | b.scalar())
            + (self.vector() | b.bivector())
            + (self.trivector() | b.bivector());
        let bivector = (self.bivector() | b.scalar()) + (self.scalar() | b.bivector());
        let trivector = self.trivector() | b.scalar();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Rotor-Multivector
// \[ R \cdot B\]
impl BitOr<VGA3DMultivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DRotor, b: VGA3DMultivector) -> VGA3DMultivector {
        let scalar = (self.scalar() * b.scalar()) + (self.bivector() | b.bivector());
        let vector = (self.scalar() ^ b.vector())
            + (self.bivector() | b.vector())
            + (self.bivector() | b.trivector());
        let bivector = (self.scalar() | b.bivector()) + (self.bivector() | b.scalar());
        let trivector = self.scalar() | b.trivector();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

#[cfg(test)]
mod inner {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn bivector_vector_inner() {
        // 3e12+5e31+4e23
        let bivector = VGA3DBivector::new(3.0, 5.0, 4.0);
        // 2e1+e2+6e3
        let vector = VGA3DVector::new(2.0, 1.0, 6.0);
        let vector_res = bivector | vector;
        // -27e1​+18e2​+6e3
        assert_relative_eq!(vector_res.e1(), -27.0, max_relative = 0.000001);
        assert_relative_eq!(vector_res.e2(), 18.0, max_relative = 0.000001);
        assert_relative_eq!(vector_res.e3(), 6.0, max_relative = 0.000001);
    }

    #[test]
    fn vector_bivector_inner() {
        // 2e1+e2+6e3
        let vector = VGA3DVector::new(2.0, 1.0, 6.0);
        // 3e12+5e31+4e23
        let bivector = VGA3DBivector::new(3.0, 5.0, 4.0);
        let vector_res = vector | bivector;
        // 27e1​-18e2​-6e3
        assert_relative_eq!(vector_res.e1(), 27.0, max_relative = 0.000001);
        assert_relative_eq!(vector_res.e2(), -18.0, max_relative = 0.000001);
        assert_relative_eq!(vector_res.e3(), -6.0, max_relative = 0.000001);
    }
}
