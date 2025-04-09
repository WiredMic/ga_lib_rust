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

use core::ops::BitAnd;

/// # Regressive Product
/// $$ A \vee B = ( -A\star  \wedge -B\star )\star $$

// /// Scalar-Vector
// impl<F:Float> BitAnd<Vector<F>> for F {
//     type Output = Vector<F>;
//     fn bitand(self, b: Vector<F>) -> Vector<F> {
//         let e1 = b.e1() * self;
//         let e2 = b.e2() * self;
//         let e3 = b.e3() * self;
//         Vector::new(e1, e2, e3)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for F, Vector<F>);

// /// Vector-Scalar
// impl<F:Float> BitAnd<F> for Vector<F> {
//     type Output = Vector<F>;
//     fn bitand(self, b: F) -> Vector<F> {
//         let e1 = self.e1() * b;
//         let e2 = self.e2() * b;
//         let e3 = self.e3() * b;
//         Vector::new(e1, e2, e3)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for Vector<F>,F);

// /// Scalar-Bivector
// impl<F:Float> BitAnd<Bivector<F>> for F {
//     type Output = Bivector<F>;
//     fn bitand(self, b: Bivector<F>) -> Bivector<F> {
//         let e12 = self * b.e12();
//         let e31 = self * b.e31();
//         let e23 = self * b.e23();
//         Bivector::new(e12, e31, e23)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for F, Bivector<F>);

// // Bivector-Scalar
// impl<F:Float> BitAnd<F> for Bivector<F> {
//     type Output = Bivector<F>;
//     fn bitand(self, b: F) -> Bivector<F> {
//         let e12 = self.e12() * b;
//         let e31 = self.e31() * b;
//         let e23 = self.e23() * b;
//         Bivector::new(e12, e31, e23)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for Bivector<F>,F);

// // Scalar-Trivector
// impl<F:Float> BitAnd<Trivector<F>> for F {
//     type Output = Trivector<F>;
//     fn bitand(self, b: Trivector<F>) -> Trivector<F> {
//         let e123 = self * b.e123();
//         Trivector::new(e123)
//     }
// }

// // Trivector-Scalar
// impl<F:Float> BitAnd<F> for Trivector<F> {
//     type Output = Trivector<F>;
//     fn bitand(self, b: F) -> Trivector<F> {
//         let e123 = self.e123() * b;
//         Trivector::new(e123)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for F, Trivector<F>);

// // Scalar-Multivector
// impl<F:Float> BitAnd<Multivector<F>> for F {
//     type Output = Multivector<F>;
//     fn bitand(self, b: Multivector<F>) -> Multivector<F> {
//         let scalar = self * b.scalar();
//         let vector = self * b.vector();
//         let bivector = self * b.bivector();
//         let trivector = self * b.trivector();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for Trivector<F>,F);

// // Multivector-Scalar
// impl<F:Float> BitAnd<F> for Multivector<F> {
//     type Output = Multivector<F>;
//     fn bitand(self, b: F) -> Multivector<F> {
//         let scalar = self.scalar() * b;
//         let vector = self.vector() * b;
//         let bivector = self.bivector() * b;
//         let trivector = self.trivector() * b;
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for F, Multivector<F>);

// // Scalar-Rotor
// // \[ s R\]
// impl<F:Float> BitAnd<Rotor<F>> for F {
//     type Output = Multivector<F>;
//     fn bitand(self: F, b: Rotor<F>) -> Multivector<F> {
//         Multivector::new(
//             self * b.scalar(),
//             Vector::zero(),
//             self * b.bivector(),
//             Trivector::zero(),
//         )
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for Rotor<F>, F);

// //Rotor-Scalar
// // \[ R b\]
// impl<F:Float> BitAnd<F> for Rotor<F> {
//     type Output = Multivector<F>;
//     fn bitand(self: Rotor<F>, b: F) -> Multivector<F> {
//         Multivector::new(
//             self.scalar() * b,
//             Vector::zero(),
//             self.bivector() * b,
//             Trivector::zero(),
//         )
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for F, Rotor<F>);

// Vector-Vector
// the regressive product does not exsist for vectors
impl<F: Float> BitAnd for Vector<F> {
    type Output = F;
    fn bitand(self: Vector<F>, _b: Vector<F>) -> F {
        // -(self.dual() ^ b.dual()).dual()
        F::zero()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Vector<F>, Vector<F>);

// Vector-Bivector
// $$ \vec{a} \vee \overset\Rightarrow{b} $$
impl<F: Float> BitAnd<Bivector<F>> for Vector<F> {
    type Output = F;
    fn bitand(self: Vector<F>, b: Bivector<F>) -> F {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Vector<F>, Bivector<F>);

// Bivector-Vector
// $$ \overset\Rightarrow{b} \vee \vec{a} $$
impl<F: Float> BitAnd<Vector<F>> for Bivector<F> {
    type Output = F;
    fn bitand(self: Bivector<F>, b: Vector<F>) -> F {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Bivector<F>, Vector<F>);

// Vector-Trivector
// $$ \vec{a} \vee \overset\Rrightarrow{b} $$
impl<F: Float> BitAnd<Trivector<F>> for Vector<F> {
    type Output = Vector<F>;
    fn bitand(self: Vector<F>, b: Trivector<F>) -> Vector<F> {
        (-self.dual() * Scalar(-b.dual())).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Vector<F>, Trivector<F>);

// Trivector-Vector
// $$ \overset\Rrightarrow{a} \vee \vec{b} $$
impl<F: Float> BitAnd<Vector<F>> for Trivector<F> {
    type Output = Vector<F>;
    fn bitand(self: Trivector<F>, b: Vector<F>) -> Vector<F> {
        (Scalar(-self.dual()) * -b.dual()).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Trivector<F>, Vector<F>);

// Vector-Multivector
// $$ \vec{a} \vee B $$
impl<F: Float> BitAnd<Multivector<F>> for Vector<F> {
    type Output = Multivector<F>;
    fn bitand(self: Vector<F>, b: Multivector<F>) -> Multivector<F> {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Vector<F>, Multivector<F>);

// Multivector-Vector
// $$ a \vee \vec{B} $$
impl<F: Float> BitAnd<Vector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn bitand(self: Multivector<F>, b: Vector<F>) -> Multivector<F> {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Multivector<F>, Vector<F>);

// // Vector-Rotor
// // \[ \vec{a}R\]
// impl<F:Float> BitAnd<Rotor<F>> for Vector<F> {
//     type Output = Multivector<F>;
//     fn bitand(self: Vector<F>, b: Rotor<F>) -> Multivector<F> {
//         let scalar = F::zero();
//         let vector = self * b.scalar();
//         let bivector = Bivector::zero();
//         let trivector = Trivector::zero();
//         Multivector::new(scalar, vector, bivector, trivector) + self * b.bivector()
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for Rotor<F>, Vector<F>);

// // Rotor-Vector
// // \[ \overset\Rrightarrow{a}\vec{b}\]
// impl<F:Float> BitAnd<Vector<F>> for Rotor<F> {
//     type Output = Multivector<F>;
//     fn bitand(self: Rotor<F>, b: Vector<F>) -> Multivector<F> {
//         let scalar = F::zero();
//         let vector = b * self.scalar();
//         let bivector = Bivector::zero();
//         let trivector = Trivector::zero();
//         Multivector::new(scalar, vector, bivector, trivector) + self.bivector() * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for Vector<F>, Rotor<F>);

// Bivector-Bivector
// $$ \overset\Rightarrow{a} \vee \overset\Rightarrow{b} $$
impl<F: Float> BitAnd for Bivector<F> {
    type Output = Vector<F>;
    fn bitand(self: Bivector<F>, b: Bivector<F>) -> Vector<F> {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Bivector<F>, Bivector<F>);

// Bivector-Trivector
// \[ \overset\Rightarrow{a} \vee \overset\Rrightarrow{b}\]
impl<F: Float> BitAnd<Trivector<F>> for Bivector<F> {
    type Output = Bivector<F>;
    fn bitand(self: Bivector<F>, b: Trivector<F>) -> Bivector<F> {
        (-self.dual() * Scalar(-b.dual())).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Bivector<F>, Trivector<F>);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a} \vee \overset\Rightarrow{b}\]
impl<F: Float> BitAnd<Bivector<F>> for Trivector<F> {
    type Output = Bivector<F>;
    fn bitand(self: Trivector<F>, b: Bivector<F>) -> Bivector<F> {
        (Scalar(-self.dual()) * -b.dual()).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Trivector<F>, Bivector<F>);

// Bivector-Multivector
// \[ \overset\Rightarrow{A} \vee B\]
impl<F: Float> BitAnd<Multivector<F>> for Bivector<F> {
    type Output = Multivector<F>;
    fn bitand(self: Bivector<F>, b: Multivector<F>) -> Multivector<F> {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Bivector<F>, Multivector<F>);

// Multivector-Bivector
// \[ A \vee \overset\Rightarrow{b}\]
impl<F: Float> BitAnd<Bivector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn bitand(self: Multivector<F>, b: Bivector<F>) -> Multivector<F> {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Multivector<F>, Bivector<F>);

// // Bivector-Rotor
// // \[ \vec{a}R\]
// impl<F:Float> BitAnd<Rotor<F>> for Bivector<F> {
//     type Output = Multivector<F>;
//     fn bitand(self: Bivector<F>, b: Rotor<F>) -> Multivector<F> {
//         self * b.scalar() + self * b.bivector()
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for Bivector<F>, Rotor<F>);

// // Rotor-Bivector
// // \[ \overset\Rrightarrow{a}\vec{b}\]
// impl<F:Float> BitAnd<Bivector<F>> for Rotor<F> {
//     type Output = Multivector<F>;
//     fn bitand(self: Rotor<F>, b: Bivector<F>) -> Multivector<F> {
//         self.scalar() * b + self.bivector() * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for Rotor<F>, Bivector<F>);

// Trivector-Trivector
// \[ \overset\Rrightarrow{a} \vee \overset\Rrightarrow{b}\]
impl<F: Float> BitAnd for Trivector<F> {
    type Output = Trivector<F>;
    fn bitand(self: Trivector<F>, b: Trivector<F>) -> Trivector<F> {
        Trivector::new((-self.dual()) * (-b.dual()))
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Trivector<F>, Trivector<F>);

// Trivector-Multivector
// \[ \overset\Rrightarrow{a} \vee B\]
impl<F: Float> BitAnd<Multivector<F>> for Trivector<F> {
    type Output = Multivector<F>;
    fn bitand(self: Trivector<F>, b: Multivector<F>) -> Multivector<F> {
        (Scalar(-self.dual()) * -b.dual()).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Trivector<F>,Multivector<F>);

// Multivector-Trivector
// \[ A \vee \overset\Rrightarrow{b}\]
impl<F: Float> BitAnd<Trivector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn bitand(self: Multivector<F>, b: Trivector<F>) -> Multivector<F> {
        (-self.dual() * Scalar(-b.dual())).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Multivector<F>, Trivector<F>);

// // Trivector-Rotor
// // \[ \vec{a}R\]
// impl<F:Float> BitAnd<Rotor<F>> for Trivector<F> {
//     type Output = Multivector<F>;
//     fn bitand(self: Trivector<F>, b: Rotor<F>) -> Multivector<F> {
//         let scalar = F::zero();
//         let vector = self * b.bivector();
//         let bivector = Bivector::zero();
//         let trivector = self * b.scalar();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for Trivector<F>,Rotor<F>);

// // Rotor-Trivector
// // \[ \overset\Rrightarrow{a}\vec{b}\]
// impl<F:Float> BitAnd<Trivector<F>> for Rotor<F> {
//     type Output = Multivector<F>;
//     fn bitand(self: Rotor<F>, b: Trivector<F>) -> Multivector<F> {
//         let scalar = F::zero();
//         let vector = self.bivector() * b;
//         let bivector = Bivector::zero();
//         let trivector = b * self.scalar();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for Rotor<F>, Trivector<F>);

// Multivector-Multivector
// \[ A \vee B\]
impl<F: Float> BitAnd for Multivector<F> {
    type Output = Multivector<F>;
    fn bitand(self: Multivector<F>, b: Multivector<F>) -> Multivector<F> {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl<F:Float> BitAnd, bitand for Multivector<F>, Multivector<F>);

// // Multivector-Rotor
// // \[ \vec{a}R\]
// impl<F:Float> BitAnd<Rotor<F>> for Multivector<F> {
//     type Output = Multivector<F>;
//     fn bitand(self: Multivector<F>, b: Rotor<F>) -> Multivector<F> {
//         (self * b.scalar()) + (self * b.bivector())
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for Multivector<F>, Rotor<F>);

// // Rotor-Multivector
// // \[ \overset\Rrightarrow{a}\vec{b}\]
// impl<F:Float> BitAnd<Multivector<F>> for Rotor<F> {
//     type Output = Multivector<F>;
//     fn bitand(self: Rotor<F>, b: Multivector<F>) -> Multivector<F> {
//         (self.scalar() * b) + (self.bivector() * b)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitAnd, bitand for Rotor<F>, Multivector<F>);

// Test
#[cfg(test)]
mod regressive_product {
    use core::f32::consts::TAU;

    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn vector_vector() {
        // 3e1+5e2+4e3
        let vector1 = Vector::new(3.0, 5.0, 4.0);
        // 2e1+e2+6e3
        let vector2 = Vector::new(2.0, 1.0, 6.0);
        let res = vector1 & vector2;
        // 0 ​
        assert_relative_eq!(res, 0.0, max_relative = 0.000001);
    }

    #[test]
    fn vector_bivector() {
        // 3e1+5e2+4e3
        let vector = Vector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        let res = vector & bivector;
        // 31  ​
        assert_relative_eq!(res, 31.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_vector() {
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        // 3e1+5e2+4e3
        let vector = Vector::new(3.0, 5.0, 4.0);
        let res = bivector & vector;
        // 31  ​
        assert_relative_eq!(res, 31.0, max_relative = 0.000001);
    }

    #[test]
    fn vector_trivector() {
        // 3e1+5e2+4e3
        let vector = Vector::new(3.0, 5.0, 4.0);
        // 2e123
        let trivector = Trivector::new(2.0);
        let res = vector & trivector;
        // 6e1​+10e2​+8e3
        assert_relative_eq!(res.e1(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 10.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 8.0, max_relative = 0.000001);
    }

    #[test]
    fn trivector_vector() {
        // 2e12+e31+6e23
        let trivector = Trivector::new(2.0);
        // 3e1+5e2+4e3
        let vector = Vector::new(3.0, 5.0, 4.0);
        let res = trivector & vector;
        // 6e1​+10e2​+8e3
        assert_relative_eq!(res.e1(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 10.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 8.0, max_relative = 0.000001);
    }

    #[test]
    fn vector_multivector() {
        // 3e1+5e2+4e3
        let vector = Vector::new(3.0, 5.0, 4.0);
        // ( 5.0 + 8.0e1 + 7.0e2 + 3.0e3 + 2.0e12 + 8.0e31 + 2.0e23 + 1.0e123 )
        let mvec = Multivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        let res = vector & mvec;
        // 54+3e1​+5e2​+4e3
        assert_relative_eq!(res.scalar(), 54.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 5.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 4.0, max_relative = 0.000001);
    }

    #[test]
    fn multivector_vector() {
        // ( 5.0 + 8.0e1 + 7.0e2 + 3.0e3 + 2.0e12 + 8.0e31 + 2.0e23 + 1.0e123 )
        let mvec = Multivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        // 3e1+5e2+4e3
        let vector = Vector::new(3.0, 5.0, 4.0);
        let res = mvec & vector;
        // 54+3e1​+5e2​+4e3
        assert_relative_eq!(res.scalar(), 54.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 5.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 4.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_bivector() {
        // 2e12+e31+6e23
        let bivector1 = Bivector::new(2.0, 1.0, 6.0);
        // 3e12+5e31+4e23
        let bivector2 = Bivector::new(3.0, 5.0, 4.0);
        let mvec = bivector1 & bivector2;
        // 7e1​+10e2​-26e3​
        assert_relative_eq!(mvec.e1(), 7.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e2(), 10.0, max_relative = 0.000001);
        assert_relative_eq!(mvec.e3(), -26.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_trivector() {
        // 3e12+5e31+4e23
        let bivector = Bivector::new(3.0, 5.0, 4.0);
        // 2e123
        let trivector = Trivector::new(2.0);
        let res = bivector & trivector;
        // 6e12​+10e31​+8e23
        assert_relative_eq!(res.e12(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 10.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 8.0, max_relative = 0.000001);
    }

    #[test]
    fn trivector_bivector() {
        // 2e123
        let trivector = Trivector::new(2.0);
        // 3e12+5e31+4e23
        let bivector = Bivector::new(3.0, 5.0, 4.0);
        let res = trivector & bivector;
        // 6e12​+10e31​+8e23
        assert_relative_eq!(res.e12(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 10.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 8.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_multivector() {
        // 3e12+5e31+4e23
        let bivector = Bivector::new(3.0, 5.0, 4.0);
        // ( 5.0 + 8.0e1 + 7.0e2 + 3.0e3 + 2.0e12 + 8.0e31 + 2.0e23 + 1.0e123 )
        let mvec = Multivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        let res = bivector & mvec;
        // 76+14e1​+2e2​-22e3​+3e12​+5e31​+4e23
        assert_relative_eq!(res.scalar(), 76.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 14.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 2.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), -22.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 5.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 4.0, max_relative = 0.000001);
    }

    #[test]
    fn multivector_bivector() {
        // ( 5.0 + 8.0e1 + 7.0e2 + 3.0e3 + 2.0e12 + 8.0e31 + 2.0e23 + 1.0e123 )
        let mvec = Multivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        // 3e12+5e31+4e23
        let bivector = Bivector::new(3.0, 5.0, 4.0);
        let res = mvec & bivector;
        // 76-14e1​-2e2​+22e3​+3e12​+5e31​+4e23
        assert_relative_eq!(res.scalar(), 76.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), -14.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), -2.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 22.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 5.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 4.0, max_relative = 0.000001);
    }

    #[test]
    fn trivector_trivector() {
        // 2e123
        let trivector1 = Trivector::new(2.0);
        // 3e123
        let trivector2 = Trivector::new(3.0);
        let res = trivector1 & trivector2;
        // 6e123
        assert_relative_eq!(res.e123(), 6.0, max_relative = 0.000001);
    }

    #[test]
    fn trivector_multivector() {
        // 3e123
        let trivector = Trivector::new(3.0);
        // ( 5.0 + 8.0e1 + 7.0e2 + 3.0e3 + 2.0e12 + 8.0e31 + 2.0e23 + 1.0e123 )
        let mvec = Multivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        let res = trivector & mvec;
        // 15+24e1​+21e2​+9e3​+6e12​+24e31​+6e23​+3e123​
        assert_relative_eq!(res.scalar(), 15.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 24.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 21.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 9.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 24.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 3.0, max_relative = 0.000001);
    }

    #[test]
    fn multivector_trivector() {
        // ( 5.0 + 8.0e1 + 7.0e2 + 3.0e3 + 2.0e12 + 8.0e31 + 2.0e23 + 1.0e123 )
        let mvec = Multivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        // 3e123
        let trivector = Trivector::new(3.0);
        let res = mvec & trivector;
        // 15+24e1​+21e2​+9e3​+6e12​+24e31​+6e23​+3e123
        assert_relative_eq!(res.scalar(), 15.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 24.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 21.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 9.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 24.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 3.0, max_relative = 0.000001);
    }

    #[test]
    fn multivector_multivector() {
        // ( 5.0 + 8.0e1 + 7.0e2 + 3.0e3 + 2.0e12 + 8.0e31 + 2.0e23 + 1.0e123 )
        let mvec1 = Multivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        // -4-2e1-4e2-9e3-2e12-1e31-7e23-1e123
        let mvec2 = Multivector::new_components(-4.0, -2.0, -4.0, -9.0, -2.0, -1.0, -7.0, -1.0);
        let res = mvec1 & mvec2;
        // -132+4e1​-e2​−66e3​-4e12​-9e31​-9e23​-e123
        assert_relative_eq!(res.scalar(), -132.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 4.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), -1.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), -66.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), -4.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), -9.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), -9.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), -1.0, max_relative = 0.000001);
    }
}
