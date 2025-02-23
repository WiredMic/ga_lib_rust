#![allow(unused_imports)]
#![allow(dead_code)]

use super::{
    bivector::{self, VGA3DBivector},
    multivector::VGA3DMultivector,
    rotor::VGA3DRotor,
    trivector::{self, VGA3DTrivector},
    vector::VGA3DVector,
};

use core::ops::Add;
use forward_ref::forward_ref_binop;

// Scalar-Vector
// \[ a+\vec{b}\]
impl Add<VGA3DVector> for f32 {
    type Output = VGA3DMultivector;
    fn add(self: f32, b: VGA3DVector) -> VGA3DMultivector {
        VGA3DMultivector::new(self, b, VGA3DBivector::zero(), VGA3DTrivector::zero())
    }
}
forward_ref_binop!(impl Add, add for f32, VGA3DVector);

// Vector-Scalar
// \[ \vec{a}+b\]
impl Add<f32> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DVector, b: f32) -> VGA3DMultivector {
        VGA3DMultivector::new(b, self, VGA3DBivector::zero(), VGA3DTrivector::zero())
    }
}
forward_ref_binop!(impl Add, add for VGA3DVector,f32);

// Scalar-Bivector
// \[ s+\overset\Rightarrow{b}\]
impl Add<VGA3DBivector> for f32 {
    type Output = VGA3DMultivector;
    fn add(self: f32, b: VGA3DBivector) -> VGA3DMultivector {
        VGA3DMultivector::new(self, VGA3DVector::zero(), b, VGA3DTrivector::zero())
    }
}
forward_ref_binop!(impl Add, add for f32, VGA3DBivector);

// Bivector-Scalar
// \[ \overset\Rightarrow{a}+b\]
impl Add<f32> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DBivector, b: f32) -> VGA3DMultivector {
        VGA3DMultivector::new(b, VGA3DVector::zero(), self, VGA3DTrivector::zero())
    }
}
forward_ref_binop!(impl Add, add for  VGA3DBivector, f32);

// Scalar-Trivector
// \[ s+\overset\Rrightarrow{b}\]
impl Add<VGA3DTrivector> for f32 {
    type Output = VGA3DMultivector;
    fn add(self: f32, b: VGA3DTrivector) -> VGA3DMultivector {
        VGA3DMultivector::new(self, VGA3DVector::zero(), VGA3DBivector::zero(), b)
    }
}
forward_ref_binop!(impl Add, add for f32, VGA3DTrivector);

// Trivector-Scalar
// \[ \overset\Rrightarrow{a}+b\]
impl Add<f32> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DTrivector, b: f32) -> VGA3DMultivector {
        VGA3DMultivector::new(b, VGA3DVector::zero(), VGA3DBivector::zero(), self)
    }
}
forward_ref_binop!(impl Add, add for VGA3DTrivector, f32);

// Scalar-Multivector
// \[ s+B\]
impl Add<VGA3DMultivector> for f32 {
    type Output = VGA3DMultivector;
    fn add(self: f32, b: VGA3DMultivector) -> VGA3DMultivector {
        VGA3DMultivector::new(
            self,
            VGA3DVector::zero(),
            VGA3DBivector::zero(),
            VGA3DTrivector::zero(),
        ) + b
    }
}
forward_ref_binop!(impl Add, add for f32, VGA3DMultivector);

// Multivector-Scalar
// \[ B+b\]
impl Add<f32> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DMultivector, b: f32) -> VGA3DMultivector {
        VGA3DMultivector::new(
            b,
            VGA3DVector::zero(),
            VGA3DBivector::zero(),
            VGA3DTrivector::zero(),
        ) + self
    }
}
forward_ref_binop!(impl Add, add for VGA3DMultivector, f32);

// Scalar-Rotor
// \[ s+R\]
impl Add<VGA3DRotor> for f32 {
    type Output = VGA3DMultivector;
    fn add(self: f32, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(
            self + b.scalar(),
            VGA3DVector::zero(),
            b.bivector(),
            VGA3DTrivector::zero(),
        )
    }
}
forward_ref_binop!(impl Add, add for f32, VGA3DRotor);

//Rotor-Scalar
// \[ R+b\]
impl Add<f32> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DRotor, b: f32) -> VGA3DMultivector {
        VGA3DMultivector::new(
            b + self.scalar(),
            VGA3DVector::zero(),
            self.bivector(),
            VGA3DTrivector::zero(),
        )
    }
}
forward_ref_binop!(impl Add, add for VGA3DRotor, f32);

impl Add for VGA3DVector {
    type Output = VGA3DVector;

    fn add(self: VGA3DVector, b: VGA3DVector) -> VGA3DVector {
        let e1 = self.e1() + b.e1();
        let e2 = self.e2() + b.e2();
        let e3 = self.e3() + b.e3();
        VGA3DVector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl Add, add for VGA3DVector, VGA3DVector);

// Vector-Bivector
// \[ \vec{a}+\overset\Rightarrow{b}\]
impl Add<VGA3DBivector> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DVector, b: VGA3DBivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, self, b, VGA3DTrivector::zero())
    }
}
forward_ref_binop!(impl Add, add for VGA3DVector, VGA3DBivector);

// Bivector-Vector
// \[ \overset\Rightarrow{b}+\vec{b}\]
impl Add<VGA3DVector> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DBivector, b: VGA3DVector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, b, self, VGA3DTrivector::zero())
    }
}
forward_ref_binop!(impl Add, add for VGA3DBivector, VGA3DVector);

// Vector-Trivector
// \[ \vec{a}+\overset\Rrightarrow{b}\]
impl Add<VGA3DTrivector> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DVector, b: VGA3DTrivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, self, VGA3DBivector::zero(), b)
    }
}
forward_ref_binop!(impl Add, add for VGA3DVector, VGA3DTrivector);

// Trivector-Vector
// \[ \overset\Rrightarrow{a}+\vec{b}\]
impl Add<VGA3DVector> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DTrivector, b: VGA3DVector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, b, VGA3DBivector::zero(), self)
    }
}
forward_ref_binop!(impl Add, add for VGA3DTrivector, VGA3DVector);

// Vector-Multivector
// \[ \vec{a}+B\]
impl Add<VGA3DMultivector> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DVector, b: VGA3DMultivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, self, VGA3DBivector::zero(), VGA3DTrivector::zero()) + b
    }
}
forward_ref_binop!(impl Add, add for VGA3DVector, VGA3DMultivector);

// Multivector-Vector
// \[ A+\vec{b}\]
impl Add<VGA3DVector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DMultivector, b: VGA3DVector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, b, VGA3DBivector::zero(), VGA3DTrivector::zero()) + self
    }
}
forward_ref_binop!(impl Add, add for VGA3DMultivector, VGA3DVector);

// Vector-Rotor
// \[ \vec{a}+R\]
impl Add<VGA3DRotor> for VGA3DVector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DVector, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(b.scalar(), self, b.bivector(), VGA3DTrivector::zero())
    }
}
forward_ref_binop!(impl Add, add for VGA3DVector, VGA3DRotor);

// Multivector-Vector
// \[ R+\vec{b}\]
impl Add<VGA3DVector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DRotor, b: VGA3DVector) -> VGA3DMultivector {
        VGA3DMultivector::new(self.scalar(), b, self.bivector(), VGA3DTrivector::zero())
    }
}
forward_ref_binop!(impl Add, add for VGA3DRotor, VGA3DVector);

// Bivector-Bivector
impl Add for VGA3DBivector {
    type Output = VGA3DBivector;
    fn add(self: VGA3DBivector, b: VGA3DBivector) -> VGA3DBivector {
        let e12 = self.e12() + b.e12();
        let e31 = self.e31() + b.e31();
        let e23 = self.e23() + b.e23();
        VGA3DBivector::new(e12, e31, e23)
    }
}
forward_ref_binop!(impl Add, add for VGA3DBivector, VGA3DBivector);

// Bivector-Trivector
// \[ \overset\Rightarrow{a}+\overset\Rrightarrow{b}\]
impl Add<VGA3DTrivector> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DBivector, b: VGA3DTrivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, VGA3DVector::zero(), self, b)
    }
}
forward_ref_binop!(impl Add, add for VGA3DBivector, VGA3DTrivector);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a}+\overset\Rightarrow{b}\]
impl Add<VGA3DBivector> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DTrivector, b: VGA3DBivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, VGA3DVector::zero(), b, self)
    }
}
forward_ref_binop!(impl Add, add for VGA3DTrivector, VGA3DBivector);

// Bivector-Multivector
// \[ \overset\Rightarrow{a}+B\]
impl Add<VGA3DMultivector> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DBivector, b: VGA3DMultivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, VGA3DVector::zero(), self, VGA3DTrivector::zero()) + b
    }
}
forward_ref_binop!(impl Add, add for VGA3DBivector, VGA3DMultivector);

// Multivector-Bivector
// \[ A+\overset\Rightarrow{b}\]
impl Add<VGA3DBivector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DMultivector, b: VGA3DBivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, VGA3DVector::zero(), b, VGA3DTrivector::zero()) + self
    }
}
forward_ref_binop!(impl Add, add for VGA3DMultivector, VGA3DBivector);

// Bivector-Rotor
// \[ \overset\Rightarrow{a}+R\]
impl Add<VGA3DRotor> for VGA3DBivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DBivector, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(
            b.scalar(),
            VGA3DVector::zero(),
            self + b.bivector(),
            VGA3DTrivector::zero(),
        )
    }
}
forward_ref_binop!(impl Add, add for VGA3DBivector, VGA3DRotor);

// Rotor-Bivector
// \[ R+\overset\Rightarrow{b}\]
impl Add<VGA3DBivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DRotor, b: VGA3DBivector) -> VGA3DMultivector {
        VGA3DMultivector::new(
            self.scalar(),
            VGA3DVector::zero(),
            b + self.bivector(),
            VGA3DTrivector::zero(),
        )
    }
}
forward_ref_binop!(impl Add, add for VGA3DRotor, VGA3DBivector);

// Trivector-Trivector
impl Add for VGA3DTrivector {
    type Output = VGA3DTrivector;
    fn add(self: VGA3DTrivector, b: VGA3DTrivector) -> VGA3DTrivector {
        let e123 = self.e123() + b.e123();
        VGA3DTrivector::new(e123)
    }
}
forward_ref_binop!(impl Add, add for VGA3DTrivector, VGA3DTrivector);

// Trivector-Multivector
// \[ \overset\Rrightarrow{a}+B\]
impl Add<VGA3DMultivector> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DTrivector, b: VGA3DMultivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, VGA3DVector::zero(), VGA3DBivector::zero(), self) + b
    }
}
forward_ref_binop!(impl Add, add for VGA3DTrivector, VGA3DMultivector);

// Multivector-Trivector
// \[ A+\overset\Rrightarrow{b}\]
impl Add<VGA3DTrivector> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DMultivector, b: VGA3DTrivector) -> VGA3DMultivector {
        VGA3DMultivector::new(0.0, VGA3DVector::zero(), VGA3DBivector::zero(), b) + self
    }
}
forward_ref_binop!(impl Add, add for VGA3DMultivector, VGA3DTrivector);

// Trivector-Rotor
// \[ \overset\Rrightarrow{a}+R\]
impl Add<VGA3DRotor> for VGA3DTrivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DTrivector, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(b.scalar(), VGA3DVector::zero(), b.bivector(), self)
    }
}
forward_ref_binop!(impl Add, add for VGA3DTrivector, VGA3DRotor);

// Rotor-Trivector
// \[ R+\overset\Rrightarrow{b}\]
impl Add<VGA3DTrivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DRotor, b: VGA3DTrivector) -> VGA3DMultivector {
        VGA3DMultivector::new(self.scalar(), VGA3DVector::zero(), self.bivector(), b)
    }
}
forward_ref_binop!(impl Add, add for VGA3DRotor, VGA3DTrivector);

// Multivector-Multivector
impl Add for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DMultivector, b: VGA3DMultivector) -> VGA3DMultivector {
        let scalar = self.scalar() + b.scalar();
        let vector = self.vector() + b.vector();
        let bivector = self.bivector() + b.bivector();
        let trivector = self.trivector() + b.trivector();
        VGA3DMultivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Add, add for VGA3DMultivector, VGA3DMultivector);

// Multivector-Rotor
// \[ A+R\]
impl Add<VGA3DRotor> for VGA3DMultivector {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DMultivector, b: VGA3DRotor) -> VGA3DMultivector {
        VGA3DMultivector::new(
            b.scalar(),
            VGA3DVector::zero(),
            b.bivector(),
            VGA3DTrivector::zero(),
        ) + self
    }
}
forward_ref_binop!(impl Add, add for VGA3DRotor, VGA3DMultivector);

// Rotor-Multivector
// \[ R+B\]
impl Add<VGA3DMultivector> for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DRotor, b: VGA3DMultivector) -> VGA3DMultivector {
        VGA3DMultivector::new(
            self.scalar(),
            VGA3DVector::zero(),
            self.bivector(),
            VGA3DTrivector::zero(),
        ) + b
    }
}
forward_ref_binop!(impl Add, add for VGA3DMultivector, VGA3DRotor);

impl Add for VGA3DRotor {
    type Output = VGA3DMultivector;
    fn add(self: VGA3DRotor, b: VGA3DRotor) -> VGA3DMultivector {
        let scalar = self.scalar() + b.scalar();
        let bivector = self.bivector() + b.bivector();
        VGA3DMultivector::new(
            scalar,
            VGA3DVector::zero(),
            bivector,
            VGA3DTrivector::zero(),
        )
    }
}
forward_ref_binop!(impl Add, add for VGA3DRotor, VGA3DRotor);
