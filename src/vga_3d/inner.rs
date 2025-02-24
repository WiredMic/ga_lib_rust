#![allow(unused_imports)]
#![allow(dead_code)]

use super::{
    bivector::{self, VGA3DBivector},
    multivector::VGA3DMultivector,
    rotor::VGA3DRotor,
    trivector::{self, VGA3DTrivector},
    vector::VGA3DVector,
};

use crate::forward_ref_binop;

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
forward_ref_binop!(impl BitOr, bitor for f32, VGA3DVector);

// Vector-Scalar
// \[ \vec{a} \cdot b\]
impl BitOr<f32> for VGA3DVector {
    type Output = VGA3DVector;
    fn bitor(self: VGA3DVector, b: f32) -> VGA3DVector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DVector, f32);

// Scalar-Bivector
// \[ s \cdot \overset\Rightarrow{b}\]
impl BitOr<VGA3DBivector> for f32 {
    type Output = VGA3DBivector;
    fn bitor(self: f32, b: VGA3DBivector) -> VGA3DBivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for f32, VGA3DBivector);

// Bivector-Scalar
// \[ \overset\Rightarrow{a} \cdot b\]
impl BitOr<f32> for VGA3DBivector {
    type Output = VGA3DBivector;
    fn bitor(self: VGA3DBivector, b: f32) -> VGA3DBivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DBivector,f32);

// Scalar-Trivector
// \[ s \cdot \overset\Rrightarrow{b}\]
impl BitOr<VGA3DTrivector> for f32 {
    type Output = VGA3DTrivector;
    fn bitor(self: f32, b: VGA3DTrivector) -> VGA3DTrivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for f32, VGA3DTrivector);

// Trivector-Scalar
// \[ \overset\Rrightarrow{a} \cdot b\]
impl BitOr<f32> for VGA3DTrivector {
    type Output = VGA3DTrivector;
    fn bitor(self: VGA3DTrivector, b: f32) -> VGA3DTrivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DTrivector, f32);

// Scalar-Multivector
// \[ s \cdot B\]
impl BitOr<VGA3DMultivector> for f32 {
    type Output = VGA3DMultivector;
    fn bitor(self: f32, b: VGA3DMultivector) -> VGA3DMultivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for f32, VGA3DMultivector);

// Multivector-Scalar
// \[ B \cdot b\]
impl BitOr<f32> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DMultivector, b: f32) -> VGA3DMultivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DMultivector, f32);

// Scalar-Rotor
// \[ s \cdot R\]
impl BitOr<VGA3DRotor> for f32 {
    type Output = VGA3DMultivector;
    fn bitor(self: f32, b: VGA3DRotor) -> VGA3DMultivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for f32, VGA3DRotor);

//Rotor-Scalar
// \[ R \cdot b\]
impl BitOr<f32> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DRotor, b: f32) -> VGA3DMultivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DRotor, f32);

// Vector-Vector
// \[ \vec{u} \cdot \vec{v} = u_1 \cdot v_1 + u_2 \cdot v_2 + u_3 \cdot v_3 \]
impl BitOr for VGA3DVector {
    type Output = f32;
    fn bitor(self: VGA3DVector, b: VGA3DVector) -> f32 {
        self.e1() * b.e1() + self.e2() * b.e2() + self.e3() * b.e3()
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DVector, VGA3DVector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DVector, VGA3DBivector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DBivector, VGA3DVector);

// Vector-Trivector
// \[ \vec{a} \cdot \overset\Rrightarrow{b}\]
impl BitOr<VGA3DTrivector> for VGA3DVector {
    type Output = VGA3DBivector;
    fn bitor(self: VGA3DVector, b: VGA3DTrivector) -> VGA3DBivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DVector, VGA3DTrivector);

// Trivector-Vector
// \[ \overset\Rrightarrow{a} \cdot \vec{b}\]
impl BitOr<VGA3DVector> for VGA3DTrivector {
    type Output = VGA3DBivector;
    fn bitor(self: VGA3DTrivector, b: VGA3DVector) -> VGA3DBivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DTrivector, VGA3DVector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DVector, VGA3DMultivector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DMultivector, VGA3DVector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DVector, VGA3DRotor);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DRotor, VGA3DVector);

// Bivector-Bivector
// \[ \overset\Rightarrow{a} \cdot \overset\Rightarrow{b} = \left <\overset\Rightarrow{a} \overset\Rightarrow{b} \right>_0 \]
impl BitOr for VGA3DBivector {
    type Output = f32;
    fn bitor(self: VGA3DBivector, b: VGA3DBivector) -> f32 {
        -self.e12() * b.e12() - self.e31() * b.e31() - self.e23() * b.e23()
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DBivector, VGA3DBivector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DBivector, VGA3DTrivector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DTrivector, VGA3DBivector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DBivector, VGA3DMultivector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DMultivector, VGA3DBivector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DBivector, VGA3DRotor);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DRotor, VGA3DBivector);

// Trivector-Trivector
// In 3D there the geometric product of two trivectors is there inner product
// \[ \overset\Rrightarrow{a} \cdot \overset\Rrightarrow{b} = \left <\overset\Rrightarrow{a} \overset\Rrightarrow{b} \right>_0 = \overset\Rrightarrow{a} \overset\Rrightarrow{b} \]
impl BitOr for VGA3DTrivector {
    type Output = f32;
    fn bitor(self: VGA3DTrivector, b: VGA3DTrivector) -> f32 {
        self * b
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
forward_ref_binop!(impl BitOr, bitor for VGA3DTrivector, VGA3DMultivector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DMultivector, VGA3DTrivector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DTrivector, VGA3DRotor);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DRotor, VGA3DTrivector);

// Multivector-Multivector
// // \[ A \cdot B = \left <A B \right>_{|a-b|} \]
impl BitOr for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DMultivector, b: VGA3DMultivector) -> VGA3DMultivector {
        (self | b.scalar()) + (self | b.vector()) + (self | b.bivector()) + (self | b.trivector())
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DMultivector, VGA3DMultivector);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DMultivector, VGA3DRotor);

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
forward_ref_binop!(impl BitOr, bitor for VGA3DRotor, VGA3DMultivector);

// Inner Product
// \[ R_1 \cdot R_2\]
impl BitOr for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn bitor(self: VGA3DRotor, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = (self.scalar() * b.scalar()) + (self.bivector() | b.bivector());
        let vector = VGA3DVector::zero();
        let bivector = (self.scalar() | b.bivector()) + (self.bivector() | b.scalar());
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DRotor, VGA3DRotor);

#[cfg(test)]
mod inner {
    use super::*;
    use approx::assert_relative_eq;

    fn vector_vector_dot() {
        // 3e1+5e2+4e3
        let vector1 = VGA3DVector::new(3.0, 5.0, 4.0);
        // 2e1+1e2+6e3
        let vector2 = VGA3DVector::new(2.0, 1.0, 6.0);
        let scalar = vector1 | vector2;
        assert_relative_eq!(scalar, 35.0, max_relative = 0.000001);
    }

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

    #[test]
    fn bivector_bivector_inner() {
        // 3e12+5e31+4e23
        let bivector1 = VGA3DBivector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let bivector2 = VGA3DBivector::new(2.0, 1.0, 6.0);
        let scalar = bivector1 | bivector2;
        assert_relative_eq!(scalar, -35.0, max_relative = 0.000001);
    }

    #[test]
    fn trivec_trivec_inner() {
        let trivector1 = VGA3DTrivector::new(3.0);
        let trivector2 = VGA3DTrivector::new(6.0);
        let res = trivector1 | trivector2;
        assert_relative_eq!(res, -18.0, max_relative = 0.000001);
    }

    #[test]
    fn mvec_mvec_inner() {
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let mvec1 = VGA3DMultivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        // 5.0+ 8.0e1+ 7.0e2+ 3.0e3+ 2.0e12+ 8.0e31+ 2.0e23+ 1.0e123
        let mvec2 = VGA3DMultivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        let mvec_res = mvec1 | mvec2;
        // 94+126e1​−5e2​−65e3​+72e12​−124e13​+117e23​+41e123
        assert_relative_eq!(mvec_res.scalar(), 94.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), 126.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), -5.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), -65.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 72.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 124.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 117.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 41.0, max_relative = 0.000001);
    }

    #[test]
    fn negetive_mvec_mvec_inner() {
        // let mvec1 = GaMultivector::new_mvec(-6.0, -8.0, -4.0, -1.0, -6.0, -4.0, -8.0, -5.0);
        let mvec1 =
            VGA3DMultivector::new_components(-4.0, -1.0, -3.0, -2.0, -9.0, -6.0, -3.0, -10.0);
        let mvec2 =
            VGA3DMultivector::new_components(-4.0, -2.0, -4.0, -9.0, -2.0, -1.0, -7.0, -1.0);
        let mvec_res = mvec1 | mvec2;
        // −7−83e1​+9e2​+35e3​+136e12​−71e13​+61e23​+44e123
        assert_relative_eq!(mvec_res.scalar(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), -83.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), 9.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), 35.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 136.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 71.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 61.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 44.0, max_relative = 0.000001);
    }
}
