#![allow(unused_imports)]
#![allow(dead_code)]

use super::{
    bivector::{self, VGA3DBivector},
    multivector::VGA3DMultivector,
    rotor::VGA3DRotor,
    trivector::{self, VGA3DTrivector},
    vector::VGA3DVector,
};

use core::ops::{BitAnd, BitOr, Div, Mul};
use forward_ref::forward_ref_binop;

// Geometric Product
// Scalar-Vector
impl Mul<VGA3DVector> for f32 {
    type Output = VGA3DVector;
    fn mul(self, b: VGA3DVector) -> VGA3DVector {
        let e1 = b.e1() * self;
        let e2 = b.e2() * self;
        let e3 = b.e3() * self;
        VGA3DVector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl Mul, mul for f32, VGA3DVector);

// Vector-Scalar
impl Mul<f32> for VGA3DVector {
    type Output = VGA3DVector;
    fn mul(self, b: f32) -> VGA3DVector {
        let e1 = self.e1() * b;
        let e2 = self.e2() * b;
        let e3 = self.e3() * b;
        VGA3DVector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DVector,f32);

//Scalar-Bivector
impl Mul<VGA3DBivector> for f32 {
    type Output = VGA3DBivector;
    fn mul(self, b: VGA3DBivector) -> VGA3DBivector {
        let e12 = self * b.e12();
        let e31 = self * b.e31();
        let e23 = self * b.e23();
        VGA3DBivector::new(e12, e31, e23)
    }
}
forward_ref_binop!(impl Mul, mul for f32, VGA3DBivector);

// Bivector-Scalar
impl Mul<f32> for VGA3DBivector {
    type Output = VGA3DBivector;
    fn mul(self, b: f32) -> VGA3DBivector {
        let e12 = self.e12() * b;
        let e31 = self.e31() * b;
        let e23 = self.e23() * b;
        VGA3DBivector::new(e12, e31, e23)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DBivector,f32);

// Scalar-Trivector
impl Mul<VGA3DTrivector> for f32 {
    type Output = VGA3DTrivector;
    fn mul(self, b: VGA3DTrivector) -> VGA3DTrivector {
        let e123 = self * b.e123();
        VGA3DTrivector::new(e123)
    }
}

// Trivector-Scalar
impl Mul<f32> for VGA3DTrivector {
    type Output = VGA3DTrivector;
    fn mul(self, b: f32) -> VGA3DTrivector {
        let e123 = self.e123() * b;
        VGA3DTrivector::new(e123)
    }
}
forward_ref_binop!(impl Mul, mul for f32, VGA3DTrivector);

// Scalar-Multivector
impl Mul<VGA3DMultivector> for f32 {
    type Output = VGA3DMultivector;
    fn mul(self, b: VGA3DMultivector) -> VGA3DMultivector {
        let scalar = self * b.scalar();
        let vector = self * b.vector();
        let bivector = self * b.bivector();
        let trivector = self * b.trivector();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DTrivector,f32);

// Multivector-Scalar
impl Mul<f32> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn mul(self, b: f32) -> VGA3DMultivector {
        let scalar = self.scalar() * b;
        let vector = self.vector() * b;
        let bivector = self.bivector() * b;
        let trivector = self.trivector() * b;
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Mul, mul for f32, VGA3DMultivector);

// Scalar-Rotor
// \[ s R\]
impl Mul<VGA3DRotor> for f32 {
    type Output = VGA3DMultivector;
    fn mul(self: f32, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(
            self * b.scalar(),
            VGA3DVector::zero(),
            self * b.bivector(),
            VGA3DTrivector::zero(),
        )
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DRotor, f32);

//Rotor-Scalar
// \[ R b\]
impl Mul<f32> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DRotor, b: f32) -> VGA3DMultivector {
        VGA3DMultivector::new(
            self.scalar() * b,
            VGA3DVector::zero(),
            self.bivector() * b,
            VGA3DTrivector::zero(),
        )
    }
}
forward_ref_binop!(impl Mul, mul for f32, VGA3DRotor);

// Vector-Vector
// \[ \vec{a} \vec{b} = \vec{a} \cdot \vec{b} +   \vec{a} \wedge \vec{b}\]
impl Mul for VGA3DVector {
    type Output = VGA3DMultivector;

    fn mul(self: VGA3DVector, b: VGA3DVector) -> VGA3DMultivector {
        let scalar = self | b;
        let vector = VGA3DVector::zero();
        let bivector = self ^ b;
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DVector, VGA3DVector);

// Vector-Bivector
// \[ \vec{a}\overset\Rightarrow{b}\]
impl Mul<VGA3DBivector> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DVector, b: VGA3DBivector) -> VGA3DMultivector {
        let e1 = -self.e2() * b.e12() + self.e3() * b.e31();
        let e2 = self.e1() * b.e12() - self.e3() * b.e23();
        let e3 = -self.e1() * b.e31() + self.e2() * b.e23();
        let vector = VGA3DVector::new(e1, e2, e3);

        let e123 = self.e3() * b.e12() + self.e2() * b.e31() + self.e1() * b.e23();
        let trivector = VGA3DTrivector::new(e123);
        VGA3DMultivector::new(0.0, vector, VGA3DBivector::zero(), trivector)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DBivector, VGA3DVector);

// Bivector-Vector
// \[ \overset\Rightarrow{a}\vec{b}\]
impl Mul<VGA3DVector> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DBivector, b: VGA3DVector) -> VGA3DMultivector {
        let e1 = self.e12() * b.e2() - self.e31() * b.e3();
        let e2 = -self.e12() * b.e1() + self.e23() * b.e3();
        let e3 = self.e31() * b.e1() - self.e23() * b.e2();
        let vector = VGA3DVector::new(e1, e2, e3);

        let e123 = self.e23() * b.e1() + self.e31() * b.e2() + self.e12() * b.e3();
        let trivector = VGA3DTrivector::new(e123);
        VGA3DMultivector::new(0.0, vector, VGA3DBivector::zero(), trivector)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DVector, VGA3DBivector);

// Vector-Trivector
// \[ \vec{a}\overset\Rrightarrow{b}\]
impl Mul<VGA3DTrivector> for VGA3DVector {
    type Output = VGA3DBivector;
    fn mul(self: VGA3DVector, b: VGA3DTrivector) -> VGA3DBivector {
        let e12 = self.e3() * b.e123();
        let e31 = self.e2() * b.e123();
        let e23 = self.e1() * b.e123();
        VGA3DBivector::new(e12, e31, e23)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DTrivector, VGA3DVector);

// Trivector-Vector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl Mul<VGA3DVector> for VGA3DTrivector {
    type Output = VGA3DBivector;
    fn mul(self: VGA3DTrivector, b: VGA3DVector) -> VGA3DBivector {
        let e12 = self.e123() * b.e3();
        let e31 = self.e123() * b.e2();
        let e23 = self.e123() * b.e1();
        VGA3DBivector::new(e12, e31, e23)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DVector, VGA3DTrivector);

// Vector-Multivector
// \[ \vec{a}B\]
impl Mul<VGA3DMultivector> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DVector, b: VGA3DMultivector) -> VGA3DMultivector {
        let vec_scalar = self * b.scalar();
        let vec_vec = self * b.vector();
        let vec_bivec = self * b.bivector();
        let vec_trivec = self * b.trivector();

        let scalar = 0.0;
        let vector = vec_scalar;
        let bivector = vec_trivec;
        let trivector = VGA3DTrivector::zero();

        VGA3DMultivector::new(scalar, vector, bivector, trivector) + vec_vec + vec_bivec
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DMultivector, VGA3DVector);

// Multivector-Vector
impl Mul<VGA3DVector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DMultivector, b: VGA3DVector) -> VGA3DMultivector {
        let scalar_vec = self.scalar() * b;
        let vec_vec = self.vector() * b;
        let bivec_vec = self.bivector() * b;
        let trivec_vec = self.trivector() * b;

        let scalar = 0.0;
        let vector = scalar_vec;
        let bivector = trivec_vec;
        let trivector = VGA3DTrivector::zero();

        VGA3DMultivector::new(scalar, vector, bivector, trivector) + vec_vec + bivec_vec
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DVector, VGA3DMultivector);

// Vector-Rotor
// \[ \vec{a}R\]
impl Mul<VGA3DRotor> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DVector, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = 0.0;
        let vector = self * b.scalar();
        let bivector = VGA3DBivector::zero();
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector) + self * b.bivector()
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DRotor, VGA3DVector);

// Rotor-Vector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl Mul<VGA3DVector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DRotor, b: VGA3DVector) -> VGA3DMultivector {
        let scalar = 0.0;
        let vector = b * self.scalar();
        let bivector = VGA3DBivector::zero();
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector) + self.bivector() * b
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DVector, VGA3DRotor);

// Bivector-Bivector
// \[ \overset\Rightarrow{a} \overset\Rightarrow{b} = \overset\Rightarrow{a} \cdot \overset\Rightarrow{b} +  \overset\Rightarrow{a} \times \overset\Rightarrow{b} + \overset\Rightarrow{a} \wedge \overset\Rightarrow{b} \]
impl Mul for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DBivector, b: VGA3DBivector) -> VGA3DMultivector {
        let scalar = (self | b) + (self ^ b);
        let vector = VGA3DVector::zero();
        let bivector = self.cross(b);
        let trivector = VGA3DTrivector::zero();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DBivector, VGA3DBivector);

// Bivector-Trivector
// \[ \overset\Rightarrow{a}\overset\Rrightarrow{b}\]
impl Mul<VGA3DTrivector> for VGA3DBivector {
    type Output = VGA3DVector;
    fn mul(self: VGA3DBivector, b: VGA3DTrivector) -> VGA3DVector {
        let e1 = -self.e23() * b.e123();
        let e2 = -self.e31() * b.e123();
        let e3 = -self.e12() * b.e123();
        VGA3DVector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DBivector, VGA3DTrivector);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a}\overset\Rightarrow{b}\]
impl Mul<VGA3DBivector> for VGA3DTrivector {
    type Output = VGA3DVector;
    fn mul(self: VGA3DTrivector, b: VGA3DBivector) -> VGA3DVector {
        let e1 = -self.e123() * b.e23();
        let e2 = -self.e123() * b.e31();
        let e3 = -self.e123() * b.e12();
        VGA3DVector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DTrivector, VGA3DBivector);

// Bivector-Multivector
impl Mul<VGA3DMultivector> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DBivector, b: VGA3DMultivector) -> VGA3DMultivector {
        let bivec_scalar = self * b.scalar();
        let bivec_vec = self * b.vector();
        let bivec_bivec = self * b.bivector();
        let bivec_trivec = self * b.trivector();

        let scalar = 0.0;
        let vector = bivec_trivec;
        let bivector = bivec_scalar;
        let trivector = VGA3DTrivector::zero();

        VGA3DMultivector::new(scalar, vector, bivector, trivector) + bivec_vec + bivec_bivec
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DBivector, VGA3DMultivector);

// Multivector-Bivector
impl Mul<VGA3DBivector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DMultivector, b: VGA3DBivector) -> VGA3DMultivector {
        let scalar_bivec = self.scalar() * b;
        let vec_bivec = self.vector() * b;
        let bivec_bivec = self.bivector() * b;
        let trivec_bivec = self.trivector() * b;

        let scalar = 0.0;
        let vector = trivec_bivec;
        let bivector = scalar_bivec;
        let trivector = VGA3DTrivector::zero();

        VGA3DMultivector::new(scalar, vector, bivector, trivector) + vec_bivec + bivec_bivec
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DMultivector, VGA3DBivector);

// Bivector-Rotor
// \[ \vec{a}R\]
impl Mul<VGA3DRotor> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DBivector, b: VGA3DRotor) -> VGA3DMultivector {
        self * b.scalar() + self * b.bivector()
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DBivector, VGA3DRotor);

// Rotor-Bivector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl Mul<VGA3DBivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DRotor, b: VGA3DBivector) -> VGA3DMultivector {
        self.scalar() * b + self.bivector() * b
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DRotor, VGA3DBivector);

// Trivector-Trivector
// \[ \overset\Rrightarrow{a}\overset\Rrightarrow{b}= \overset\Rrightarrow{a} \cdot \overset\Rrightarrow{b}\]
impl Mul for VGA3DTrivector {
    type Output = f32;
    fn mul(self: VGA3DTrivector, b: VGA3DTrivector) -> f32 {
        -self.e123() * b.e123()
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DTrivector, VGA3DTrivector);

// Trivector-Multivector
impl Mul<VGA3DMultivector> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DTrivector, b: VGA3DMultivector) -> VGA3DMultivector {
        let trivec_scalar = self * b.scalar();
        let trivec_vec = self * b.vector();
        let trivec_bivec = self * b.bivector();
        let trivec_trivec = self * b.trivector();

        let scalar = trivec_trivec;
        let vector = trivec_bivec;
        let bivector = trivec_vec;
        let trivector = trivec_scalar;

        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DTrivector,VGA3DMultivector);

// Multivector-Trivector
impl Mul<VGA3DTrivector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DMultivector, b: VGA3DTrivector) -> VGA3DMultivector {
        let scalar_trivec = self.scalar() * b;
        let vec_trivec = self.vector() * b;
        let bivec_trivec = self.bivector() * b;
        let trivec_trivec = self.trivector() * b;

        let scalar = trivec_trivec;
        let vector = bivec_trivec;
        let bivector = vec_trivec;
        let trivector = scalar_trivec;

        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DMultivector, VGA3DTrivector);

// Trivector-Rotor
// \[ \vec{a}R\]
impl Mul<VGA3DRotor> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DTrivector, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = 0.0;
        let vector = self * b.bivector();
        let bivector = VGA3DBivector::zero();
        let trivector = self * b.scalar();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DTrivector,VGA3DRotor);

// Rotor-Trivector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl Mul<VGA3DTrivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DRotor, b: VGA3DTrivector) -> VGA3DMultivector {
        let scalar = 0.0;
        let vector = self.bivector() * b;
        let bivector = VGA3DBivector::zero();
        let trivector = b * self.scalar();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DRotor, VGA3DTrivector);

// Multivector-Multivector
impl Mul for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DMultivector, b: VGA3DMultivector) -> VGA3DMultivector {
        (self.scalar() * b) + (self.vector() * b) + (self.bivector() * b) + (self.trivector() * b)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DMultivector, VGA3DMultivector);

// Multivector-Rotor
// \[ \vec{a}R\]
impl Mul<VGA3DRotor> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DMultivector, b: VGA3DRotor) -> VGA3DMultivector {
        (self * b.scalar()) + (self * b.bivector())
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DMultivector, VGA3DRotor);

// Rotor-Multivector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl Mul<VGA3DMultivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn mul(self: VGA3DRotor, b: VGA3DMultivector) -> VGA3DMultivector {
        (self.scalar() * b) + (self.bivector() * b)
    }
}
forward_ref_binop!(impl Mul, mul for VGA3DRotor, VGA3DMultivector);

// Test
#[cfg(test)]
mod geometric_product {
    use core::f32::consts::TAU;

    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn vector_bivector_geo() {
        // 3e1+5e2+4e3
        let vector = VGA3DVector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let bivector = VGA3DBivector::new(2.0, 1.0, 6.0);
        let mvec = vector * bivector;
        // −6e1​−18e2​+27e3​+31e123​
        assert_relative_eq!(mvec.e1(), -6.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e2(), -18.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e3(), 27.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e123(), 31.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_vector_geo() {
        // 2e12+e31+6e23
        let bivector = VGA3DBivector::new(2.0, 1.0, 6.0);
        // 3e1+5e2+4e3
        let vector = VGA3DVector::new(3.0, 5.0, 4.0);
        let mvec = bivector * vector;
        // 6e1​+18e2​−27e3​+31e123​
        assert_relative_eq!(mvec.e1(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e2(), 18.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e3(), -27.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e123(), 31.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_bivector_mul() {
        // 3e12+5e31+4e23
        let bivector1 = VGA3DBivector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let bivector2 = VGA3DBivector::new(2.0, 1.0, 6.0);
        let mvec = bivector1 * bivector2;
        // −35+26e12​-10e31​−7e23​
        assert_relative_eq!(mvec.scalar(), -35.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e12(), 26.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e31(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e23(), -7.0, max_relative = 0.000001);
    }

    #[test]
    fn rotor_bivector_geo() {
        let angle = TAU / 4.0;
        let rotation_plane = VGA3DBivector::new(4.0, 2.0, -3.0);
        let rotor = VGA3DRotor::new(angle, rotation_plane);
        // 2e12+e31+6e23
        let bivector = VGA3DBivector::new(2.0, 1.0, 6.0);
        // 0.7071+0.5252e12+0.2626e31-0.3939e23
        let res = rotor * bivector;
        // 1.0504512787+3.3838100433e12​-3.2320864201e31​+4.2426404953e23
        assert_relative_eq!(res.scalar(), 1.0504512787, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 3.3838100433, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), -3.2320864201, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 4.2426404953, max_relative = 0.000001);
    }

    #[test]
    fn bivector_rotor_geo() {
        // 2e12+e31+6e23
        let bivector = VGA3DBivector::new(2.0, 1.0, 6.0);
        // 0.7071+0.5252e12+0.2626e31-0.3939e23
        let angle = TAU / 4.0;
        let rotation_plane = VGA3DBivector::new(4.0, 2.0, -3.0);
        let rotor = VGA3DRotor::new(angle, rotation_plane);
        let mvec = bivector * rotor;
        // 1.0504512787−0.5553830266e12​+4.646299839e31​+4.2426404953e23
        assert_relative_eq!(mvec.scalar(), 1.0504, max_relative = 0.001);
        assert_relative_eq!(mvec.e12(), -0.5553, max_relative = 0.001);
        assert_relative_eq!(mvec.e31(), 4.6461, max_relative = 0.001);
        assert_relative_eq!(mvec.e23(), 4.2425, max_relative = 0.001);
    }

    #[test]
    fn trivec_trivec_geo() {
        let trivector1 = VGA3DTrivector::new(3.0);
        let trivector2 = VGA3DTrivector::new(6.0);
        let res = trivector1 * trivector2;
        assert_relative_eq!(res, -18.0, max_relative = 0.000001);
    }

    #[test]
    fn mvec_mvec_mul() {
        let mvec1 = VGA3DMultivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        let mvec2 = VGA3DMultivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        let mvec_res = mvec1 * mvec2;
        // 94+126e1​−5e2​−65e3​+23e12​−131e13​+158e23​+236e123
        assert_relative_eq!(mvec_res.scalar(), 94.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), 126.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), -5.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), -65.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 23.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 131.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 158.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 236.0, max_relative = 0.000001);
    }
    #[test]
    fn negetive_mvec_mvec_mul() {
        // let mvec1 = GaMultivector::new_mvec(-6.0, -8.0, -4.0, -1.0, -6.0, -4.0, -8.0, -5.0);
        let mvec1 =
            VGA3DMultivector::new_components(-4.0, -1.0, -3.0, -2.0, -9.0, -6.0, -3.0, -10.0);
        let mvec2 =
            VGA3DMultivector::new_components(-4.0, -2.0, -4.0, -9.0, -2.0, -1.0, -7.0, -1.0);
        let mvec_res = mvec1 * mvec2;
        // −7−83e1​+9e2​+35e3​+173e12​−9e13​+77e23​+169e123
        assert_relative_eq!(mvec_res.scalar(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), -83.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), 9.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), 35.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 173.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 9.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 77.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 169.0, max_relative = 0.000001);
    }
}
