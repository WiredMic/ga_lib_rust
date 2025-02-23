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
use forward_ref::forward_ref_binop;

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
forward_ref_binop!(impl BitXor, bitxor for f32, VGA3DVector);

// Vector-Scalar
// \[ \vec{a}\wedge b\]
impl BitXor<f32> for VGA3DVector {
    type Output = VGA3DVector;
    fn bitxor(self: VGA3DVector, b: f32) -> VGA3DVector {
        self * b
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DVector, f32);

// Scalar-Bivector
// \[ s\wedge \overset\Rightarrow{b}\]
impl BitXor<VGA3DBivector> for f32 {
    type Output = VGA3DBivector;
    fn bitxor(self: f32, b: VGA3DBivector) -> VGA3DBivector {
        self * b
    }
}
forward_ref_binop!(impl BitXor, bitxor for f32, VGA3DBivector);

// Bivector-Scalar
// \[ \overset\Rightarrow{a}\wedge b\]
impl BitXor<f32> for VGA3DBivector {
    type Output = VGA3DBivector;
    fn bitxor(self: VGA3DBivector, b: f32) -> VGA3DBivector {
        self * b
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DBivector, f32);

// Scalar-Trivector
// \[ s\wedge \overset\Rrightarrow{b}\]
impl BitXor<VGA3DTrivector> for f32 {
    type Output = VGA3DTrivector;
    fn bitxor(self: f32, b: VGA3DTrivector) -> VGA3DTrivector {
        self * b
    }
}
forward_ref_binop!(impl BitXor, bitxor for f32, VGA3DTrivector);

// Trivector-Scalar
// \[ \overset\Rrightarrow{a}\wedge b\]
impl BitXor<f32> for VGA3DTrivector {
    type Output = VGA3DTrivector;
    fn bitxor(self: VGA3DTrivector, b: f32) -> VGA3DTrivector {
        self * b
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DTrivector, f32);

// Scalar-Multivector
// \[ s\wedge B\]
impl BitXor<VGA3DMultivector> for f32 {
    type Output = VGA3DMultivector;
    fn bitxor(self: f32, b: VGA3DMultivector) -> VGA3DMultivector {
        self * b
    }
}
forward_ref_binop!(impl BitXor, bitxor for f32, VGA3DMultivector);

// Multivector-Scalar
// \[ B\wedge b\]
impl BitXor<f32> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DMultivector, b: f32) -> VGA3DMultivector {
        self * b
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DMultivector, f32);

// Scalar-Rotor
// \[ s\wedge R\]
impl BitXor<VGA3DRotor> for f32 {
    type Output = VGA3DMultivector;
    fn bitxor(self: f32, b: VGA3DRotor) -> VGA3DMultivector {
        self * b
    }
}
forward_ref_binop!(impl BitXor, bitxor for f32, VGA3DRotor);

//Rotor-Scalar
// \[ R\wedge b\]
impl BitXor<f32> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DRotor, b: f32) -> VGA3DMultivector {
        self * b
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DRotor, f32);

// Vector-Vector
impl BitXor for VGA3DVector {
    type Output = VGA3DBivector;
    fn bitxor(self: VGA3DVector, b: VGA3DVector) -> VGA3DBivector {
        let e12 = self.e1() * b.e2() - self.e2() * b.e1();
        let e31 = self.e3() * b.e1() - self.e1() * b.e3();
        let e23 = self.e2() * b.e3() - self.e3() * b.e2();
        VGA3DBivector::new(e12, e31, e23)
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DVector, VGA3DVector);

// Vector-Bivector
// \[ \vec{a}\wedge \overset\Rightarrow{b}\]
impl BitXor<VGA3DBivector> for VGA3DVector {
    type Output = VGA3DTrivector;
    fn bitxor(self: VGA3DVector, b: VGA3DBivector) -> VGA3DTrivector {
        VGA3DTrivector::new(self.e1() * b.e23() + self.e2() * b.e31() + self.e3() * b.e12())
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DVector, VGA3DBivector);

// Bivector-Vector
// \[ \overset\Rightarrow{b}\wedge \vec{b}\]
impl BitXor<VGA3DVector> for VGA3DBivector {
    type Output = VGA3DTrivector;
    fn bitxor(self: VGA3DBivector, b: VGA3DVector) -> VGA3DTrivector {
        VGA3DTrivector::new(self.e23() * b.e1() + self.e31() * b.e2() + self.e12() * b.e3())
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DBivector, VGA3DVector);

// Vector-Trivector
// \[ \vec{a}\wedge \overset\Rrightarrow{b}\]
impl BitXor<VGA3DTrivector> for VGA3DVector {
    type Output = f32;
    fn bitxor(self: VGA3DVector, _b: VGA3DTrivector) -> f32 {
        0.0
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DVector, VGA3DTrivector);

// Trivector-Vector
// \[ \overset\Rrightarrow{a}\wedge \vec{b}\]
impl BitXor<VGA3DVector> for VGA3DTrivector {
    type Output = f32;
    fn bitxor(self: VGA3DTrivector, _b: VGA3DVector) -> f32 {
        0.0
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DTrivector, VGA3DVector);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DVector, VGA3DMultivector);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DMultivector, VGA3DVector);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DVector, VGA3DRotor);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DRotor, VGA3DVector);

// Bivector-Bivector
// \[ \overset\Rightarrow{a} \wedge \overset\Rightarrow{b} = \left <\overset\Rightarrow{a} \overset\Rightarrow{b} \right>_4 \]
// There is no object of grade 4 in 3D VGA
impl BitXor for VGA3DBivector {
    type Output = f32;

    fn bitxor(self: VGA3DBivector, _b: VGA3DBivector) -> f32 {
        0.0
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
forward_ref_binop!(impl BitXor, bitxor for VGA3DBivector, VGA3DTrivector);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a}\wedge \overset\Rightarrow{b}\]
impl BitXor<VGA3DBivector> for VGA3DTrivector {
    type Output = f32;
    fn bitxor(self: VGA3DTrivector, _b: VGA3DBivector) -> f32 {
        0.0
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DTrivector, VGA3DBivector);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DBivector, VGA3DMultivector);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DMultivector, VGA3DBivector);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DBivector, VGA3DRotor);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DRotor, VGA3DBivector);

// Trivector-Trivector
// \[ \overset\Rrightarrow{a} \wedge \overset\Rrightarrow{b} = \left <\overset\Rrightarrow{a} \overset\Rrightarrow{b} \right>_4 \]
// There is no object of grade 6 in 3D VGA
impl BitXor for VGA3DTrivector {
    type Output = f32;
    fn bitxor(self: VGA3DTrivector, _b: VGA3DTrivector) -> f32 {
        0.0
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DTrivector, VGA3DTrivector);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DTrivector, VGA3DMultivector);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DMultivector, VGA3DTrivector);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DTrivector, VGA3DRotor);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DRotor, VGA3DTrivector);

// Multivector-Multivector
// \[ A \wedge B = \left <A B \right>_{a+b} \]
impl BitXor for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DMultivector, b: VGA3DMultivector) -> VGA3DMultivector {
        (self ^ b.scalar()) + (self ^ b.vector()) + (self ^ b.bivector()) + (self ^ b.trivector())
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
forward_ref_binop!(impl BitXor, bitxor for VGA3DMultivector, VGA3DRotor);

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
forward_ref_binop!(impl BitXor, bitxor for VGA3DRotor, VGA3DMultivector);

// Rotor-Rotor
// \[ R_1 \wedge R_2\]
impl BitXor for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitxor(self: VGA3DRotor, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = (self.scalar() * b.scalar()) + (self.bivector() ^ b.bivector());
        let vector = VGA3DVector::zero();
        let bivector = (self.scalar() ^ b.bivector()) + (self.bivector() ^ b.scalar());
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitXor, bitxor for VGA3DRotor, VGA3DRotor);

#[cfg(test)]
mod exterior {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn vector_vector_exterior() {
        // 3e1+5e2+4e3
        let vector1 = VGA3DVector::new(3.0, 5.0, 4.0);
        // 2e1+1e2+6e3
        let vector2 = VGA3DVector::new(2.0, 1.0, 6.0);
        let bivector_ref1 = &vector1 ^ &vector2;
        let bivector_ref2 = &vector1 ^ vector2;
        let bivector_ref3 = vector1 ^ &vector2;
        let bivector = vector1 ^ vector2;
        // −7e12​-10e31​+26e23
        assert_relative_eq!(bivector_ref1.e12(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(bivector_ref1.e31(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(bivector_ref1.e23(), 26.0, max_relative = 0.000001);

        assert_relative_eq!(bivector_ref2.e12(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(bivector_ref2.e31(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(bivector_ref2.e23(), 26.0, max_relative = 0.000001);

        assert_relative_eq!(bivector_ref3.e12(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(bivector_ref3.e31(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(bivector_ref3.e23(), 26.0, max_relative = 0.000001);

        assert_relative_eq!(bivector.e12(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(bivector.e31(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(bivector.e23(), 26.0, max_relative = 0.000001);
    }

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

    #[test]
    fn bivector_bivector_exterior() {
        // 3e1+5e2+4e3
        let bivector1 = VGA3DBivector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let bivector2 = VGA3DBivector::new(2.0, 1.0, 6.0);
        // 31e123​
        assert_relative_eq!(bivector1 ^ bivector2, 0.0, max_relative = 0.000001);
    }

    #[test]
    fn trivec_trivec_exterior() {
        let trivector1 = VGA3DTrivector::new(3.0);
        let trivector2 = VGA3DTrivector::new(6.0);
        let res = trivector1 ^ trivector2;
        assert_relative_eq!(res, 0.0, max_relative = 0.000001);
    }

    #[test]
    fn mvec_mvec_exterior() {
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let mvec1 = VGA3DMultivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        // 5.0+ 8.0e1+ 7.0e2+ 3.0e3+ 2.0e12+ 8.0e31+ 2.0e23+ 1.0e123
        let mvec2 = VGA3DMultivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        let mvec_res = mvec1 ^ mvec2;
        // 30+93e1​+77e2​+38e3​+54e12​−73e13​+45e23​+236e123
        assert_relative_eq!(mvec_res.scalar(), 30.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), 93.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), 77.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), 38.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 54.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 73.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 45.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 236.0, max_relative = 0.000001);
    }

    #[test]
    fn negetive_mvec_mvec_exterior() {
        // -4.0-1.0e1 -3.0e2 -2.0e3 -9.0e12 -6.0e31 -3.0e23 -10.0e123
        let mvec1 =
            VGA3DMultivector::new_components(-4.0, -1.0, -3.0, -2.0, -9.0, -6.0, -3.0, -10.0);
        // -4.0 -2.0e1 -4.0e2 -9.0e3 -2.0e12 -1.0e31 -7.0e23 -1.0e123
        let mvec2 =
            VGA3DMultivector::new_components(-4.0, -2.0, -4.0, -9.0, -2.0, -1.0, -7.0, -1.0);
        let mvec_res = mvec1 ^ mvec2;
        // 16+12e1​+28e2​+44e3​+42e12​−23e13​+59e23​+169e123
        assert_relative_eq!(mvec_res.scalar(), 16.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), 12.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), 28.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), 44.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 42.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 23.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 59.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 169.0, max_relative = 0.000001);
    }
}
