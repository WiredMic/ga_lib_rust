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
    bivector::Bivector, multivector::Multivector, rotor::Rotor, trivector::Trivector,
    vector::Vector,
};

use crate::forward_ref_binop;

use core::ops::BitAnd;

/// # Regressive Product
/// $$ A \vee B = ( -A\star  \wedge -B\star )\star $$

// /// Scalar-Vector
// impl BitAnd<Vector> for f32 {
//     type Output = Vector;
//     fn bitand(self, b: Vector) -> Vector {
//         let e1 = b.e1() * self;
//         let e2 = b.e2() * self;
//         let e3 = b.e3() * self;
//         Vector::new(e1, e2, e3)
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for f32, Vector);

// /// Vector-Scalar
// impl BitAnd<f32> for Vector {
//     type Output = Vector;
//     fn bitand(self, b: f32) -> Vector {
//         let e1 = self.e1() * b;
//         let e2 = self.e2() * b;
//         let e3 = self.e3() * b;
//         Vector::new(e1, e2, e3)
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for Vector,f32);

// /// Scalar-Bivector
// impl BitAnd<Bivector> for f32 {
//     type Output = Bivector;
//     fn bitand(self, b: Bivector) -> Bivector {
//         let e12 = self * b.e12();
//         let e31 = self * b.e31();
//         let e23 = self * b.e23();
//         Bivector::new(e12, e31, e23)
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for f32, Bivector);

// // Bivector-Scalar
// impl BitAnd<f32> for Bivector {
//     type Output = Bivector;
//     fn bitand(self, b: f32) -> Bivector {
//         let e12 = self.e12() * b;
//         let e31 = self.e31() * b;
//         let e23 = self.e23() * b;
//         Bivector::new(e12, e31, e23)
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for Bivector,f32);

// // Scalar-Trivector
// impl BitAnd<Trivector> for f32 {
//     type Output = Trivector;
//     fn bitand(self, b: Trivector) -> Trivector {
//         let e123 = self * b.e123();
//         Trivector::new(e123)
//     }
// }

// // Trivector-Scalar
// impl BitAnd<f32> for Trivector {
//     type Output = Trivector;
//     fn bitand(self, b: f32) -> Trivector {
//         let e123 = self.e123() * b;
//         Trivector::new(e123)
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for f32, Trivector);

// // Scalar-Multivector
// impl BitAnd<Multivector> for f32 {
//     type Output = Multivector;
//     fn bitand(self, b: Multivector) -> Multivector {
//         let scalar = self * b.scalar();
//         let vector = self * b.vector();
//         let bivector = self * b.bivector();
//         let trivector = self * b.trivector();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for Trivector,f32);

// // Multivector-Scalar
// impl BitAnd<f32> for Multivector {
//     type Output = Multivector;
//     fn bitand(self, b: f32) -> Multivector {
//         let scalar = self.scalar() * b;
//         let vector = self.vector() * b;
//         let bivector = self.bivector() * b;
//         let trivector = self.trivector() * b;
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for f32, Multivector);

// // Scalar-Rotor
// // \[ s R\]
// impl BitAnd<Rotor> for f32 {
//     type Output = Multivector;
//     fn bitand(self: f32, b: Rotor) -> Multivector {
//         Multivector::new(
//             self * b.scalar(),
//             Vector::zero(),
//             self * b.bivector(),
//             Trivector::zero(),
//         )
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for Rotor, f32);

// //Rotor-Scalar
// // \[ R b\]
// impl BitAnd<f32> for Rotor {
//     type Output = Multivector;
//     fn bitand(self: Rotor, b: f32) -> Multivector {
//         Multivector::new(
//             self.scalar() * b,
//             Vector::zero(),
//             self.bivector() * b,
//             Trivector::zero(),
//         )
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for f32, Rotor);

// Vector-Vector
// the regressive product does not exsist for vectors
impl BitAnd for Vector {
    type Output = f32;
    fn bitand(self: Vector, _b: Vector) -> f32 {
        // -(self.dual() ^ b.dual()).dual()
        0.0
    }
}
forward_ref_binop!(impl BitAnd, bitand for Vector, Vector);

// Vector-Bivector
// $$ \vec{a} \vee \overset\Rightarrow{b} $$
impl BitAnd<Bivector> for Vector {
    type Output = f32;
    fn bitand(self: Vector, b: Bivector) -> f32 {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Vector, Bivector);

// Bivector-Vector
// $$ \overset\Rightarrow{b} \vee \vec{a} $$
impl BitAnd<Vector> for Bivector {
    type Output = f32;
    fn bitand(self: Bivector, b: Vector) -> f32 {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Bivector, Vector);

// Vector-Trivector
// $$ \vec{a} \vee \overset\Rrightarrow{b} $$
impl BitAnd<Trivector> for Vector {
    type Output = Vector;
    fn bitand(self: Vector, b: Trivector) -> Vector {
        (-self.dual() * -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Vector, Trivector);

// Trivector-Vector
// $$ \overset\Rrightarrow{a} \vee \vec{b} $$
impl BitAnd<Vector> for Trivector {
    type Output = Vector;
    fn bitand(self: Trivector, b: Vector) -> Vector {
        (-self.dual() * -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Trivector, Vector);

// Vector-Multivector
// $$ \vec{a} \vee B $$
impl BitAnd<Multivector> for Vector {
    type Output = Multivector;
    fn bitand(self: Vector, b: Multivector) -> Multivector {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Vector, Multivector);

// Multivector-Vector
// $$ a \vee \vec{B} $$
impl BitAnd<Vector> for Multivector {
    type Output = Multivector;
    fn bitand(self: Multivector, b: Vector) -> Multivector {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Multivector, Vector);

// // Vector-Rotor
// // \[ \vec{a}R\]
// impl BitAnd<Rotor> for Vector {
//     type Output = Multivector;
//     fn bitand(self: Vector, b: Rotor) -> Multivector {
//         let scalar = 0.0;
//         let vector = self * b.scalar();
//         let bivector = Bivector::zero();
//         let trivector = Trivector::zero();
//         Multivector::new(scalar, vector, bivector, trivector) + self * b.bivector()
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for Rotor, Vector);

// // Rotor-Vector
// // \[ \overset\Rrightarrow{a}\vec{b}\]
// impl BitAnd<Vector> for Rotor {
//     type Output = Multivector;
//     fn bitand(self: Rotor, b: Vector) -> Multivector {
//         let scalar = 0.0;
//         let vector = b * self.scalar();
//         let bivector = Bivector::zero();
//         let trivector = Trivector::zero();
//         Multivector::new(scalar, vector, bivector, trivector) + self.bivector() * b
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for Vector, Rotor);

// Bivector-Bivector
// $$ \overset\Rightarrow{a} \vee \overset\Rightarrow{b} $$
impl BitAnd for Bivector {
    type Output = Vector;
    fn bitand(self: Bivector, b: Bivector) -> Vector {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Bivector, Bivector);

// Bivector-Trivector
// \[ \overset\Rightarrow{a} \vee \overset\Rrightarrow{b}\]
impl BitAnd<Trivector> for Bivector {
    type Output = Bivector;
    fn bitand(self: Bivector, b: Trivector) -> Bivector {
        (-self.dual() * -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Bivector, Trivector);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a} \vee \overset\Rightarrow{b}\]
impl BitAnd<Bivector> for Trivector {
    type Output = Bivector;
    fn bitand(self: Trivector, b: Bivector) -> Bivector {
        (-self.dual() * -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Trivector, Bivector);

// Bivector-Multivector
// \[ \overset\Rightarrow{A} \vee B\]
impl BitAnd<Multivector> for Bivector {
    type Output = Multivector;
    fn bitand(self: Bivector, b: Multivector) -> Multivector {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Bivector, Multivector);

// Multivector-Bivector
// \[ A \vee \overset\Rightarrow{b}\]
impl BitAnd<Bivector> for Multivector {
    type Output = Multivector;
    fn bitand(self: Multivector, b: Bivector) -> Multivector {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Multivector, Bivector);

// // Bivector-Rotor
// // \[ \vec{a}R\]
// impl BitAnd<Rotor> for Bivector {
//     type Output = Multivector;
//     fn bitand(self: Bivector, b: Rotor) -> Multivector {
//         self * b.scalar() + self * b.bivector()
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for Bivector, Rotor);

// // Rotor-Bivector
// // \[ \overset\Rrightarrow{a}\vec{b}\]
// impl BitAnd<Bivector> for Rotor {
//     type Output = Multivector;
//     fn bitand(self: Rotor, b: Bivector) -> Multivector {
//         self.scalar() * b + self.bivector() * b
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for Rotor, Bivector);

// Trivector-Trivector
// \[ \overset\Rrightarrow{a} \vee \overset\Rrightarrow{b}\]
impl BitAnd for Trivector {
    type Output = Trivector;
    fn bitand(self: Trivector, b: Trivector) -> Trivector {
        Trivector::new((-self.dual()) * (-b.dual()))
    }
}
forward_ref_binop!(impl BitAnd, bitand for Trivector, Trivector);

// Trivector-Multivector
// \[ \overset\Rrightarrow{a} \vee B\]
impl BitAnd<Multivector> for Trivector {
    type Output = Multivector;
    fn bitand(self: Trivector, b: Multivector) -> Multivector {
        (-self.dual() * -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Trivector,Multivector);

// Multivector-Trivector
// \[ A \vee \overset\Rrightarrow{b}\]
impl BitAnd<Trivector> for Multivector {
    type Output = Multivector;
    fn bitand(self: Multivector, b: Trivector) -> Multivector {
        (-self.dual() * -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Multivector, Trivector);

// // Trivector-Rotor
// // \[ \vec{a}R\]
// impl BitAnd<Rotor> for Trivector {
//     type Output = Multivector;
//     fn bitand(self: Trivector, b: Rotor) -> Multivector {
//         let scalar = 0.0;
//         let vector = self * b.bivector();
//         let bivector = Bivector::zero();
//         let trivector = self * b.scalar();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for Trivector,Rotor);

// // Rotor-Trivector
// // \[ \overset\Rrightarrow{a}\vec{b}\]
// impl BitAnd<Trivector> for Rotor {
//     type Output = Multivector;
//     fn bitand(self: Rotor, b: Trivector) -> Multivector {
//         let scalar = 0.0;
//         let vector = self.bivector() * b;
//         let bivector = Bivector::zero();
//         let trivector = b * self.scalar();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for Rotor, Trivector);

// Multivector-Multivector
// \[ A \vee B\]
impl BitAnd for Multivector {
    type Output = Multivector;
    fn bitand(self: Multivector, b: Multivector) -> Multivector {
        (-self.dual() ^ -b.dual()).dual()
    }
}
forward_ref_binop!(impl BitAnd, bitand for Multivector, Multivector);

// // Multivector-Rotor
// // \[ \vec{a}R\]
// impl BitAnd<Rotor> for Multivector {
//     type Output = Multivector;
//     fn bitand(self: Multivector, b: Rotor) -> Multivector {
//         (self * b.scalar()) + (self * b.bivector())
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for Multivector, Rotor);

// // Rotor-Multivector
// // \[ \overset\Rrightarrow{a}\vec{b}\]
// impl BitAnd<Multivector> for Rotor {
//     type Output = Multivector;
//     fn bitand(self: Rotor, b: Multivector) -> Multivector {
//         (self.scalar() * b) + (self.bivector() * b)
//     }
// }
// forward_ref_binop!(impl BitAnd, bitand for Rotor, Multivector);

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
