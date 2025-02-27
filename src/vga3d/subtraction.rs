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
    bivector::{self, Bivector},
    multivector::Multivector,
    rotor::VGA3DRotor,
    trivector::{self, Trivector},
    vector::Vector,
};

use crate::forward_ref_binop;
use core::ops::Sub;

// Scalar-Vector
// \[ a-\vec{b}\]
impl Sub<Vector> for f32 {
    type Output = Multivector;
    fn sub(self: f32, b: Vector) -> Multivector {
        Multivector::new(self, -b, Bivector::zero(), Trivector::zero())
    }
}
forward_ref_binop!(impl Sub, sub for f32, Vector);

// Vector-Scalar
// \[ \vec{a}-b\]
impl Sub<f32> for Vector {
    type Output = Multivector;
    fn sub(self: Vector, b: f32) -> Multivector {
        Multivector::new(-b, self, Bivector::zero(), Trivector::zero())
    }
}
forward_ref_binop!(impl Sub, sub for Vector,f32);

// Scalar-Bivector
// \[ s-\overset\Rightarrow{b}\]
impl Sub<Bivector> for f32 {
    type Output = Multivector;
    fn sub(self: f32, b: Bivector) -> Multivector {
        Multivector::new(self, Vector::zero(), -b, Trivector::zero())
    }
}
forward_ref_binop!(impl Sub, sub for f32, Bivector);

// Bivector-Scalar
// \[ \overset\Rightarrow{a}-b\]
impl Sub<f32> for Bivector {
    type Output = Multivector;
    fn sub(self: Bivector, b: f32) -> Multivector {
        Multivector::new(-b, Vector::zero(), self, Trivector::zero())
    }
}
forward_ref_binop!(impl Sub, sub for  Bivector, f32);

// Scalar-Trivector
// \[ s-\overset\Rrightarrow{b}\]
impl Sub<Trivector> for f32 {
    type Output = Multivector;
    fn sub(self: f32, b: Trivector) -> Multivector {
        Multivector::new(self, Vector::zero(), Bivector::zero(), -b)
    }
}
forward_ref_binop!(impl Sub, sub for f32, Trivector);

// Trivector-Scalar
// \[ \overset\Rrightarrow{a}-b\]
impl Sub<f32> for Trivector {
    type Output = Multivector;
    fn sub(self: Trivector, b: f32) -> Multivector {
        Multivector::new(-b, Vector::zero(), Bivector::zero(), self)
    }
}
forward_ref_binop!(impl Sub, sub for Trivector, f32);

// Scalar-Multivector
// \[ s-B\]
impl Sub<Multivector> for f32 {
    type Output = Multivector;
    fn sub(self: f32, b: Multivector) -> Multivector {
        Multivector::new(
            self,
            Vector::zero(),
            Bivector::zero(),
            Trivector::zero(),
        ) - b
    }
}
forward_ref_binop!(impl Sub, sub for f32, Multivector);

// Multivector-Scalar
// \[ B-b\]
impl Sub<f32> for Multivector {
    type Output = Multivector;
    fn sub(self: Multivector, b: f32) -> Multivector {
        Multivector::new(
            -b,
            Vector::zero(),
            Bivector::zero(),
            Trivector::zero(),
        ) + self
    }
}
forward_ref_binop!(impl Sub, sub for Multivector, f32);

// Scalar-Rotor
// \[ s-R\]
impl Sub<VGA3DRotor> for f32 {
    type Output = Multivector;
    fn sub(self: f32, b: VGA3DRotor) -> Multivector {
        Multivector::new(
            self - b.scalar(),
            Vector::zero(),
            -b.bivector(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl Sub, sub for f32, VGA3DRotor);

//Rotor-Scalar
// \[ R-b\]
impl Sub<f32> for VGA3DRotor {
    type Output = Multivector;
    fn sub(self: VGA3DRotor, b: f32) -> Multivector {
        Multivector::new(
            -b + self.scalar(),
            Vector::zero(),
            self.bivector(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl Sub, sub for VGA3DRotor, f32);

impl Sub for Vector {
    type Output = Vector;

    fn sub(self: Vector, b: Vector) -> Vector {
        let e1 = self.e1() - b.e1();
        let e2 = self.e2() - b.e2();
        let e3 = self.e3() - b.e3();
        Vector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl Sub, sub for Vector, Vector);

// Vector-Bivector
// \[ \vec{a}-\overset\Rightarrow{b}\]
impl Sub<Bivector> for Vector {
    type Output = Multivector;
    fn sub(self: Vector, b: Bivector) -> Multivector {
        Multivector::new(0.0, self, -b, Trivector::zero())
    }
}
forward_ref_binop!(impl Sub, sub for Vector, Bivector);

// Bivector-Vector
// \[ \overset\Rightarrow{b}-\vec{b}\]
impl Sub<Vector> for Bivector {
    type Output = Multivector;
    fn sub(self: Bivector, b: Vector) -> Multivector {
        Multivector::new(0.0, -b, self, Trivector::zero())
    }
}
forward_ref_binop!(impl Sub, sub for Bivector, Vector);

// Vector-Trivector
// \[ \vec{a}-\overset\Rrightarrow{b}\]
impl Sub<Trivector> for Vector {
    type Output = Multivector;
    fn sub(self: Vector, b: Trivector) -> Multivector {
        Multivector::new(0.0, self, Bivector::zero(), -b)
    }
}
forward_ref_binop!(impl Sub, sub for Vector, Trivector);

// Trivector-Vector
// \[ \overset\Rrightarrow{a}-\vec{b}\]
impl Sub<Vector> for Trivector {
    type Output = Multivector;
    fn sub(self: Trivector, b: Vector) -> Multivector {
        Multivector::new(0.0, -b, Bivector::zero(), self)
    }
}
forward_ref_binop!(impl Sub, sub for Trivector, Vector);

// Vector-Multivector
// \[ \vec{a}-B\]
impl Sub<Multivector> for Vector {
    type Output = Multivector;
    fn sub(self: Vector, b: Multivector) -> Multivector {
        Multivector::new(0.0, self, Bivector::zero(), Trivector::zero()) - b
    }
}
forward_ref_binop!(impl Sub, sub for Vector, Multivector);

// Multivector-Vector
// \[ A-\vec{b}\]
impl Sub<Vector> for Multivector {
    type Output = Multivector;
    fn sub(self: Multivector, b: Vector) -> Multivector {
        Multivector::new(0.0, -b, Bivector::zero(), Trivector::zero()) + self
    }
}
forward_ref_binop!(impl Sub, sub for Multivector, Vector);

// Vector-Rotor
// \[ \vec{a}-R\]
impl Sub<VGA3DRotor> for Vector {
    type Output = Multivector;
    fn sub(self: Vector, b: VGA3DRotor) -> Multivector {
        Multivector::new(-b.scalar(), self, -b.bivector(), Trivector::zero())
    }
}
forward_ref_binop!(impl Sub, sub for Vector, VGA3DRotor);

// Multivector-Vector
// \[ R-\vec{b}\]
impl Sub<Vector> for VGA3DRotor {
    type Output = Multivector;
    fn sub(self: VGA3DRotor, b: Vector) -> Multivector {
        Multivector::new(self.scalar(), -b, self.bivector(), Trivector::zero())
    }
}
forward_ref_binop!(impl Sub, sub for VGA3DRotor, Vector);

// Bivector-Bivector
impl Sub for Bivector {
    type Output = Bivector;
    fn sub(self: Bivector, b: Bivector) -> Bivector {
        let e12 = self.e12() - b.e12();
        let e31 = self.e31() - b.e31();
        let e23 = self.e23() - b.e23();
        Bivector::new(e12, e31, e23)
    }
}
forward_ref_binop!(impl Sub, sub for Bivector, Bivector);

// Bivector-Trivector
// \[ \overset\Rightarrow{a}-\overset\Rrightarrow{b}\]
impl Sub<Trivector> for Bivector {
    type Output = Multivector;
    fn sub(self: Bivector, b: Trivector) -> Multivector {
        Multivector::new(0.0, Vector::zero(), self, -b)
    }
}
forward_ref_binop!(impl Sub, sub for Bivector, Trivector);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a}-\overset\Rightarrow{b}\]
impl Sub<Bivector> for Trivector {
    type Output = Multivector;
    fn sub(self: Trivector, b: Bivector) -> Multivector {
        Multivector::new(0.0, Vector::zero(), -b, self)
    }
}
forward_ref_binop!(impl Sub, sub for Trivector, Bivector);

// Bivector-Multivector
// \[ \overset\Rightarrow{a}-B\]
impl Sub<Multivector> for Bivector {
    type Output = Multivector;
    fn sub(self: Bivector, b: Multivector) -> Multivector {
        Multivector::new(0.0, Vector::zero(), self, Trivector::zero()) - b
    }
}
forward_ref_binop!(impl Sub, sub for Bivector, Multivector);

// Multivector-Bivector
// \[ A-\overset\Rightarrow{b}\]
impl Sub<Bivector> for Multivector {
    type Output = Multivector;
    fn sub(self: Multivector, b: Bivector) -> Multivector {
        Multivector::new(0.0, Vector::zero(), -b, Trivector::zero()) + self
    }
}
forward_ref_binop!(impl Sub, sub for Multivector, Bivector);

// Bivector-Rotor
// \[ \overset\Rightarrow{a}-R\]
impl Sub<VGA3DRotor> for Bivector {
    type Output = Multivector;
    fn sub(self: Bivector, b: VGA3DRotor) -> Multivector {
        Multivector::new(
            -b.scalar(),
            Vector::zero(),
            self - b.bivector(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl Sub, sub for Bivector, VGA3DRotor);

// Rotor-Bivector
// \[ R-\overset\Rightarrow{b}\]
impl Sub<Bivector> for VGA3DRotor {
    type Output = Multivector;
    fn sub(self: VGA3DRotor, b: Bivector) -> Multivector {
        Multivector::new(
            self.scalar(),
            Vector::zero(),
            -b + self.bivector(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl Sub, sub for VGA3DRotor, Bivector);

// Trivector-Trivector
impl Sub for Trivector {
    type Output = Trivector;
    fn sub(self: Trivector, b: Trivector) -> Trivector {
        let e123 = self.e123() - b.e123();
        Trivector::new(e123)
    }
}
forward_ref_binop!(impl Sub, sub for Trivector, Trivector);

// Trivector-Multivector
// \[ \overset\Rrightarrow{a}-B\]
impl Sub<Multivector> for Trivector {
    type Output = Multivector;
    fn sub(self: Trivector, b: Multivector) -> Multivector {
        Multivector::new(0.0, Vector::zero(), Bivector::zero(), self) - b
    }
}
forward_ref_binop!(impl Sub, sub for Trivector, Multivector);

// Multivector-Trivector
// \[ A-\overset\Rrightarrow{b}\]
impl Sub<Trivector> for Multivector {
    type Output = Multivector;
    fn sub(self: Multivector, b: Trivector) -> Multivector {
        Multivector::new(0.0, Vector::zero(), Bivector::zero(), -b) + self
    }
}
forward_ref_binop!(impl Sub, sub for Multivector, Trivector);

// Trivector-Rotor
// \[ \overset\Rrightarrow{a}-R\]
impl Sub<VGA3DRotor> for Trivector {
    type Output = Multivector;
    fn sub(self: Trivector, b: VGA3DRotor) -> Multivector {
        Multivector::new(-b.scalar(), Vector::zero(), -b.bivector(), self)
    }
}
forward_ref_binop!(impl Sub, sub for Trivector, VGA3DRotor);

// Rotor-Trivector
// \[ R-\overset\Rrightarrow{b}\]
impl Sub<Trivector> for VGA3DRotor {
    type Output = Multivector;
    fn sub(self: VGA3DRotor, b: Trivector) -> Multivector {
        Multivector::new(self.scalar(), Vector::zero(), self.bivector(), -b)
    }
}
forward_ref_binop!(impl Sub, sub for VGA3DRotor, Trivector);

// Multivector-Multivector
impl Sub for Multivector {
    type Output = Multivector;
    fn sub(self: Multivector, b: Multivector) -> Multivector {
        let scalar = self.scalar() - b.scalar();
        let vector = self.vector() - b.vector();
        let bivector = self.bivector() - b.bivector();
        let trivector = self.trivector() - b.trivector();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Sub, sub for Multivector, Multivector);

// Multivector-Rotor
// \[ A-R\]
impl Sub<VGA3DRotor> for Multivector {
    type Output = Multivector;
    fn sub(self: Multivector, b: VGA3DRotor) -> Multivector {
        Multivector::new(
            -b.scalar(),
            Vector::zero(),
            -b.bivector(),
            Trivector::zero(),
        ) + self
    }
}
forward_ref_binop!(impl Sub, sub for VGA3DRotor, Multivector);

// Rotor-Multivector
// \[ R-B\]
impl Sub<Multivector> for VGA3DRotor {
    type Output = Multivector;
    fn sub(self: VGA3DRotor, b: Multivector) -> Multivector {
        Multivector::new(
            self.scalar(),
            Vector::zero(),
            self.bivector(),
            Trivector::zero(),
        ) - b
    }
}
forward_ref_binop!(impl Sub, sub for Multivector, VGA3DRotor);

impl Sub for VGA3DRotor {
    type Output = Multivector;
    fn sub(self: VGA3DRotor, b: VGA3DRotor) -> Multivector {
        let scalar = self.scalar() - b.scalar();
        let bivector = self.bivector() - b.bivector();
        Multivector::new(
            scalar,
            Vector::zero(),
            bivector,
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl Sub, sub for VGA3DRotor, VGA3DRotor);
