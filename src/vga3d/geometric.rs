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
    bivector::Bivector, multivector::Multivector, rotor::Rotor, scalar::Scalar,
    trivector::Trivector, vector::Vector,
};

use num_traits::Float;

use crate::forward_ref_binop;

use core::ops::{BitAnd, BitOr, Div, Mul};

// Geometric Product
/// Scalar-Vector
impl<F: Float> Mul for Scalar<F> {
    type Output = F;
    fn mul(self, b: Scalar<F>) -> F {
        self.0 * b.0
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Scalar<F>, Scalar<F>);

/// Scalar-Vector
impl<F: Float> Mul<Vector<F>> for Scalar<F> {
    type Output = Vector<F>;
    fn mul(self, b: Vector<F>) -> Vector<F> {
        let e1 = b.e1() * self.0;
        let e2 = b.e2() * self.0;
        let e3 = b.e3() * self.0;
        Vector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Scalar<F>, Vector<F>);

/// Vector-Scalar
impl<F: Float> Mul<Scalar<F>> for Vector<F> {
    type Output = Vector<F>;
    fn mul(self, b: Scalar<F>) -> Vector<F> {
        let e1 = self.e1() * b.0;
        let e2 = self.e2() * b.0;
        let e3 = self.e3() * b.0;
        Vector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Vector<F>, Scalar<F>);

/// Scalar-Bivector
impl<F: Float> Mul<Bivector<F>> for Scalar<F> {
    type Output = Bivector<F>;
    fn mul(self, b: Bivector<F>) -> Bivector<F> {
        let e12 = self.0 * b.e12();
        let e31 = self.0 * b.e31();
        let e23 = self.0 * b.e23();
        Bivector::new(e12, e31, e23)
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Scalar<F>, Bivector<F>);

// Bivector-Scalar
impl<F: Float> Mul<Scalar<F>> for Bivector<F> {
    type Output = Bivector<F>;
    fn mul(self, b: Scalar<F>) -> Bivector<F> {
        Bivector::new(self.e12() * b.0, self.e31() * b.0, self.e23() * b.0)
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Bivector<F>, Scalar<F>);

// Scalar-Trivector
impl<F: Float> Mul<Trivector<F>> for Scalar<F> {
    type Output = Trivector<F>;
    fn mul(self, b: Trivector<F>) -> Trivector<F> {
        let e123 = self.0 * b.e123();
        Trivector::new(e123)
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Scalar<F>,Trivector<F>);

// Trivector-Scalar
impl<F: Float> Mul<Scalar<F>> for Trivector<F> {
    type Output = Trivector<F>;
    fn mul(self, b: Scalar<F>) -> Trivector<F> {
        let e123 = self.e123() * b.0;
        Trivector::new(e123)
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Trivector<F>, Scalar<F>);

// Scalar-Multivector
impl<F: Float> Mul<Multivector<F>> for Scalar<F> {
    type Output = Multivector<F>;
    fn mul(self, b: Multivector<F>) -> Multivector<F> {
        let scalar = self.0 * b.scalar();
        let vector = self * b.vector();
        let bivector = self * b.bivector();
        let trivector = self * b.trivector();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Scalar<F>, Multivector<F>);

// Multivector-Scalar
impl<F: Float> Mul<Scalar<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn mul(self, b: Scalar<F>) -> Multivector<F> {
        let scalar = self.scalar() * b.0;
        let vector = self.vector() * b;
        let bivector = self.bivector() * b;
        let trivector = self.trivector() * b;
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Multivector<F>, Scalar<F>);

// Scalar-Rotor
// \[ s R\]
impl<F: Float> Mul<Rotor<F>> for Scalar<F> {
    type Output = Multivector<F>;
    fn mul(self, b: Rotor<F>) -> Multivector<F> {
        Multivector::new(
            self * Scalar(b.scalar()),
            Vector::zero(),
            self * b.bivector(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Rotor<F>, Scalar<F>);

//Rotor-Scalar
// \[ R b\]
impl<F: Float> Mul<Scalar<F>> for Rotor<F> {
    type Output = Multivector<F>;
    fn mul(self, b: Scalar<F>) -> Multivector<F> {
        Multivector::new(
            self.scalar() * b.0,
            Vector::zero(),
            self.bivector() * b,
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Scalar<F>, Rotor<F>);

/// Vector-Vector
/// $$ \vec{a} \vec{b} = \vec{a} \cdot \vec{b} +   \vec{a} \wedge \vec{b} $$
impl<F: Float> Mul for Vector<F> {
    type Output = Multivector<F>;
    fn mul(self, b: Vector<F>) -> Multivector<F> {
        let scalar = self.e1() * b.e1() + self.e2() * b.e2() + self.e3() * b.e3();
        let vector = Vector::zero();
        let bivector = Bivector::new(
            self.e1() * b.e2() - self.e2() * b.e1(),
            self.e3() * b.e1() - self.e1() * b.e3(),
            self.e2() * b.e3() - self.e3() * b.e2(),
        );
        let trivector = Trivector::zero();
        Multivector::new(scalar, vector, bivector, trivector)
        // let scalar = self | b;
        // let vector = Vector::zero();
        // let bivector = self ^ b;
        // let trivector = Trivector<F>::zero();
        // Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Vector<F>, Vector<F>);

/// Vector-Bivector
/// $$ \vec{a}\overset\Rightarrow{b} $$
impl<F: Float> Mul<Bivector<F>> for Vector<F> {
    type Output = Multivector<F>;
    fn mul(self: Vector<F>, b: Bivector<F>) -> Multivector<F> {
        let vector = Vector::new(
            -self.e2() * b.e12() + self.e3() * b.e31(),
            self.e1() * b.e12() - self.e3() * b.e23(),
            -self.e1() * b.e31() + self.e2() * b.e23(),
        );
        let trivector =
            Trivector::new(self.e3() * b.e12() + self.e2() * b.e31() + self.e1() * b.e23());
        Multivector::new(F::zero(), vector, Bivector::zero(), trivector)
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Bivector<F>, Vector<F>);

// Bivector-Vector
// \[ \overset\Rightarrow{a}\vec{b}\]
impl<F: Float> Mul<Vector<F>> for Bivector<F> {
    type Output = Multivector<F>;
    fn mul(self: Bivector<F>, b: Vector<F>) -> Multivector<F> {
        let vector = Vector::new(
            self.e12() * b.e2() - self.e31() * b.e3(),
            -self.e12() * b.e1() + self.e23() * b.e3(),
            self.e31() * b.e1() - self.e23() * b.e2(),
        );
        let trivector =
            Trivector::new(self.e23() * b.e1() + self.e31() * b.e2() + self.e12() * b.e3());
        Multivector::new(F::zero(), vector, Bivector::zero(), trivector)
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Vector<F>, Bivector<F>);

// Vector-Trivector
// \[ \vec{a}\overset\Rrightarrow{b}\]
impl<F: Float> Mul<Trivector<F>> for Vector<F> {
    type Output = Bivector<F>;
    fn mul(self: Vector<F>, b: Trivector<F>) -> Bivector<F> {
        Bivector::new(
            self.e3() * b.e123(),
            self.e2() * b.e123(),
            self.e1() * b.e123(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Trivector<F>, Vector<F>);

// Trivector-Vector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl<F: Float> Mul<Vector<F>> for Trivector<F> {
    type Output = Bivector<F>;
    fn mul(self: Trivector<F>, b: Vector<F>) -> Bivector<F> {
        Bivector::new(
            self.e123() * b.e3(),
            self.e123() * b.e2(),
            self.e123() * b.e1(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Vector<F>, Trivector<F>);

// Vector-Multivector
// \[ \vec{a}B\]
impl<F: Float> Mul<Multivector<F>> for Vector<F> {
    type Output = Multivector<F>;
    fn mul(self: Vector<F>, b: Multivector<F>) -> Multivector<F> {
        let scalar = self.e1() * b.e1() + self.e2() * b.e2() + self.e3() * b.e3();
        let vector = Vector::new(
            self.e1() * b.scalar() - self.e2() * b.e12() + self.e3() * b.e31(),
            self.e2() * b.scalar() + self.e1() * b.e12() - self.e3() * b.e23(),
            self.e3() * b.scalar() - self.e1() * b.e31() + self.e2() * b.e23(),
        );
        let bivector = Bivector::new(
            self.e1() * b.e2() - self.e2() * b.e1() + self.e3() * b.e123(),
            self.e3() * b.e1() - self.e1() * b.e3() + self.e2() * b.e123(),
            self.e2() * b.e3() - self.e3() * b.e2() + self.e1() * b.e123(),
        );
        let trivector =
            Trivector::new(self.e3() * b.e12() + self.e2() * b.e31() + self.e1() * b.e23());
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Multivector<F>, Vector<F>);

// Multivector-Vector
impl<F: Float> Mul<Vector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn mul(self: Multivector<F>, b: Vector<F>) -> Multivector<F> {
        let scalar_vec = Scalar(self.scalar()) * b;
        let vec_vec = self.vector() * b;
        let bivec_vec = self.bivector() * b;
        let trivec_vec = self.trivector() * b;

        let scalar = F::zero();
        let vector = scalar_vec;
        let bivector = trivec_vec;
        let trivector = Trivector::zero();

        Multivector::new(scalar, vector, bivector, trivector) + vec_vec + bivec_vec
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Vector<F>, Multivector<F>);

// Vector-Rotor
// \[ \vec{a}R\]
impl<F: Float> Mul<Rotor<F>> for Vector<F> {
    type Output = Multivector<F>;
    fn mul(self: Vector<F>, b: Rotor<F>) -> Multivector<F> {
        let scalar = F::zero();
        let vector = Vector::new(
            self.e1() * b.scalar() - self.e2() * b.e12() + self.e3() * b.e31(),
            self.e2() * b.scalar() + self.e1() * b.e12() - self.e3() * b.e23(),
            self.e3() * b.scalar() - self.e1() * b.e31() + self.e2() * b.e23(),
        );
        let bivector = Bivector::zero();
        let trivector =
            Trivector::new(self.e3() * b.e12() + self.e2() * b.e31() + self.e1() * b.e23());
        Multivector::new(scalar, vector, bivector, trivector)
    }
}

forward_ref_binop!(impl<F:Float> Mul, mul for Rotor<F>, Vector<F>);

// Rotor-Vector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl<F: Float> Mul<Vector<F>> for Rotor<F> {
    type Output = Multivector<F>;
    fn mul(self: Rotor<F>, b: Vector<F>) -> Multivector<F> {
        let scalar = F::zero();
        let vector = Vector::new(
            self.scalar() * b.e1() + self.e12() * b.e2() - self.e31() * b.e3(),
            self.scalar() * b.e2() - self.e12() * b.e1() + self.e23() * b.e3(),
            self.scalar() * b.e3() + self.e31() * b.e1() - self.e23() * b.e2(),
        );
        let bivector = Bivector::zero();
        let trivector =
            Trivector::new(self.e23() * b.e1() + self.e31() * b.e2() + self.e12() * b.e3());
        Multivector::new(scalar, vector, bivector, trivector)
        // let scalar = F::zero();
        // let vector = b * self.scalar();
        // let bivector = Bivector<F>::zero();
        // let trivector = Trivector<F>::zero();
        // Multivector<F>::new(scalar, vector, bivector, trivector) + self.bivector() * b
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Vector<F>, Rotor<F>);

// Bivector-Bivector
// \[ \overset\Rightarrow{a} \overset\Rightarrow{b} = \overset\Rightarrow{a} \cdot \overset\Rightarrow{b} +  \overset\Rightarrow{a} \times \overset\Rightarrow{b} + \overset\Rightarrow{a} \wedge \overset\Rightarrow{b} \]
impl<F: Float> Mul for Bivector<F> {
    type Output = Multivector<F>;
    fn mul(self: Bivector<F>, b: Bivector<F>) -> Multivector<F> {
        Multivector::new(
            -self.e12() * b.e12() - self.e31() * b.e31() - self.e23() * b.e23(),
            Vector::zero(),
            Bivector::new(
                self.e31() * b.e23() - self.e23() * b.e31(),
                self.e23() * b.e12() - self.e12() * b.e23(),
                self.e12() * b.e31() - self.e31() * b.e12(),
            ),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Bivector<F>, Bivector<F>);

// Bivector-Trivector
// \[ \overset\Rightarrow{a}\overset\Rrightarrow{b}\]
impl<F: Float> Mul<Trivector<F>> for Bivector<F> {
    type Output = Vector<F>;
    fn mul(self: Bivector<F>, b: Trivector<F>) -> Vector<F> {
        Vector::new(
            -self.e23() * b.e123(),
            -self.e31() * b.e123(),
            -self.e12() * b.e123(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Bivector<F>, Trivector<F>);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a}\overset\Rightarrow{b}\]
impl<F: Float> Mul<Bivector<F>> for Trivector<F> {
    type Output = Vector<F>;
    fn mul(self: Trivector<F>, b: Bivector<F>) -> Vector<F> {
        Vector::new(
            -self.e123() * b.e23(),
            -self.e123() * b.e31(),
            -self.e123() * b.e12(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Trivector<F>, Bivector<F>);

// Bivector-Multivector
impl<F: Float> Mul<Multivector<F>> for Bivector<F> {
    type Output = Multivector<F>;
    fn mul(self: Bivector<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(
            -self.e12() * b.e12() - self.e31() * b.e31() - self.e23() * b.e23(),
            Vector::new(
                self.e12() * b.e2() - self.e31() * b.e3() - self.e23() * b.e123(),
                -self.e12() * b.e1() + self.e23() * b.e3() - self.e31() * b.e123(),
                self.e31() * b.e1() - self.e23() * b.e2() - self.e12() * b.e123(),
            ),
            Bivector::new(
                self.e12() * b.scalar() + self.e31() * b.e23() - self.e23() * b.e31(),
                self.e31() * b.scalar() + self.e23() * b.e12() - self.e12() * b.e23(),
                self.e23() * b.scalar() + self.e12() * b.e31() - self.e31() * b.e12(),
            ),
            Trivector::new(self.e23() * b.e1() + self.e31() * b.e2() + self.e12() * b.e3()),
        )

        // Multivector::new(scalar, vector, bivector, trivector) + bivec_vec + bivec_bivec
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Bivector<F>, Multivector<F>);

// Multivector-Bivector
impl<F: Float> Mul<Bivector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn mul(self: Multivector<F>, b: Bivector<F>) -> Multivector<F> {
        Multivector::new(
            -self.e12() * b.e12() - self.e31() * b.e31() - self.e23() * b.e23(),
            Vector::new(
                -self.e2() * b.e12() + self.e3() * b.e31() - self.e123() * b.e23(),
                self.e1() * b.e12() - self.e3() * b.e23() - self.e123() * b.e31(),
                -self.e1() * b.e31() + self.e2() * b.e23() - self.e123() * b.e12(),
            ),
            Bivector::new(
                self.scalar() * b.e12() + self.e31() * b.e23() - self.e23() * b.e31(),
                self.scalar() * b.e31() + self.e23() * b.e12() - self.e12() * b.e23(),
                self.scalar() * b.e23() + self.e12() * b.e31() - self.e31() * b.e12(),
            ),
            Trivector::new(self.e3() * b.e23() + self.e2() * b.e31() + self.e1() * b.e23()),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Multivector<F>, Bivector<F>);

// Bivector-Rotor
// \[ \vec{a}R\]
impl<F: Float> Mul<Rotor<F>> for Bivector<F> {
    type Output = Multivector<F>;
    fn mul(self: Bivector<F>, b: Rotor<F>) -> Multivector<F> {
        Multivector::new(
            -(self.e12() * b.e12()) - (self.e31() * b.e31()) - (self.e23() * b.e23()),
            Vector::zero(),
            Bivector::new(
                (self.e12() * b.scalar()) + (self.e31() * b.e23()) - (self.e23() * b.e31()),
                (self.e31() * b.scalar()) + (self.e23() * b.e12()) - (self.e12() * b.e23()),
                (self.e23() * b.scalar()) + (self.e12() * b.e31()) - (self.e31() * b.e12()),
            ),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Bivector<F>, Rotor<F>);

// Rotor-Bivector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl<F: Float> Mul<Bivector<F>> for Rotor<F> {
    type Output = Multivector<F>;
    fn mul(self: Rotor<F>, b: Bivector<F>) -> Multivector<F> {
        Multivector::new(
            -self.e12() * b.e12() - self.e31() * b.e31() - self.e23() * b.e23(),
            Vector::zero(),
            Bivector::new(
                self.scalar() * b.e12() + self.e31() * b.e23() - self.e23() * b.e31(),
                self.scalar() * b.e31() + self.e23() * b.e12() - self.e12() * b.e23(),
                self.scalar() * b.e23() + self.e12() * b.e31() - self.e31() * b.e12(),
            ),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Rotor<F>, Bivector<F>);

// Trivector-Trivector
// \[ \overset\Rrightarrow{a}\overset\Rrightarrow{b}= \overset\Rrightarrow{a} \cdot \overset\Rrightarrow{b}\]
impl<F: Float> Mul for Trivector<F> {
    type Output = Scalar<F>;
    fn mul(self: Trivector<F>, b: Trivector<F>) -> Scalar<F> {
        Scalar(-self.e123() * b.e123())
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Trivector<F>, Trivector<F>);

// Trivector-Multivector
impl<F: Float> Mul<Multivector<F>> for Trivector<F> {
    type Output = Multivector<F>;
    fn mul(self: Trivector<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(
            -self.e123() * b.e123(),
            Vector::new(
                -self.e123() * b.e23(),
                -self.e123() * b.e31(),
                -self.e123() * b.e12(),
            ),
            Bivector::new(
                self.e123() * b.e3(),
                self.e123() * b.e2(),
                self.e123() * b.e1(),
            ),
            Trivector::new(self.e123() * b.scalar()),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Trivector<F>,Multivector<F>);

// Multivector-Trivector
impl<F: Float> Mul<Trivector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn mul(self: Multivector<F>, b: Trivector<F>) -> Multivector<F> {
        Multivector::new(
            -self.e123() * b.e123(),
            Vector::new(
                -self.e23() * b.e123(),
                -self.e31() * b.e123(),
                -self.e12() * b.e123(),
            ),
            Bivector::new(
                self.e3() * b.e123(),
                self.e2() * b.e123(),
                self.e1() * b.e123(),
            ),
            Trivector::new(self.scalar() * b.e123()),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Multivector<F>, Trivector<F>);

// Trivector-Rotor
// \[ \vec{a}R\]
impl<F: Float> Mul<Rotor<F>> for Trivector<F> {
    type Output = Multivector<F>;
    fn mul(self: Trivector<F>, b: Rotor<F>) -> Multivector<F> {
        Multivector::new(
            F::zero(),
            Vector::new(
                -self.e123() * b.e23(),
                -self.e123() * b.e31(),
                -self.e123() * b.e12(),
            ),
            Bivector::zero(),
            Trivector::new(self.e123() * b.scalar()),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Trivector<F>,Rotor<F>);

// Rotor-Trivector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl<F: Float> Mul<Trivector<F>> for Rotor<F> {
    type Output = Multivector<F>;
    fn mul(self: Rotor<F>, b: Trivector<F>) -> Multivector<F> {
        Multivector::new(
            F::zero(),
            Vector::new(
                -self.e23() * b.e123(),
                -self.e31() * b.e123(),
                -self.e12() * b.e123(),
            ),
            Bivector::zero(),
            Trivector::new(self.scalar() * b.e123()),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Rotor<F>, Trivector<F>);

// Multivector-Multivector
impl<F: Float> Mul for Multivector<F> {
    type Output = Multivector<F>;
    fn mul(self: Multivector<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(
            self.scalar() * b.scalar()
                + self.e1() * b.e1()
                + self.e2() * b.e2()
                + self.e3() * b.e3()
                - self.e12() * b.e12()
                - self.e31() * b.e31()
                - self.e23() * b.e23()
                - self.e123() * b.e123(),
            Vector::new(
                (self.scalar() * b.e1() + self.e1() * b.scalar())
                    + (self.e12() * b.e2() - self.e2() * b.e12())
                    + (self.e3() * b.e31() - self.e31() * b.e3())
                    + (-self.e23() * b.e123() - self.e123() * b.e23()),
                (self.scalar() * b.e2() + self.e2() * b.scalar())
                    + (self.e1() * b.e12() - self.e12() * b.e1())
                    + (self.e23() * b.e3() - self.e3() * b.e23())
                    + (-self.e31() * b.e123() - self.e123() * b.e31()),
                (self.scalar() * b.e3() + self.e3() * b.scalar())
                    + (self.e31() * b.e1() - self.e1() * b.e31())
                    + (self.e2() * b.e23() - self.e23() * b.e2())
                    + (-self.e12() * b.e123() - self.e123() * b.e12()),
            ),
            Bivector::new(
                (self.scalar() * b.e12() + self.e12() * b.scalar())
                    + (self.e1() * b.e2() - self.e2() * b.e1())
                    + (self.e31() * b.e23() - self.e23() * b.e31())
                    + (self.e3() * b.e123() + self.e123() * b.e3()),
                (self.scalar() * b.e31() + self.e31() * b.scalar())
                    + (self.e3() * b.e1() - self.e1() * b.e3())
                    + (self.e23() * b.e12() - self.e12() * b.e23())
                    + (self.e2() * b.e123() + self.e123() * b.e2()),
                (self.scalar() * b.e23() + self.e23() * b.scalar())
                    + (self.e2() * b.e3() - self.e3() * b.e2())
                    + (self.e12() * b.e31() - self.e31() * b.e12())
                    + (self.e1() * b.e123() + self.e123() * b.e1()),
            ),
            Trivector::new(
                self.e123() * b.scalar()
                    + self.e23() * b.e1()
                    + self.e31() * b.e2()
                    + self.e12() * b.e3()
                    + self.e3() * b.e12()
                    + self.e2() * b.e31()
                    + self.e1() * b.e23()
                    + self.scalar() * b.e123(),
            ),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Multivector<F>, Multivector<F>);

// Multivector-Rotor
// \[ \vec{a}R\]
impl<F: Float> Mul<Rotor<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn mul(self: Multivector<F>, b: Rotor<F>) -> Multivector<F> {
        Multivector::new(
            self.scalar() * b.scalar()
                - self.e12() * b.e12()
                - self.e31() * b.e31()
                - self.e23() * b.e23(),
            Vector::new(
                self.e1() * b.scalar() - self.e2() * b.e12() + self.e3() * b.e31()
                    - self.e123() * b.e23(),
                self.e2() * b.scalar() + self.e1() * b.e12()
                    - self.e3() * b.e23()
                    - self.e123() * b.e31(),
                self.e3() * b.scalar() - self.e1() * b.e31() + self.e2() * b.e23()
                    - self.e123() * b.e12(),
            ),
            Bivector::new(
                (self.scalar() * b.e12() + self.e12() * b.scalar())
                    + (self.e31() * b.e23() - self.e23() * b.e31()),
                (self.scalar() * b.e31() + self.e31() * b.scalar())
                    + (self.e23() * b.e12() - self.e12() * b.e23()),
                (self.scalar() * b.e23() + self.e23() * b.scalar())
                    + (self.e12() * b.e31() - self.e31() * b.e12()),
            ),
            Trivector::new(
                self.e123() * b.scalar()
                    + self.e3() * b.e12()
                    + self.e2() * b.e31()
                    + self.e1() * b.e23(),
            ),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Multivector<F>, Rotor<F>);

// Rotor-Multivector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl<F: Float> Mul<Multivector<F>> for Rotor<F> {
    type Output = Multivector<F>;
    fn mul(self: Rotor<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(
            self.scalar() * b.scalar()
                - self.e12() * b.e12()
                - self.e31() * b.e31()
                - self.e23() * b.e23(),
            Vector::new(
                self.scalar() * b.e1() + self.e12() * b.e2()
                    - self.e31() * b.e3()
                    - self.e23() * b.e123(),
                self.scalar() * b.e2() - self.e12() * b.e1() + self.e23() * b.e3()
                    - self.e31() * b.e123(),
                self.scalar() * b.e3() + self.e31() * b.e1()
                    - self.e23() * b.e2()
                    - self.e12() * b.e123(),
            ),
            Bivector::new(
                (self.scalar() * b.e12() + self.e12() * b.scalar())
                    + (self.e31() * b.e23() - self.e23() * b.e31()),
                (self.scalar() * b.e31() + self.e31() * b.scalar())
                    + (self.e23() * b.e12() - self.e12() * b.e23()),
                (self.scalar() * b.e23() + self.e23() * b.scalar())
                    + (self.e12() * b.e31() - self.e31() * b.e12()),
            ),
            Trivector::new(
                self.e23() * b.e1()
                    + self.e31() * b.e2()
                    + self.e12() * b.e3()
                    + self.scalar() * b.e123(),
            ),
        )
    }
}
forward_ref_binop!(impl<F:Float> Mul, mul for Rotor<F>, Multivector<F>);

// Test
#[cfg(test)]
mod geometric_product {
    use core::f32::consts::TAU;

    use crate::vga3d::VGA3DOpsRef;

    use super::*;
    use approx::assert_relative_eq;
    use nalgebra::ComplexField;
    #[test]
    fn scalar_bivector_geo() {
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        let scalar = Scalar::new(3.0);
        let res = scalar * bivector;

        assert_relative_eq!(res.e12(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 18.0, max_relative = 0.000001);
    }

    fn bivector_scalar_geo() {
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        let scalar = Scalar::new(3.0);
        let res = bivector * scalar;

        assert_relative_eq!(res.e12(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 18.0, max_relative = 0.000001);
    }

    fn vector_bivector_geo() {
        // 3e1+5e2+4e3
        let vector = Vector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
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
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        // 3e1+5e2+4e3
        let vector = Vector::new(3.0, 5.0, 4.0);
        let mvec = bivector * vector;
        // 6e1​+18e2​−27e3​+31e123​
        assert_relative_eq!(mvec.e1(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e2(), 18.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e3(), -27.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e123(), 31.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_bivector_geo() {
        // 3e12+5e31+4e23
        let bivector1 = Bivector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let bivector2 = Bivector::new(2.0, 1.0, 6.0);
        let mvec = bivector1 * bivector2;
        // −35+26e12​-10e31​−7e23​
        assert_relative_eq!(mvec.scalar(), -35.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e12(), 26.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e31(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e23(), -7.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_norm() {
        let bivector = Bivector::new(3.0, 5.0, 4.0);
        // -3e12-5e31-4e23
        // let bivector_reverse = bivector.reverse();
        // assert_relative_eq!(bivector_reverse.e12(), -3.0, max_relative = 0.000001);
        // assert_relative_eq!(bivector_reverse.e31(), -5.0, max_relative = 0.000001);
        // assert_relative_eq!(bivector_reverse.e23(), -4.0, max_relative = 0.000001);

        assert_relative_eq!(
            bivector.norm().scalar(),
            7.0710678118654755,
            max_relative = 0.000001
        );
    }

    #[test]
    fn rotor_vector_geo() {
        let angle = TAU / 4.0;
        let rotation_plane = Bivector::new(4.0, 2.0, -3.0);
        let rotor = Rotor::try_new_from_half_angle_plane(angle / 2.0, rotation_plane);
        // 2e12+e31+6e23
        let vector = Vector::new(2.0, 1.0, 6.0);
        let rotor = match rotor {
            Some(rotor) => rotor,
            None => Rotor::identity(),
        };

        // 0.7071+0.5252e12+0.2626e31-0.3939e23
        let res_impl = vector * rotor;
        let res_new = Multivector::new(
            0.0,
            Vector::new(
                vector.e1() * rotor.scalar() - vector.e2() * rotor.e12()
                    + vector.e3() * rotor.e31(),
                vector.e2() * rotor.scalar() + vector.e1() * rotor.e12()
                    - vector.e3() * rotor.e23(),
                vector.e3() * rotor.scalar() - vector.e1() * rotor.e31()
                    + vector.e2() * rotor.e23(),
            ),
            Bivector::zero(),
            Trivector::new(
                vector.e3() * rotor.e12() + vector.e2() * rotor.e31() + vector.e1() * rotor.e23(),
            ),
        );

        assert_relative_eq!(res_impl.e1(), res_new.e1(), max_relative = 0.000001);
        assert_relative_eq!(res_impl.e2(), res_new.e2(), max_relative = 0.000001);
        assert_relative_eq!(res_impl.e3(), res_new.e3(), max_relative = 0.000001);
    }

    #[test]
    fn rotor_bivector_geo() {
        let angle = TAU / 4.0;
        let rotation_plane = Bivector::new(4.0, 2.0, -3.0);
        let rotor = Rotor::try_new_from_half_angle_plane(angle / 2.0, rotation_plane);
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        // 0.7071+0.5252e12+0.2626e31-0.3939e23
        let res = match rotor {
            Some(rotor) => rotor * bivector,
            None => Multivector::zero(),
        };

        // 1.0504512787+3.3838100433e12​-3.2320864201e31​+4.2426404953e23
        assert_relative_eq!(res.scalar(), 1.0504512787, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 3.3838100433, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), -3.2320864201, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 4.2426404953, max_relative = 0.000001);
    }

    #[test]
    fn bivector_rotor_geo() {
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        // 0.7071+0.5252e12+0.2626e31-0.3939e23
        let angle = TAU / 4.0;
        let rotation_plane = Bivector::new(4.0, 2.0, -3.0);
        let rotor = Rotor::try_new_from_half_angle_plane(angle / 2.0, rotation_plane);
        let mvec = match rotor {
            Some(rotor) => bivector * rotor,
            None => Multivector::zero(),
        };

        // 1.0504512787−0.5553830266e12​+4.646299839e31​+4.2426404953e23
        assert_relative_eq!(mvec.scalar(), 1.0504, max_relative = 0.001);
        assert_relative_eq!(mvec.e12(), -0.5553, max_relative = 0.001);
        assert_relative_eq!(mvec.e31(), 4.6461, max_relative = 0.001);
        assert_relative_eq!(mvec.e23(), 4.2425, max_relative = 0.001);
    }

    #[test]
    fn trivec_trivec_geo() {
        let trivector1 = Trivector::new(3.0);
        let trivector2 = Trivector::new(6.0);
        let res = trivector1 * trivector2;
        assert_relative_eq!(res.scalar(), -18.0, max_relative = 0.000001);
    }

    #[test]
    fn mvec_mvec_mul() {
        let mvec1 = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        let mvec2 = Multivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
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
    fn negetive_mvec_mvec_geo() {
        // let mvec1 = GaMultivector::new_mvec(-6.0, -8.0, -4.0, -1.0, -6.0, -4.0, -8.0, -5.0);
        let mvec1 = Multivector::new_components(-4.0, -1.0, -3.0, -2.0, -9.0, -6.0, -3.0, -10.0);
        let mvec2 = Multivector::new_components(-4.0, -2.0, -4.0, -9.0, -2.0, -1.0, -7.0, -1.0);
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

    #[test]
    fn mvec_rotor_geo() {
        // ( 0 + 0.55708605e12 + 0.3713907e31 + 0.7427814e23)
        let scalar = TAU / 2.0;
        let bivector = Bivector::new(6.0 / 10.770, 4.0 / 10.770, 8.0 / 10.770);
        let rotor = Rotor::try_new_from_half_angle_plane(scalar / 2.0, bivector);
        let rotor = match rotor {
            Some(rotor) => rotor,
            None => Rotor {
                scalar: Scalar(1.0),
                bivector: Bivector::zero(),
            },
        };

        assert_relative_eq!(rotor.scalar(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(rotor.e12(), 0.55708605, max_relative = 0.000001);
        assert_relative_eq!(rotor.e31(), 0.3713907, max_relative = 0.000001);
        assert_relative_eq!(rotor.e23(), 0.7427814, max_relative = 0.000001);

        // ( 5.0 + 8.0e1 + 7.0e2 + 3.0e3 + 2.0e12 + 8.0e31 + 2.0e23 + 1.0e123 )
        let mvec = Multivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);

        // -5.5708603859 -3.5282115936e1​ +1.8569535017e2​ +1.6712582111e3 ​+7.9848999977e12​ +1.4855628014e31​ +10.2132444382e123
        let mvec_res = mvec * rotor;

        assert_relative_eq!(mvec_res.scalar(), -5.5708603859, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), -3.5282115936, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), 1.8569535017, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), 1.6712582111, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 7.9848999977, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 1.4855628014, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 10.2132444382, max_relative = 0.000001);
    }

    #[test]
    fn rotor_mvec_geo() {
        // ( 0 + 0.55708605e12 + 0.3713907e31 + 0.7427814e23)
        let scalar = TAU / 2.0;
        let bivector = Bivector::new(6.0 / 10.770, 4.0 / 10.770, 8.0 / 10.770);
        let rotor = Rotor::try_new_from_half_angle_plane(scalar / 2.0, bivector);
        let rotor = match rotor {
            Some(rotor) => rotor,
            None => Rotor {
                scalar: Scalar(1.0),
                bivector: Bivector::zero(),
            },
        };

        assert_relative_eq!(rotor.scalar(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(rotor.e12(), 0.55708605, max_relative = 0.000001);
        assert_relative_eq!(rotor.e31(), 0.3713907, max_relative = 0.000001);
        assert_relative_eq!(rotor.e23(), 0.7427814, max_relative = 0.000001);

        // ( 5.0 + 8.0e1 + 7.0e2 + 3.0e3 + 2.0e12 + 8.0e31 + 2.0e23 + 1.0e123 )
        let mvec2 = Multivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);

        // -5.5708603859+2.0426487923e1​-2.5997347832e2​-2.7854301929e3​-2.4140396118e12​+2.228344202e31​+7.4278140068e23​+10.2132444382e123
        let mvec_res = rotor * mvec2;

        assert_relative_eq!(mvec_res.scalar(), -5.5708603859, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), 2.0426487923, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), -2.5997347832, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), -2.7854301929, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), -2.4140396118, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 2.228344202, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 7.4278140068, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 10.2132444382, max_relative = 0.000001);
    }
}
