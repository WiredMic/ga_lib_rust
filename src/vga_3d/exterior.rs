#![allow(unused_imports)]
#![allow(dead_code)]

use super::{
    bivector::{self, VGA3DBivector},
    multivector::VGA3DMultivector,
    rotor::VGA3DRotor,
    trivector::{self, VGA3DTrivector},
    vector::VGA3DVector,
};

use core::ops::BitXor;

// // Scalar-Scalar
// // \[ a\wedge b\]
// impl BitXor for f32 {
//     type Output = f32;
//     fn bitxor(self: f32, b: VGA3DVector) -> f32 {
//         self * b
//     }
// }

// Scalar-Vector
// \[ a\wedge \vec{b}\]
impl BitXor<VGA3DVector> for f32 {
    type Output = VGA3DVector;
    fn bitxor(self: f32, b: VGA3DVector) -> VGA3DVector {
        self * b
    }
}

// Vector-Scalar
// \[ \vec{a}\wedge b\]
impl BitXor<f32> for VGA3DVector {
    type Output = VGA3DVector;
    fn bitxor(self: VGA3DVector, b: f32) -> VGA3DVector {
        self * b
    }
}

// Scalar-Bivector
// \[ s\wedge \overset\Rightarrow{b}\]
impl BitXor<VGA3DBivector> for f32 {
    type Output = VGA3DBivector;
    fn bitxor(self: f32, b: VGA3DBivector) -> VGA3DBivector {
        self * b
    }
}

// Bivector-Scalar
// \[ \overset\Rightarrow{a}\wedge b\]
impl BitXor<f32> for VGA3DBivector {
    type Output = VGA3DBivector;
    fn bitxor(self: VGA3DBivector, b: f32) -> VGA3DBivector {
        self * b
    }
}

// Scalar-Trivector
// \[ s\wedge \overset\Rrightarrow{b}\]
impl BitXor<VGA3DTrivector> for f32 {
    type Output = VGA3DTrivector;
    fn bitxor(self: f32, b: VGA3DTrivector) -> VGA3DTrivector {
        self * b
    }
}

// Trivector-Scalar
// \[ \overset\Rrightarrow{a}\wedge b\]
impl BitXor<f32> for VGA3DTrivector {
    type Output = VGA3DTrivector;
    fn bitxor(self: VGA3DTrivector, b: f32) -> VGA3DTrivector {
        self * b
    }
}

// Scalar-Multivector
// \[ s\wedge B\]
impl BitXor<VGA3DMultivector> for f32 {
    type Output = VGA3DMultivector;
    fn bitxor(self: f32, b: VGA3DMultivector) -> VGA3DMultivector {
        self * b
    }
}

// Multivector-Scalar
// \[ B\wedge b\]
impl BitXor<f32> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DMultivector, b: f32) -> VGA3DMultivector {
        self * b
    }
}

// Scalar-Rotor
// \[ s\wedge R\]
impl BitXor<VGA3DRotor> for f32 {
    type Output = VGA3DMultivector;
    fn bitxor(self: f32, b: VGA3DRotor) -> VGA3DMultivector {
        self * b
    }
}

//Rotor-Scalar
// \[ R\wedge b\]
impl BitXor<f32> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DRotor, b: f32) -> VGA3DMultivector {
        self * b
    }
}

// Vector-Bivector
// \[ \vec{a}\wedge \overset\Rightarrow{b}\]
impl BitXor<VGA3DBivector> for VGA3DVector {
    type Output = VGA3DTrivector;
    fn bitxor(self: VGA3DVector, b: VGA3DBivector) -> VGA3DTrivector {
        VGA3DTrivector::new(self.e1() * b.e23() + self.e2() * b.e31() + self.e3() * b.e12())
    }
}

// Bivector-Vector
// \[ \overset\Rightarrow{b}\wedge \vec{b}\]
impl BitXor<VGA3DVector> for VGA3DBivector {
    type Output = VGA3DTrivector;
    fn bitxor(self: VGA3DBivector, b: VGA3DVector) -> VGA3DTrivector {
        VGA3DTrivector::new(self.e23() * b.e1() + self.e31() * b.e2() + self.e12() * b.e3())
    }
}

// Vector-Trivector
// \[ \vec{a}\wedge \overset\Rrightarrow{b}\]
impl BitXor<VGA3DTrivector> for VGA3DVector {
    type Output = f32;
    fn bitxor(self: VGA3DVector, _b: VGA3DTrivector) -> f32 {
        0.0
    }
}

// Trivector-Vector
// \[ \overset\Rrightarrow{a}\wedge \vec{b}\]
impl BitXor<VGA3DVector> for VGA3DTrivector {
    type Output = f32;
    fn bitxor(self: VGA3DTrivector, _b: VGA3DVector) -> f32 {
        0.0
    }
}

// Vector-Multivector
// \[ \vec{a}\wedge B\]
impl BitXor<VGA3DMultivector> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DVector, b: VGA3DMultivector) -> VGA3DMultivector {
        let scalar = self ^ b.trivector();
        let vector = self ^ b.scalar();
        let bivector = self ^ b.vector();
        let trivector = self ^ b.bivector();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Multivector-Vector
// \[ A\wedge \vec{b}\]
impl BitXor<VGA3DVector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DMultivector, b: VGA3DVector) -> VGA3DMultivector {
        let scalar = self.trivector() ^ b;
        let vector = self.scalar() ^ b;
        let bivector = self.vector() ^ b;
        let trivector = self.bivector() ^ b;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Vector-Rotor
// \[ \vec{a} \wedge R\]
impl BitXor<VGA3DRotor> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DVector, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(
            0.0,
            self ^ b.scalar(),
            VGA3DBivector::zero(),
            self ^ b.bivector(),
        )
    }
}

// Rotor-Vector
// \[ R\wedge \vec{b}\]
impl BitXor<VGA3DVector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DRotor, b: VGA3DVector) -> VGA3DMultivector {
        VGA3DMultivector::new(
            0.0,
            self.scalar() ^ b,
            VGA3DBivector::zero(),
            self.bivector() ^ b,
        )
    }
}

// Bivector-Trivector
// \[ \overset\Rightarrow{a}\wedge \overset\Rrightarrow{b}\]
impl BitXor<VGA3DTrivector> for VGA3DBivector {
    type Output = f32;
    fn bitxor(self: VGA3DBivector, _b: VGA3DTrivector) -> f32 {
        0.0
    }
}

// Trivector-Bivector
// \[ \overset\Rrightarrow{a}\wedge \overset\Rightarrow{b}\]
impl BitXor<VGA3DBivector> for VGA3DTrivector {
    type Output = f32;
    fn bitxor(self: VGA3DTrivector, _b: VGA3DBivector) -> f32 {
        0.0
    }
}

// Bivector-Multivector
// \[ \overset\Rightarrow{a}\wedge B\]
impl BitXor<VGA3DMultivector> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DBivector, b: VGA3DMultivector) -> VGA3DMultivector {
        let scalar = (self ^ b.bivector()) + (self ^ b.trivector());
        let vector = VGA3DVector::zero();
        let bivector = self ^ b.scalar();
        let trivector = self ^ b.vector();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Multivector-Bivector
// \[ A\wedge \overset\Rightarrow{b}\]
impl BitXor<VGA3DBivector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DMultivector, b: VGA3DBivector) -> VGA3DMultivector {
        let scalar = (self.bivector() ^ b) + (self.trivector() ^ b);
        let vector = VGA3DVector::zero();
        let bivector = self.scalar() ^ b;
        let trivector = self.vector() ^ b;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Bivector-Rotor
// \[ \overset\Rightarrow{a}\wedge R\]
impl BitXor<VGA3DRotor> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DBivector, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = self ^ b.bivector();
        let vector = VGA3DVector::zero();
        let bivector = self ^ b.scalar();
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Rotor-Bivector
// \[ R\wedge \overset\Rightarrow{b}\]
impl BitXor<VGA3DBivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DRotor, b: VGA3DBivector) -> VGA3DMultivector {
        let scalar = self.bivector() ^ b;
        let vector = VGA3DVector::zero();
        let bivector = self.scalar() ^ b;
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Trivector-Multivector
// \[ \overset\Rrightarrow{a}\wedge B\]
impl BitXor<VGA3DMultivector> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DTrivector, b: VGA3DMultivector) -> VGA3DMultivector {
        let scalar = (self ^ b.vector()) + (self ^ b.bivector()) + (self ^ b.trivector());
        let vector = VGA3DVector::zero();
        let bivector = VGA3DBivector::zero();
        let trivector = self ^ b.scalar();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Multivector-Trivector
// \[ A\wedge \overset\Rrightarrow{b}\]
impl BitXor<VGA3DTrivector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DMultivector, b: VGA3DTrivector) -> VGA3DMultivector {
        let scalar = (self.vector() ^ b) + (self.bivector() ^ b) + (self.trivector() ^ b);
        let vector = VGA3DVector::zero();
        let bivector = VGA3DBivector::zero();
        let trivector = self.scalar() ^ b;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Trivector-Rotor
// \[ \overset\Rrightarrow{a}\wedge R\]
impl BitXor<VGA3DRotor> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DTrivector, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = self ^ b.bivector();
        let vector = VGA3DVector::zero();
        let bivector = VGA3DBivector::zero();
        let trivector = self ^ b.scalar();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Rotor-Trivector
// \[ R\wedge \overset\Rrightarrow{b}\]
impl BitXor<VGA3DTrivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DRotor, b: VGA3DTrivector) -> VGA3DMultivector {
        let scalar = self.bivector() ^ b;
        let vector = VGA3DVector::zero();
        let bivector = VGA3DBivector::zero();
        let trivector = self.scalar() ^ b;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Multivector-Rotor
// \[ A\wedge R\]
impl BitXor<VGA3DRotor> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DMultivector, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = (self.scalar() * b.scalar())
            + (self.bivector() ^ b.bivector())
            + (self.trivector() ^ b.bivector());
        let vector = self.vector() ^ b.scalar();
        let bivector = (self.bivector() ^ b.scalar()) + (self.scalar() ^ b.bivector());
        let trivector = (self.trivector() ^ b.scalar()) + (self.vector() ^ b.bivector());
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

// Rotor-Multivector
// \[ R\wedge B\]
impl BitXor<VGA3DMultivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DRotor, b: VGA3DMultivector) -> VGA3DMultivector {
        let scalar = (self.scalar() * b.scalar())
            + (self.bivector() ^ b.bivector())
            + (self.bivector() ^ b.trivector());
        let vector = self.scalar() ^ b.vector();
        let bivector = (self.scalar() ^ b.bivector()) + (self.bivector() ^ b.scalar());
        let trivector = (self.scalar() ^ b.trivector()) + (self.bivector() ^ b.vector());
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}

#[cfg(test)]
mod exterior {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn bivector_vector_exterior() {
        // 3e1+5e2+4e3
        let bivector = VGA3DBivector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let vector = VGA3DVector::new(2.0, 1.0, 6.0);
        // 31e123​
        let trivector = bivector ^ vector;
        assert_relative_eq!(trivector.e123(), 31.0, max_relative = 0.000001);
    }
}
