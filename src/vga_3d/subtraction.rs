#![allow(unused_imports)]
#![allow(dead_code)]

use super::{
    bivector::{self, VGA3DBivector},
    multivector::VGA3DMultivector,
    rotor::VGA3DRotor,
    trivector::{self, VGA3DTrivector},
    vector::VGA3DVector,
};

use core::ops::Sub;

// Scalar-Vector
// \[ a-\vec{b}\]
impl Sub<VGA3DVector> for f32 {
    type Output = VGA3DMultivector;
    fn sub(self: f32, b: VGA3DVector) -> VGA3DMultivector {
        VGA3DMultivector::new(self, -b, VGA3DBivector::zero(), VGA3DTrivector::zero())
    }
}

// Vector-Scalar
// \[ \vec{a}-b\]
impl Sub<f32> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DVector, b: f32) -> VGA3DMultivector {
        VGA3DMultivector::new(-b, self, VGA3DBivector::zero(), VGA3DTrivector::zero())
    }
}

// Scalar-Bivector
// \[ s-\overset\Rightarrow{b}\]
impl Sub<VGA3DBivector> for f32 {
    type Output = VGA3DMultivector;
    fn sub(self: f32, b: VGA3DBivector) -> VGA3DMultivector {
        VGA3DMultivector::new(self, VGA3DVector::zero(), -b, VGA3DTrivector::zero())
    }
}

// Bivector-Scalar
// \[ \overset\Rightarrow{a}-b\]
impl Sub<f32> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DBivector, b: f32) -> VGA3DMultivector {
        VGA3DMultivector::new(-b, VGA3DVector::zero(), self, VGA3DTrivector::zero())
    }
}

// Scalar-Trivector
// \[ s-\overset\Rrightarrow{b}\]
impl Sub<VGA3DTrivector> for f32 {
    type Output = VGA3DMultivector;
    fn sub(self: f32, b: VGA3DTrivector) -> VGA3DMultivector {
        VGA3DMultivector::new(self, VGA3DVector::zero(), VGA3DBivector::zero(), -b)
    }
}

// Trivector-Scalar
// \[ \overset\Rrightarrow{a}-b\]
impl Sub<f32> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DTrivector, b: f32) -> VGA3DMultivector {
        VGA3DMultivector::new(-b, VGA3DVector::zero(), VGA3DBivector::zero(), self)
    }
}

// Scalar-Multivector
// \[ s-B\]
impl Sub<VGA3DMultivector> for f32 {
    type Output = VGA3DMultivector;
    fn sub(self: f32, b: VGA3DMultivector) -> VGA3DMultivector {
        VGA3DMultivector::new(
            self,
            VGA3DVector::zero(),
            VGA3DBivector::zero(),
            VGA3DTrivector::zero(),
        ) - b
    }
}

// Multivector-Scalar
// \[ B-b\]
impl Sub<f32> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DMultivector, b: f32) -> VGA3DMultivector {
        VGA3DMultivector::new(
            -b,
            VGA3DVector::zero(),
            VGA3DBivector::zero(),
            VGA3DTrivector::zero(),
        ) + self
    }
}

// Scalar-Rotor
// \[ s-R\]
impl Sub<VGA3DRotor> for f32 {
    type Output = VGA3DMultivector;
    fn sub(self: f32, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(
            self - b.scalar(),
            VGA3DVector::zero(),
            b.bivector(),
            VGA3DTrivector::zero(),
        )
    }
}

//Rotor-Scalar
// \[ R-b\]
impl Sub<f32> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DRotor, b: f32) -> VGA3DMultivector {
        VGA3DMultivector::new(
            -b + self.scalar(),
            VGA3DVector::zero(),
            self.bivector(),
            VGA3DTrivector::zero(),
        )
    }
}

// Vector-Bivector
// \[ \vec{a}-\overset\Rightarrow{b}\]
impl Sub<VGA3DBivector> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DVector, b: VGA3DBivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, self, -b, VGA3DTrivector::zero())
    }
}

// Bivector-Vector
// \[ \overset\Rightarrow{b}-\vec{b}\]
impl Sub<VGA3DVector> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DBivector, b: VGA3DVector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, -b, self, VGA3DTrivector::zero())
    }
}

// Vector-Trivector
// \[ \vec{a}-\overset\Rrightarrow{b}\]
impl Sub<VGA3DTrivector> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DVector, b: VGA3DTrivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, self, VGA3DBivector::zero(), -b)
    }
}

// Trivector-Vector
// \[ \overset\Rrightarrow{a}-\vec{b}\]
impl Sub<VGA3DVector> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DTrivector, b: VGA3DVector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, -b, VGA3DBivector::zero(), self)
    }
}

// Vector-Multivector
// \[ \vec{a}-B\]
impl Sub<VGA3DMultivector> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DVector, b: VGA3DMultivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, self, VGA3DBivector::zero(), VGA3DTrivector::zero()) - b
    }
}

// Multivector-Vector
// \[ A-\vec{b}\]
impl Sub<VGA3DVector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DMultivector, b: VGA3DVector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, -b, VGA3DBivector::zero(), VGA3DTrivector::zero()) + self
    }
}

// Vector-Rotor
// \[ \vec{a}-R\]
impl Sub<VGA3DRotor> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DVector, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(b.scalar(), self, -b.bivector(), VGA3DTrivector::zero())
    }
}

// Multivector-Vector
// \[ R-\vec{b}\]
impl Sub<VGA3DVector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DRotor, b: VGA3DVector) -> VGA3DMultivector {
        VGA3DMultivector::new(self.scalar(), -b, self.bivector(), VGA3DTrivector::zero())
    }
}

// Bivector-Trivector
// \[ \overset\Rightarrow{a}-\overset\Rrightarrow{b}\]
impl Sub<VGA3DTrivector> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DBivector, b: VGA3DTrivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, VGA3DVector::zero(), self, -b)
    }
}

// Trivector-Bivector
// \[ \overset\Rrightarrow{a}-\overset\Rightarrow{b}\]
impl Sub<VGA3DBivector> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DTrivector, b: VGA3DBivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, VGA3DVector::zero(), -b, self)
    }
}

// Bivector-Multivector
// \[ \overset\Rightarrow{a}-B\]
impl Sub<VGA3DMultivector> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DBivector, b: VGA3DMultivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, VGA3DVector::zero(), self, VGA3DTrivector::zero()) - b
    }
}

// Multivector-Bivector
// \[ A-\overset\Rightarrow{b}\]
impl Sub<VGA3DBivector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DMultivector, b: VGA3DBivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, VGA3DVector::zero(), -b, VGA3DTrivector::zero()) + self
    }
}

// Bivector-Rotor
// \[ \overset\Rightarrow{a}-R\]
impl Sub<VGA3DRotor> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DBivector, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(
            -b.scalar(),
            VGA3DVector::zero(),
            self - b.bivector(),
            VGA3DTrivector::zero(),
        )
    }
}

// Rotor-Bivector
// \[ R-\overset\Rightarrow{b}\]
impl Sub<VGA3DBivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DRotor, b: VGA3DBivector) -> VGA3DMultivector {
        VGA3DMultivector::new(
            self.scalar(),
            VGA3DVector::zero(),
            -b + self.bivector(),
            VGA3DTrivector::zero(),
        )
    }
}

// Trivector-Multivector
// \[ \overset\Rrightarrow{a}-B\]
impl Sub<VGA3DMultivector> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DTrivector, b: VGA3DMultivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, VGA3DVector::zero(), VGA3DBivector::zero(), self) - b
    }
}

// Multivector-Trivector
// \[ A-\overset\Rrightarrow{b}\]
impl Sub<VGA3DTrivector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DMultivector, b: VGA3DTrivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, VGA3DVector::zero(), VGA3DBivector::zero(), -b) + self
    }
}

// Trivector-Rotor
// \[ \overset\Rrightarrow{a}-R\]
impl Sub<VGA3DRotor> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DTrivector, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(b.scalar(), VGA3DVector::zero(), -b.bivector(), self)
    }
}

// Rotor-Trivector
// \[ R-\overset\Rrightarrow{b}\]
impl Sub<VGA3DTrivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DRotor, b: VGA3DTrivector) -> VGA3DMultivector {
        VGA3DMultivector::new(self.scalar(), VGA3DVector::zero(), self.bivector(), -b)
    }
}

// Multivector-Rotor
// \[ A-R\]
impl Sub<VGA3DRotor> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DMultivector, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(
            -b.scalar(),
            VGA3DVector::zero(),
            -b.bivector(),
            VGA3DTrivector::zero(),
        ) + self
    }
}

// Rotor-Multivector
// \[ R-B\]
impl Sub<VGA3DMultivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn sub(self: VGA3DRotor, b: VGA3DMultivector) -> VGA3DMultivector {
        VGA3DMultivector::new(
            self.scalar(),
            VGA3DVector::zero(),
            self.bivector(),
            VGA3DTrivector::zero(),
        ) + b
    }
}
