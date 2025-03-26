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
    bivector::Bivector, multivector::Multivector, rotor::Rotor, trivector::Trivector,
    vector::Vector,
};

use crate::forward_ref_binop;
use core::ops::{BitAnd, BitOr, Div, Mul};

// Geometric Product
/// Scalar-Vector
impl Mul<Vector> for f32 {
    type Output = Vector;
    fn mul(self, b: Vector) -> Vector {
        let e1 = b.e1() * self;
        let e2 = b.e2() * self;
        let e3 = b.e3() * self;
        Vector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl Mul, mul for f32, Vector);

/// Vector-Scalar
impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, b: f32) -> Vector {
        let e1 = self.e1() * b;
        let e2 = self.e2() * b;
        let e3 = self.e3() * b;
        Vector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl Mul, mul for Vector,f32);

/// Scalar-Bivector
impl Mul<Bivector> for f32 {
    type Output = Bivector;
    fn mul(self, b: Bivector) -> Bivector {
        let e12 = self * b.e12();
        let e31 = self * b.e31();
        let e23 = self * b.e23();
        Bivector::new(e12, e31, e23)
    }
}
forward_ref_binop!(impl Mul, mul for f32, Bivector);

// Bivector-Scalar
impl Mul<f32> for Bivector {
    type Output = Bivector;
    fn mul(self, b: f32) -> Bivector {
        Bivector::new(self.e12() * b, self.e31() * b, self.e23() * b)
    }
}
forward_ref_binop!(impl Mul, mul for Bivector,f32);

// Scalar-Trivector
impl Mul<Trivector> for f32 {
    type Output = Trivector;
    fn mul(self, b: Trivector) -> Trivector {
        let e123 = self * b.e123();
        Trivector::new(e123)
    }
}
forward_ref_binop!(impl Mul, mul for f32,Trivector);

// Trivector-Scalar
impl Mul<f32> for Trivector {
    type Output = Trivector;
    fn mul(self, b: f32) -> Trivector {
        let e123 = self.e123() * b;
        Trivector::new(e123)
    }
}
forward_ref_binop!(impl Mul, mul for Trivector, f32);

// Scalar-Multivector
impl Mul<Multivector> for f32 {
    type Output = Multivector;
    fn mul(self, b: Multivector) -> Multivector {
        let scalar = self * b.scalar();
        let vector = self * b.vector();
        let bivector = self * b.bivector();
        let trivector = self * b.trivector();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Mul, mul for f32, Multivector);

// Multivector-Scalar
impl Mul<f32> for Multivector {
    type Output = Multivector;
    fn mul(self, b: f32) -> Multivector {
        let scalar = self.scalar() * b;
        let vector = self.vector() * b;
        let bivector = self.bivector() * b;
        let trivector = self.trivector() * b;
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Mul, mul for Multivector, f32);

// Scalar-Rotor
// \[ s R\]
impl Mul<Rotor> for f32 {
    type Output = Multivector;
    fn mul(self: f32, b: Rotor) -> Multivector {
        Multivector::new(
            self * b.scalar(),
            Vector::zero(),
            self * b.bivector(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl Mul, mul for Rotor, f32);

//Rotor-Scalar
// \[ R b\]
impl Mul<f32> for Rotor {
    type Output = Multivector;
    fn mul(self: Rotor, b: f32) -> Multivector {
        Multivector::new(
            self.scalar() * b,
            Vector::zero(),
            self.bivector() * b,
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl Mul, mul for f32, Rotor);

/// Vector-Vector
/// $$ \vec{a} \vec{b} = \vec{a} \cdot \vec{b} +   \vec{a} \wedge \vec{b} $$
impl Mul for Vector {
    type Output = Multivector;
    fn mul(self: Vector, b: Vector) -> Multivector {
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
        // let trivector = Trivector::zero();
        // Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl Mul, mul for Vector, Vector);

/// Vector-Bivector
/// $$ \vec{a}\overset\Rightarrow{b} $$
impl Mul<Bivector> for Vector {
    type Output = Multivector;
    fn mul(self: Vector, b: Bivector) -> Multivector {
        let vector = Vector::new(
            -self.e2() * b.e12() + self.e3() * b.e31(),
            self.e1() * b.e12() - self.e3() * b.e23(),
            -self.e1() * b.e31() + self.e2() * b.e23(),
        );
        let trivector =
            Trivector::new(self.e3() * b.e12() + self.e2() * b.e31() + self.e1() * b.e23());
        Multivector::new(0.0, vector, Bivector::zero(), trivector)
    }
}
forward_ref_binop!(impl Mul, mul for Bivector, Vector);

// Bivector-Vector
// \[ \overset\Rightarrow{a}\vec{b}\]
impl Mul<Vector> for Bivector {
    type Output = Multivector;
    fn mul(self: Bivector, b: Vector) -> Multivector {
        let vector = Vector::new(
            self.e12() * b.e2() - self.e31() * b.e3(),
            -self.e12() * b.e1() + self.e23() * b.e3(),
            self.e31() * b.e1() - self.e23() * b.e2(),
        );
        let trivector =
            Trivector::new(self.e23() * b.e1() + self.e31() * b.e2() + self.e12() * b.e3());
        Multivector::new(0.0, vector, Bivector::zero(), trivector)
    }
}
forward_ref_binop!(impl Mul, mul for Vector, Bivector);

// Vector-Trivector
// \[ \vec{a}\overset\Rrightarrow{b}\]
impl Mul<Trivector> for Vector {
    type Output = Bivector;
    fn mul(self: Vector, b: Trivector) -> Bivector {
        Bivector::new(
            self.e3() * b.e123(),
            self.e2() * b.e123(),
            self.e1() * b.e123(),
        )
    }
}
forward_ref_binop!(impl Mul, mul for Trivector, Vector);

// Trivector-Vector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl Mul<Vector> for Trivector {
    type Output = Bivector;
    fn mul(self: Trivector, b: Vector) -> Bivector {
        Bivector::new(
            self.e123() * b.e3(),
            self.e123() * b.e2(),
            self.e123() * b.e1(),
        )
    }
}
forward_ref_binop!(impl Mul, mul for Vector, Trivector);

// Vector-Multivector
// \[ \vec{a}B\]
impl Mul<Multivector> for Vector {
    type Output = Multivector;
    fn mul(self: Vector, b: Multivector) -> Multivector {
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
forward_ref_binop!(impl Mul, mul for Multivector, Vector);

// Multivector-Vector
impl Mul<Vector> for Multivector {
    type Output = Multivector;
    fn mul(self: Multivector, b: Vector) -> Multivector {
        let scalar_vec = self.scalar() * b;
        let vec_vec = self.vector() * b;
        let bivec_vec = self.bivector() * b;
        let trivec_vec = self.trivector() * b;

        let scalar = 0.0;
        let vector = scalar_vec;
        let bivector = trivec_vec;
        let trivector = Trivector::zero();

        Multivector::new(scalar, vector, bivector, trivector) + vec_vec + bivec_vec
    }
}
forward_ref_binop!(impl Mul, mul for Vector, Multivector);

// Vector-Rotor
// \[ \vec{a}R\]
impl Mul<Rotor> for Vector {
    type Output = Multivector;
    fn mul(self: Vector, b: Rotor) -> Multivector {
        let scalar = 0.0;
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

forward_ref_binop!(impl Mul, mul for Rotor, Vector);

// Rotor-Vector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl Mul<Vector> for Rotor {
    type Output = Multivector;
    fn mul(self: Rotor, b: Vector) -> Multivector {
        let scalar = 0.0;
        let vector = Vector::new(
            self.scalar() * b.e1() + self.e12() * b.e2() - self.e31() * b.e3(),
            self.scalar() * b.e2() - self.e12() * b.e1() + self.e23() * b.e3(),
            self.scalar() * b.e3() + self.e31() * b.e1() - self.e23() * b.e2(),
        );
        let bivector = Bivector::zero();
        let trivector =
            Trivector::new(self.e23() * b.e1() + self.e31() * b.e2() + self.e12() * b.e3());
        Multivector::new(scalar, vector, bivector, trivector)
        // let scalar = 0.0;
        // let vector = b * self.scalar();
        // let bivector = Bivector::zero();
        // let trivector = Trivector::zero();
        // Multivector::new(scalar, vector, bivector, trivector) + self.bivector() * b
    }
}
forward_ref_binop!(impl Mul, mul for Vector, Rotor);

// Bivector-Bivector
// \[ \overset\Rightarrow{a} \overset\Rightarrow{b} = \overset\Rightarrow{a} \cdot \overset\Rightarrow{b} +  \overset\Rightarrow{a} \times \overset\Rightarrow{b} + \overset\Rightarrow{a} \wedge \overset\Rightarrow{b} \]
impl Mul for Bivector {
    type Output = Multivector;
    fn mul(self: Bivector, b: Bivector) -> Multivector {
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
forward_ref_binop!(impl Mul, mul for Bivector, Bivector);

// Bivector-Trivector
// \[ \overset\Rightarrow{a}\overset\Rrightarrow{b}\]
impl Mul<Trivector> for Bivector {
    type Output = Vector;
    fn mul(self: Bivector, b: Trivector) -> Vector {
        Vector::new(
            -self.e23() * b.e123(),
            -self.e31() * b.e123(),
            -self.e12() * b.e123(),
        )
    }
}
forward_ref_binop!(impl Mul, mul for Bivector, Trivector);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a}\overset\Rightarrow{b}\]
impl Mul<Bivector> for Trivector {
    type Output = Vector;
    fn mul(self: Trivector, b: Bivector) -> Vector {
        Vector::new(
            -self.e123() * b.e23(),
            -self.e123() * b.e31(),
            -self.e123() * b.e12(),
        )
    }
}
forward_ref_binop!(impl Mul, mul for Trivector, Bivector);

// Bivector-Multivector
impl Mul<Multivector> for Bivector {
    type Output = Multivector;
    fn mul(self: Bivector, b: Multivector) -> Multivector {
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
forward_ref_binop!(impl Mul, mul for Bivector, Multivector);

// Multivector-Bivector
impl Mul<Bivector> for Multivector {
    type Output = Multivector;
    fn mul(self: Multivector, b: Bivector) -> Multivector {
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
forward_ref_binop!(impl Mul, mul for Multivector, Bivector);

// Bivector-Rotor
// \[ \vec{a}R\]
impl Mul<Rotor> for Bivector {
    type Output = Multivector;
    fn mul(self: Bivector, b: Rotor) -> Multivector {
        Multivector::new(
            -self.e12() * b.e12() - self.e31() * b.e31() - self.e23() * b.e23(),
            Vector::zero(),
            Bivector::new(
                self.e12() * b.scalar() + self.e31() * b.e23() - self.e23() * b.e31(),
                self.e31() * b.scalar() + self.e23() * b.e12() - self.e12() * b.e23(),
                self.e23() * b.scalar() + self.e12() * b.e31() - self.e31() * b.e12(),
            ),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl Mul, mul for Bivector, Rotor);

// Rotor-Bivector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl Mul<Bivector> for Rotor {
    type Output = Multivector;
    fn mul(self: Rotor, b: Bivector) -> Multivector {
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
forward_ref_binop!(impl Mul, mul for Rotor, Bivector);

// Trivector-Trivector
// \[ \overset\Rrightarrow{a}\overset\Rrightarrow{b}= \overset\Rrightarrow{a} \cdot \overset\Rrightarrow{b}\]
impl Mul for Trivector {
    type Output = f32;
    fn mul(self: Trivector, b: Trivector) -> f32 {
        -self.e123() * b.e123()
    }
}
forward_ref_binop!(impl Mul, mul for Trivector, Trivector);

// Trivector-Multivector
impl Mul<Multivector> for Trivector {
    type Output = Multivector;
    fn mul(self: Trivector, b: Multivector) -> Multivector {
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
forward_ref_binop!(impl Mul, mul for Trivector,Multivector);

// Multivector-Trivector
impl Mul<Trivector> for Multivector {
    type Output = Multivector;
    fn mul(self: Multivector, b: Trivector) -> Multivector {
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
forward_ref_binop!(impl Mul, mul for Multivector, Trivector);

// Trivector-Rotor
// \[ \vec{a}R\]
impl Mul<Rotor> for Trivector {
    type Output = Multivector;
    fn mul(self: Trivector, b: Rotor) -> Multivector {
        Multivector::new(
            0.0,
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
forward_ref_binop!(impl Mul, mul for Trivector,Rotor);

// Rotor-Trivector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl Mul<Trivector> for Rotor {
    type Output = Multivector;
    fn mul(self: Rotor, b: Trivector) -> Multivector {
        Multivector::new(
            0.0,
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
forward_ref_binop!(impl Mul, mul for Rotor, Trivector);

// Multivector-Multivector
impl Mul for Multivector {
    type Output = Multivector;
    fn mul(self: Multivector, b: Multivector) -> Multivector {
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
forward_ref_binop!(impl Mul, mul for Multivector, Multivector);

// Multivector-Rotor
// \[ \vec{a}R\]
impl Mul<Rotor> for Multivector {
    type Output = Multivector;
    fn mul(self: Multivector, b: Rotor) -> Multivector {
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
forward_ref_binop!(impl Mul, mul for Multivector, Rotor);

// Rotor-Multivector
// \[ \overset\Rrightarrow{a}\vec{b}\]
impl Mul<Multivector> for Rotor {
    type Output = Multivector;
    fn mul(self: Rotor, b: Multivector) -> Multivector {
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
forward_ref_binop!(impl Mul, mul for Rotor, Multivector);

// Test
#[cfg(test)]
mod geometric_product {
    use core::f32::consts::TAU;

    use super::*;
    use approx::assert_relative_eq;
    #[test]
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
    fn rotor_vector_geo() {
        let angle = TAU / 4.0;
        let rotation_plane = Bivector::new(4.0, 2.0, -3.0);
        let rotor = Rotor::new(angle / 2.0, rotation_plane);
        // 2e12+e31+6e23
        let vector = Vector::new(2.0, 1.0, 6.0);
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
        let rotor = Rotor::new(angle / 2.0, rotation_plane);
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
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
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        // 0.7071+0.5252e12+0.2626e31-0.3939e23
        let angle = TAU / 4.0;
        let rotation_plane = Bivector::new(4.0, 2.0, -3.0);
        let rotor = Rotor::new(angle / 2.0, rotation_plane);
        let mvec = bivector * rotor;
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
        assert_relative_eq!(res, -18.0, max_relative = 0.000001);
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
        let rotor = Rotor::new(scalar / 2.0, bivector);

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
        let rotor = Rotor::new(scalar / 2.0, bivector);

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
