// ga_lib is a rust library that implements different geometric algbras.
// Copyright (C) 2025 Rasmus Enevoldsen
//
// This file is part of ga_lib.
//
// ga_lib is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// ga_lib is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more
// details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with ga_lib. If not, see <https://www.gnu.org/licenses/>.

#![allow(unused_imports)]
#![allow(dead_code)]

use super::{
    bivector::Bivector, multivector::Multivector, rotor::Rotor, scalar::Scalar,
    trivector::Trivector, vector::Vector,
};

use num_traits::Float;

use crate::forward_ref_binop;
use core::ops::Sub;

// Scalar-Vector
// \[ a-\vec{b}\]
impl<F: Float> Sub<Vector<F>> for Scalar<F> {
    type Output = Multivector<F>;
    fn sub(self: Scalar<F>, b: Vector<F>) -> Multivector<F> {
        Multivector::new(self.0, -b, Bivector::zero(), Trivector::zero())
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Scalar<F>, Vector<F>);

// Vector-Scalar
// \[ \vec{a}-b\]
impl<F: Float> Sub<Scalar<F>> for Vector<F> {
    type Output = Multivector<F>;
    fn sub(self: Vector<F>, b: Scalar<F>) -> Multivector<F> {
        Multivector::new(-b.0, self, Bivector::zero(), Trivector::zero())
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Vector<F>,Scalar<F>);

// Scalar-Bivector
// \[ s-\overset\Rightarrow{b}\]
impl<F: Float> Sub<Bivector<F>> for Scalar<F> {
    type Output = Multivector<F>;
    fn sub(self: Scalar<F>, b: Bivector<F>) -> Multivector<F> {
        Multivector::new(self.0, Vector::zero(), -b, Trivector::zero())
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Scalar<F>, Bivector<F>);

// Bivector-Scalar
// \[ \overset\Rightarrow{a}-b\]
impl<F: Float> Sub<Scalar<F>> for Bivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Bivector<F>, b: Scalar<F>) -> Multivector<F> {
        Multivector::new(-b.0, Vector::zero(), self, Trivector::zero())
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for  Bivector<F>, Scalar<F>);

// Scalar-Trivector
// \[ s-\overset\Rrightarrow{b}\]
impl<F: Float> Sub<Trivector<F>> for Scalar<F> {
    type Output = Multivector<F>;
    fn sub(self: Scalar<F>, b: Trivector<F>) -> Multivector<F> {
        Multivector::new(self.0, Vector::zero(), Bivector::zero(), -b)
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Scalar<F>, Trivector<F>);

// Trivector-Scalar
// \[ \overset\Rrightarrow{a}-b\]
impl<F: Float> Sub<Scalar<F>> for Trivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Trivector<F>, b: Scalar<F>) -> Multivector<F> {
        Multivector::new(-b.0, Vector::zero(), Bivector::zero(), self)
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Trivector<F>, Scalar<F>);

// Scalar-Multivector
// \[ s-B\]
impl<F: Float> Sub<Multivector<F>> for Scalar<F> {
    type Output = Multivector<F>;
    fn sub(self: Scalar<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(self.0, Vector::zero(), Bivector::zero(), Trivector::zero()) - b
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Scalar<F>, Multivector<F>);

// Multivector-Scalar
// \[ B-b\]
impl<F: Float> Sub<Scalar<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Multivector<F>, b: Scalar<F>) -> Multivector<F> {
        Multivector::new(-b.0, Vector::zero(), Bivector::zero(), Trivector::zero()) + self
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Multivector<F>, Scalar<F>);

// Scalar-Rotor
// \[ s-R\]
impl<F: Float> Sub<Rotor<F>> for Scalar<F> {
    type Output = Multivector<F>;
    fn sub(self: Scalar<F>, b: Rotor<F>) -> Multivector<F> {
        Multivector::new(
            self.0 - b.scalar(),
            Vector::zero(),
            -b.bivector(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Scalar<F>, Rotor<F>);

//Rotor-Scalar
// \[ R-b\]
impl<F: Float> Sub<Scalar<F>> for Rotor<F> {
    type Output = Multivector<F>;
    fn sub(self: Rotor<F>, b: Scalar<F>) -> Multivector<F> {
        Multivector::new(
            -b.0 + self.scalar(),
            Vector::zero(),
            self.bivector(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Rotor<F>, Scalar<F>);

impl<F: Float> Sub for Vector<F> {
    type Output = Vector<F>;
    fn sub(self: Vector<F>, b: Vector<F>) -> Vector<F> {
        let e1 = self.e1() - b.e1();
        let e2 = self.e2() - b.e2();
        let e3 = self.e3() - b.e3();
        Vector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Vector<F>, Vector<F>);

// Vector-Bivector
// \[ \vec{a}-\overset\Rightarrow{b}\]
impl<F: Float> Sub<Bivector<F>> for Vector<F> {
    type Output = Multivector<F>;
    fn sub(self: Vector<F>, b: Bivector<F>) -> Multivector<F> {
        Multivector::new(F::zero(), self, -b, Trivector::zero())
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Vector<F>, Bivector<F>);

// Bivector-Vector
// \[ \overset\Rightarrow{b}-\vec{b}\]
impl<F: Float> Sub<Vector<F>> for Bivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Bivector<F>, b: Vector<F>) -> Multivector<F> {
        Multivector::new(F::zero(), -b, self, Trivector::zero())
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Bivector<F>, Vector<F>);

// Vector-Trivector
// \[ \vec{a}-\overset\Rrightarrow{b}\]
impl<F: Float> Sub<Trivector<F>> for Vector<F> {
    type Output = Multivector<F>;
    fn sub(self: Vector<F>, b: Trivector<F>) -> Multivector<F> {
        Multivector::new(F::zero(), self, Bivector::zero(), -b)
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Vector<F>, Trivector<F>);

// Trivector-Vector
// \[ \overset\Rrightarrow{a}-\vec{b}\]
impl<F: Float> Sub<Vector<F>> for Trivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Trivector<F>, b: Vector<F>) -> Multivector<F> {
        Multivector::new(F::zero(), -b, Bivector::zero(), self)
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Trivector<F>, Vector<F>);

// Vector-Multivector
// \[ \vec{a}-B\]
impl<F: Float> Sub<Multivector<F>> for Vector<F> {
    type Output = Multivector<F>;
    fn sub(self: Vector<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(F::zero(), self, Bivector::zero(), Trivector::zero()) - b
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Vector<F>, Multivector<F>);

// Multivector-Vector
// \[ A-\vec{b}\]
impl<F: Float> Sub<Vector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Multivector<F>, b: Vector<F>) -> Multivector<F> {
        Multivector::new(F::zero(), -b, Bivector::zero(), Trivector::zero()) + self
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Multivector<F>, Vector<F>);

// Vector-Rotor
// \[ \vec{a}-R\]
impl<F: Float> Sub<Rotor<F>> for Vector<F> {
    type Output = Multivector<F>;
    fn sub(self: Vector<F>, b: Rotor<F>) -> Multivector<F> {
        Multivector::new(-b.scalar(), self, -b.bivector(), Trivector::zero())
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Vector<F>, Rotor<F>);

// Multivector-Vector
// \[ R-\vec{b}\]
impl<F: Float> Sub<Vector<F>> for Rotor<F> {
    type Output = Multivector<F>;
    fn sub(self: Rotor<F>, b: Vector<F>) -> Multivector<F> {
        Multivector::new(self.scalar(), -b, self.bivector(), Trivector::zero())
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Rotor<F>, Vector<F>);

// Bivector-Bivector
impl<F: Float> Sub for Bivector<F> {
    type Output = Bivector<F>;
    fn sub(self: Bivector<F>, b: Bivector<F>) -> Bivector<F> {
        let e12 = self.e12() - b.e12();
        let e31 = self.e31() - b.e31();
        let e23 = self.e23() - b.e23();
        Bivector::new(e12, e31, e23)
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Bivector<F>, Bivector<F>);

// Bivector-Trivector
// \[ \overset\Rightarrow{a}-\overset\Rrightarrow{b}\]
impl<F: Float> Sub<Trivector<F>> for Bivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Bivector<F>, b: Trivector<F>) -> Multivector<F> {
        Multivector::new(F::zero(), Vector::zero(), self, -b)
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Bivector<F>, Trivector<F>);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a}-\overset\Rightarrow{b}\]
impl<F: Float> Sub<Bivector<F>> for Trivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Trivector<F>, b: Bivector<F>) -> Multivector<F> {
        Multivector::new(F::zero(), Vector::zero(), -b, self)
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Trivector<F>, Bivector<F>);

// Bivector-Multivector
// \[ \overset\Rightarrow{a}-B\]
impl<F: Float> Sub<Multivector<F>> for Bivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Bivector<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(F::zero(), Vector::zero(), self, Trivector::zero()) - b
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Bivector<F>, Multivector<F>);

// Multivector-Bivector
// \[ A-\overset\Rightarrow{b}\]
impl<F: Float> Sub<Bivector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Multivector<F>, b: Bivector<F>) -> Multivector<F> {
        Multivector::new(F::zero(), Vector::zero(), -b, Trivector::zero()) + self
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Multivector<F>, Bivector<F>);

// Bivector-Rotor
// \[ \overset\Rightarrow{a}-R\]
impl<F: Float> Sub<Rotor<F>> for Bivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Bivector<F>, b: Rotor<F>) -> Multivector<F> {
        Multivector::new(
            -b.scalar(),
            Vector::zero(),
            self - b.bivector(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Bivector<F>, Rotor<F>);

// Rotor-Bivector
// \[ R-\overset\Rightarrow{b}\]
impl<F: Float> Sub<Bivector<F>> for Rotor<F> {
    type Output = Multivector<F>;
    fn sub(self: Rotor<F>, b: Bivector<F>) -> Multivector<F> {
        Multivector::new(
            self.scalar(),
            Vector::zero(),
            -b + self.bivector(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Rotor<F>, Bivector<F>);

// Trivector-Trivector
impl<F: Float> Sub for Trivector<F> {
    type Output = Trivector<F>;
    fn sub(self: Trivector<F>, b: Trivector<F>) -> Trivector<F> {
        let e123 = self.e123() - b.e123();
        Trivector::new(e123)
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Trivector<F>, Trivector<F>);

// Trivector-Multivector
// \[ \overset\Rrightarrow{a}-B\]
impl<F: Float> Sub<Multivector<F>> for Trivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Trivector<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(F::zero(), Vector::zero(), Bivector::zero(), self) - b
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Trivector<F>, Multivector<F>);

// Multivector-Trivector
// \[ A-\overset\Rrightarrow{b}\]
impl<F: Float> Sub<Trivector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Multivector<F>, b: Trivector<F>) -> Multivector<F> {
        Multivector::new(F::zero(), Vector::zero(), Bivector::zero(), -b) + self
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Multivector<F>, Trivector<F>);

// Trivector-Rotor
// \[ \overset\Rrightarrow{a}-R\]
impl<F: Float> Sub<Rotor<F>> for Trivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Trivector<F>, b: Rotor<F>) -> Multivector<F> {
        Multivector::new(-b.scalar(), Vector::zero(), -b.bivector(), self)
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Trivector<F>, Rotor<F>);

// Rotor-Trivector
// \[ R-\overset\Rrightarrow{b}\]
impl<F: Float> Sub<Trivector<F>> for Rotor<F> {
    type Output = Multivector<F>;
    fn sub(self: Rotor<F>, b: Trivector<F>) -> Multivector<F> {
        Multivector::new(self.scalar(), Vector::zero(), self.bivector(), -b)
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Rotor<F>, Trivector<F>);

// Multivector-Multivector
impl<F: Float> Sub for Multivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Multivector<F>, b: Multivector<F>) -> Multivector<F> {
        let scalar = self.scalar() - b.scalar();
        let vector = self.vector() - b.vector();
        let bivector = self.bivector() - b.bivector();
        let trivector = self.trivector() - b.trivector();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Multivector<F>, Multivector<F>);

// Multivector-Rotor
// \[ A-R\]
impl<F: Float> Sub<Rotor<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn sub(self: Multivector<F>, b: Rotor<F>) -> Multivector<F> {
        Multivector::new(
            -b.scalar(),
            Vector::zero(),
            -b.bivector(),
            Trivector::zero(),
        ) + self
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Rotor<F>, Multivector<F>);

// Rotor-Multivector
// \[ R-B\]
impl<F: Float> Sub<Multivector<F>> for Rotor<F> {
    type Output = Multivector<F>;
    fn sub(self: Rotor<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(
            self.scalar(),
            Vector::zero(),
            self.bivector(),
            Trivector::zero(),
        ) - b
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Multivector<F>, Rotor<F>);

impl<F: Float> Sub for Rotor<F> {
    type Output = Multivector<F>;
    fn sub(self: Rotor<F>, b: Rotor<F>) -> Multivector<F> {
        let scalar = self.scalar() - b.scalar();
        let bivector = self.bivector() - b.bivector();
        Multivector::new(scalar, Vector::zero(), bivector, Trivector::zero())
    }
}
forward_ref_binop!(impl<F:Float> Sub, sub for Rotor<F>, Rotor<F>);
