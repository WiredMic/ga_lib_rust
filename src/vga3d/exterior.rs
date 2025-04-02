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
    bivector::{self, Bivector},
    multivector::Multivector,
    rotor::Rotor,
    trivector::{self, Trivector},
    vector::Vector,
};

use crate::forward_ref_binop;
use core::ops::BitXor;

// // Scalar-Scalar
// // \[ a\wedge b\]
// impl BitXor for f32 {
//     type Output = f32;
//     fn bitxor(self: f32, b: Vector) -> f32 {
//         self * b
//     }
// }

// // Scalar-Vector
// // \[ a\wedge \vec{b}\]
// impl BitXor<Vector> for f32 {
//     type Output = Vector;
//     fn bitxor(self: f32, b: Vector) -> Vector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for f32, Vector);

// // Vector-Scalar
// // \[ \vec{a}\wedge b\]
// impl BitXor<f32> for Vector {
//     type Output = Vector;
//     fn bitxor(self: Vector, b: f32) -> Vector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Vector, f32);

// // Scalar-Bivector
// // \[ s\wedge \overset\Rightarrow{b}\]
// impl BitXor<Bivector> for f32 {
//     type Output = Bivector;
//     fn bitxor(self: f32, b: Bivector) -> Bivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for f32, Bivector);

// // Bivector-Scalar
// // \[ \overset\Rightarrow{a}\wedge b\]
// impl BitXor<f32> for Bivector {
//     type Output = Bivector;
//     fn bitxor(self: Bivector, b: f32) -> Bivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Bivector, f32);

// // Scalar-Trivector
// // \[ s\wedge \overset\Rrightarrow{b}\]
// impl BitXor<Trivector> for f32 {
//     type Output = Trivector;
//     fn bitxor(self: f32, b: Trivector) -> Trivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for f32, Trivector);

// // Trivector-Scalar
// // \[ \overset\Rrightarrow{a}\wedge b\]
// impl BitXor<f32> for Trivector {
//     type Output = Trivector;
//     fn bitxor(self: Trivector, b: f32) -> Trivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Trivector, f32);

// // Scalar-Multivector
// // \[ s\wedge B\]
// impl BitXor<Multivector> for f32 {
//     type Output = Multivector;
//     fn bitxor(self: f32, b: Multivector) -> Multivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for f32, Multivector);

// // Multivector-Scalar
// // \[ B\wedge b\]
// impl BitXor<f32> for Multivector {
//     type Output = Multivector;
//     fn bitxor(self: Multivector, b: f32) -> Multivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Multivector, f32);

// // Scalar-Rotor
// // \[ s\wedge R\]
// impl BitXor<Rotor> for f32 {
//     type Output = Multivector;
//     fn bitxor(self: f32, b: Rotor) -> Multivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for f32, Rotor);

// //Rotor-Scalar
// // \[ R\wedge b\]
// impl BitXor<f32> for Rotor {
//     type Output = Multivector;
//     fn bitxor(self: Rotor, b: f32) -> Multivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Rotor, f32);

// Vector-Vector
impl BitXor for Vector {
    type Output = Bivector;
    fn bitxor(self: Vector, b: Vector) -> Bivector {
        Bivector::new(
            self.e1() * b.e2() - self.e2() * b.e1(),
            self.e3() * b.e1() - self.e1() * b.e3(),
            self.e2() * b.e3() - self.e3() * b.e2(),
        )
    }
}
forward_ref_binop!(impl BitXor, bitxor for Vector, Vector);

// Vector-Bivector
// \[ \vec{a}\wedge \overset\Rightarrow{b}\]
impl BitXor<Bivector> for Vector {
    type Output = Trivector;
    fn bitxor(self: Vector, b: Bivector) -> Trivector {
        Trivector::new(self.e1() * b.e23() + self.e2() * b.e31() + self.e3() * b.e12())
    }
}
forward_ref_binop!(impl BitXor, bitxor for Vector, Bivector);

// Bivector-Vector
// \[ \overset\Rightarrow{b}\wedge \vec{b}\]
impl BitXor<Vector> for Bivector {
    type Output = Trivector;
    fn bitxor(self: Bivector, b: Vector) -> Trivector {
        Trivector::new(self.e23() * b.e1() + self.e31() * b.e2() + self.e12() * b.e3())
    }
}
forward_ref_binop!(impl BitXor, bitxor for Bivector, Vector);

// Vector-Trivector
// \[ \vec{a}\wedge \overset\Rrightarrow{b}\]
impl BitXor<Trivector> for Vector {
    type Output = f32;
    fn bitxor(self: Vector, _b: Trivector) -> f32 {
        0.0
    }
}
forward_ref_binop!(impl BitXor, bitxor for Vector, Trivector);

// Trivector-Vector
// \[ \overset\Rrightarrow{a}\wedge \vec{b}\]
impl BitXor<Vector> for Trivector {
    type Output = f32;
    fn bitxor(self: Trivector, _b: Vector) -> f32 {
        0.0
    }
}
forward_ref_binop!(impl BitXor, bitxor for Trivector, Vector);

// Vector-Multivector
// \[ \vec{a}\wedge B\]
impl BitXor<Multivector> for Vector {
    type Output = Multivector;
    fn bitxor(self: Vector, b: Multivector) -> Multivector {
        Multivector::new(
            self ^ b.trivector(),
            self * b.scalar(),
            self ^ b.vector(),
            self ^ b.bivector(),
        )
    }
}
forward_ref_binop!(impl BitXor, bitxor for Vector, Multivector);

// Multivector-Vector
// \[ A\wedge \vec{b}\]
impl BitXor<Vector> for Multivector {
    type Output = Multivector;
    fn bitxor(self: Multivector, b: Vector) -> Multivector {
        Multivector::new(
            self.trivector() ^ b,
            self.scalar() * b,
            self.vector() ^ b,
            self.bivector() ^ b,
        )
    }
}
forward_ref_binop!(impl BitXor, bitxor for Multivector, Vector);

// // Vector-Rotor
// // \[ \vec{a} \wedge R\]
// impl BitXor<Rotor> for Vector {
//     type Output = Multivector;
//     fn bitxor(self: Vector, b: Rotor) -> Multivector {
//         Multivector::new(
//             0.0,
//             self ^ b.scalar(),
//             Bivector::zero(),
//             self ^ b.bivector(),
//         )
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Vector, Rotor);

// // Rotor-Vector
// // \[ R\wedge \vec{b}\]
// impl BitXor<Vector> for Rotor {
//     type Output = Multivector;
//     fn bitxor(self: Rotor, b: Vector) -> Multivector {
//         Multivector::new(
//             0.0,
//             self.scalar() ^ b,
//             Bivector::zero(),
//             self.bivector() ^ b,
//         )
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Rotor, Vector);

// Bivector-Bivector
// \[ \overset\Rightarrow{a} \wedge \overset\Rightarrow{b} = \left <\overset\Rightarrow{a} \overset\Rightarrow{b} \right>_4 \]
// There is no object of grade 4 in 3D VGA
impl BitXor for Bivector {
    type Output = f32;

    fn bitxor(self: Bivector, _b: Bivector) -> f32 {
        0.0
    }
}

// Bivector-Trivector
// \[ \overset\Rightarrow{a}\wedge \overset\Rrightarrow{b}\]
impl BitXor<Trivector> for Bivector {
    type Output = f32;
    fn bitxor(self: Bivector, _b: Trivector) -> f32 {
        0.0
    }
}
forward_ref_binop!(impl BitXor, bitxor for Bivector, Trivector);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a}\wedge \overset\Rightarrow{b}\]
impl BitXor<Bivector> for Trivector {
    type Output = f32;
    fn bitxor(self: Trivector, _b: Bivector) -> f32 {
        0.0
    }
}
forward_ref_binop!(impl BitXor, bitxor for Trivector, Bivector);

// Bivector-Multivector
// \[ \overset\Rightarrow{a}\wedge B\]
impl BitXor<Multivector> for Bivector {
    type Output = Multivector;
    fn bitxor(self: Bivector, b: Multivector) -> Multivector {
        Multivector::new(
            (self ^ b.bivector()) + (self ^ b.trivector()),
            Vector::zero(),
            self * b.scalar(),
            self ^ b.vector(),
        )
    }
}
forward_ref_binop!(impl BitXor, bitxor for Bivector, Multivector);

// Multivector-Bivector
// \[ A\wedge \overset\Rightarrow{b}\]
impl BitXor<Bivector> for Multivector {
    type Output = Multivector;
    fn bitxor(self: Multivector, b: Bivector) -> Multivector {
        Multivector::new(
            (self.bivector() ^ b) + (self.trivector() ^ b),
            Vector::zero(),
            self.scalar() * b,
            self.vector() ^ b,
        )
    }
}
forward_ref_binop!(impl BitXor, bitxor for Multivector, Bivector);

// // Bivector-Rotor
// // \[ \overset\Rightarrow{a}\wedge R\]
// impl BitXor<Rotor> for Bivector {
//     type Output = Multivector;
//     fn bitxor(self: Bivector, b: Rotor) -> Multivector {
//         Multivector::new(
//             self ^ b.bivector(),
//             Vector::zero(),
//             self * b.scalar(),
//             Trivector::zero(),
//         )
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Bivector, Rotor);

// // Rotor-Bivector
// // \[ R\wedge \overset\Rightarrow{b}\]
// impl BitXor<Bivector> for Rotor {
//     type Output = Multivector;
//     fn bitxor(self: Rotor, b: Bivector) -> Multivector {
//         Multivector::new(
//             self.bivector() ^ b,
//             Vector::zero(),
//             self.scalar() * b,
//             Trivector::zero(),
//         )
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Rotor, Bivector);

// Trivector-Trivector
// \[ \overset\Rrightarrow{a} \wedge \overset\Rrightarrow{b} = \left <\overset\Rrightarrow{a} \overset\Rrightarrow{b} \right>_4 \]
// There is no object of grade 6 in 3D VGA
impl BitXor for Trivector {
    type Output = f32;
    fn bitxor(self: Trivector, _b: Trivector) -> f32 {
        0.0
    }
}
forward_ref_binop!(impl BitXor, bitxor for Trivector, Trivector);

// Trivector-Multivector
// \[ \overset\Rrightarrow{a}\wedge B\]
impl BitXor<Multivector> for Trivector {
    type Output = Multivector;
    fn bitxor(self: Trivector, b: Multivector) -> Multivector {
        Multivector::new(
            (self ^ b.vector()) + (self ^ b.bivector()) + (self ^ b.trivector()),
            Vector::zero(),
            Bivector::zero(),
            self * b.scalar(),
        )
    }
}
forward_ref_binop!(impl BitXor, bitxor for Trivector, Multivector);

// Multivector-Trivector
// \[ A\wedge \overset\Rrightarrow{b}\]
impl BitXor<Trivector> for Multivector {
    type Output = Multivector;
    fn bitxor(self: Multivector, b: Trivector) -> Multivector {
        Multivector::new(
            (self.vector() ^ b) + (self.bivector() ^ b) + (self.trivector() ^ b),
            Vector::zero(),
            Bivector::zero(),
            self.scalar() * b,
        )
    }
}
forward_ref_binop!(impl BitXor, bitxor for Multivector, Trivector);

// // Trivector-Rotor
// // \[ \overset\Rrightarrow{a}\wedge R\]
// impl BitXor<Rotor> for Trivector {
//     type Output = Multivector;
//     fn bitxor(self: Trivector, b: Rotor) -> Multivector {
//         let scalar = self ^ b.bivector();
//         let vector = Vector::zero();
//         let bivector = Bivector::zero();
//         let trivector = self ^ b.scalar();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Trivector, Rotor);

// // Rotor-Trivector
// // \[ R\wedge \overset\Rrightarrow{b}\]
// impl BitXor<Trivector> for Rotor {
//     type Output = Multivector;
//     fn bitxor(self: Rotor, b: Trivector) -> Multivector {
//         Multivector::new(
//             self.bivector() ^ b,
//             Vector::zero(),
//             Bivector::zero(),
//             self.scalar() ^ b,
//         )
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Rotor, Trivector);

// Multivector-Multivector
// \[ A \wedge B = \left <A B \right>_{a+b} \]
impl BitXor for Multivector {
    type Output = Multivector;
    fn bitxor(self: Multivector, b: Multivector) -> Multivector {
        (self * b.scalar()) + (self ^ b.vector()) + (self ^ b.bivector()) + (self ^ b.trivector())
    }
}

// // Multivector-Rotor
// // \[ A\wedge R\]
// impl BitXor<Rotor> for Multivector {
//     type Output = Multivector;
//     fn bitxor(self: Multivector, b: Rotor) -> Multivector {
//         let scalar = (self.scalar() * b.scalar())
//             + (self.bivector() ^ b.bivector())
//             + (self.trivector() ^ b.bivector());
//         let vector = self.vector() ^ b.scalar();
//         let bivector = (self.bivector() ^ b.scalar()) + (self.scalar() ^ b.bivector());
//         let trivector = (self.trivector() ^ b.scalar()) + (self.vector() ^ b.bivector());
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Multivector, Rotor);

// // Rotor-Multivector
// // \[ R\wedge B\]
// impl BitXor<Multivector> for Rotor {
//     type Output = Multivector;
//     fn bitxor(self: Rotor, b: Multivector) -> Multivector {
//         let scalar = (self.scalar() * b.scalar())
//             + (self.bivector() ^ b.bivector())
//             + (self.bivector() ^ b.trivector());
//         let vector = self.scalar() ^ b.vector();
//         let bivector = (self.scalar() ^ b.bivector()) + (self.bivector() ^ b.scalar());
//         let trivector = (self.scalar() ^ b.trivector()) + (self.bivector() ^ b.vector());
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Rotor, Multivector);

// // Rotor-Rotor
// // \[ R_1 \wedge R_2\]
// impl BitXor for Rotor {
//     type Output = Multivector;
//     fn bitxor(self: Rotor, b: Rotor) -> Multivector {
//         let scalar = (self.scalar() * b.scalar()) + (self.bivector() ^ b.bivector());
//         let vector = Vector::zero();
//         let bivector = (self.scalar() ^ b.bivector()) + (self.bivector() ^ b.scalar());
//         let trivector = Trivector::zero();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitXor, bitxor for Rotor, Rotor);

#[cfg(test)]
mod exterior_product {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn vector_vector() {
        // 3e1+5e2+4e3
        let vector1 = Vector::new(3.0, 5.0, 4.0);
        // 2e1+1e2+6e3
        let vector2 = Vector::new(2.0, 1.0, 6.0);
        let bivector_ref1 = &vector1 ^ &vector2;
        let bivector_ref2 = &vector1 ^ vector2;
        let bivector_ref3 = vector1 ^ &vector2;
        let bivector = vector1 ^ vector2;
        // −7e12​-10e31​+26e23
        assert_relative_eq!(bivector_ref1.e12(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(bivector_ref1.e31(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(bivector_ref1.e23(), 26.0, max_relative = 0.000001);

        assert_relative_eq!(bivector_ref2.e12(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(bivector_ref2.e31(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(bivector_ref2.e23(), 26.0, max_relative = 0.000001);

        assert_relative_eq!(bivector_ref3.e12(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(bivector_ref3.e31(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(bivector_ref3.e23(), 26.0, max_relative = 0.000001);

        assert_relative_eq!(bivector.e12(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(bivector.e31(), -10.0, max_relative = 0.000001);
        assert_relative_eq!(bivector.e23(), 26.0, max_relative = 0.000001);
    }

    #[test]
    fn vector_bivector() {
        // 2e1+e2+6e3
        let vector = Vector::new(2.0, 1.0, 6.0);
        // 3e12+5e31+4e23
        let bivector = Bivector::new(3.0, 5.0, 4.0);
        let vector_res = vector ^ bivector;
        // -31e1​23
        assert_relative_eq!(vector_res.e123(), 31.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_vector() {
        // 3e1+5e2+4e3
        let bivector = Bivector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let vector = Vector::new(2.0, 1.0, 6.0);
        // 31e123​
        let trivector = bivector ^ vector;
        assert_relative_eq!(trivector.e123(), 31.0, max_relative = 0.000001);
    }

    #[test]
    fn vector_trivector() {
        // 2e1+e2+6e3
        let vector = Vector::new(2.0, 1.0, 6.0);
        // 3e123
        let trivector = Trivector::new(3.0);
        let res = vector ^ trivector;
        // 0.0
        assert_relative_eq!(res, 0.0, max_relative = 0.000001);
    }

    #[test]
    fn trivector_vector() {
        // 3e123
        let trivector = Trivector::new(3.0);
        // 2e1+e2+6e3
        let vector = Vector::new(2.0, 1.0, 6.0);
        let res = trivector ^ vector;
        // 0.0
        assert_relative_eq!(res, 0.0, max_relative = 0.000001);
    }

    #[test]
    fn vector_multivector() {
        // 2e1+e2+6e3
        let vector = Vector::new(2.0, 1.0, 6.0);
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let multivector = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        let res = vector ^ multivector;
        // 12e1​+6e2​+36e3​+5e12​+46e31-38e23​+62e123
        assert_relative_eq!(res.scalar(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 12.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 36.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 5.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 46.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), -38.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 62.0, max_relative = 0.000001);
    }

    #[test]
    fn multivector_vector() {
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let multivector = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        // 2e1+e2+6e3
        let vector = Vector::new(2.0, 1.0, 6.0);
        let res = multivector ^ vector;
        // 12e1​+6e2​+36e3​-5e12​-46e31​+38e23​+62e123​
        assert_relative_eq!(res.scalar(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 12.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 36.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), -5.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), -46.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 38.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 62.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_bivector() {
        // 3e1+5e2+4e3
        let bivector1 = Bivector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let bivector2 = Bivector::new(2.0, 1.0, 6.0);
        // 31e123​
        assert_relative_eq!(bivector1 ^ bivector2, 0.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_trivector() {
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        // 3e123
        let trivector = Trivector::new(3.0);
        let res = bivector ^ trivector;
        // 0
        assert_relative_eq!(res, 0.0, max_relative = 0.000001);
    }

    #[test]
    fn trivector_bivector() {
        // 3e123
        let trivector = Trivector::new(3.0);
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        let res = trivector ^ bivector;
        // 0
        assert_relative_eq!(res, 0.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_multivector() {
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let multivector = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        let res = bivector ^ multivector;
        // 12e12​+6e31​+36e23​+69e123​
        assert_relative_eq!(res.scalar(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 12.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 36.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 69.0, max_relative = 0.000001);
    }

    #[test]
    fn multivector_bivector() {
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let multivector = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        let res = multivector ^ bivector;
        // 12e12​+6e31​+36e23​+69e123​
        assert_relative_eq!(res.scalar(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 12.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 36.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 69.0, max_relative = 0.000001);
    }

    #[test]
    fn trivec_trivec() {
        let trivector1 = Trivector::new(3.0);
        let trivector2 = Trivector::new(6.0);
        let res = trivector1 ^ trivector2;
        assert_relative_eq!(res, 0.0, max_relative = 0.000001);
    }

    #[test]
    fn trivector_multivector() {
        // 2e123
        let trivector = Trivector::new(2.0);
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let multivector = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        let res = trivector ^ multivector;
        // 12e123​
        assert_relative_eq!(res.scalar(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 12.0, max_relative = 0.000001);
    }

    #[test]
    fn multivector_trivector() {
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let multivector = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        // 2e123
        let trivector = Trivector::new(2.0);
        let res = multivector ^ trivector;
        // 12e123​
        assert_relative_eq!(res.scalar(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 0.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 12.0, max_relative = 0.000001);
    }

    #[test]
    fn mvec_mvec() {
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let mvec1 = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        // 5.0+ 8.0e1+ 7.0e2+ 3.0e3+ 2.0e12+ 8.0e31+ 2.0e23+ 1.0e123
        let mvec2 = Multivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        let mvec_res = mvec1 ^ mvec2;
        // 30+93e1​+77e2​+38e3​+54e12​−73e13​+45e23​+236e123
        assert_relative_eq!(mvec_res.scalar(), 30.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), 93.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), 77.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), 38.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 54.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 73.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 45.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 236.0, max_relative = 0.000001);
    }

    #[test]
    fn negetive_mvec_mvec() {
        // -4.0-1.0e1 -3.0e2 -2.0e3 -9.0e12 -6.0e31 -3.0e23 -10.0e123
        let mvec1 = Multivector::new_components(-4.0, -1.0, -3.0, -2.0, -9.0, -6.0, -3.0, -10.0);
        // -4.0 -2.0e1 -4.0e2 -9.0e3 -2.0e12 -1.0e31 -7.0e23 -1.0e123
        let mvec2 = Multivector::new_components(-4.0, -2.0, -4.0, -9.0, -2.0, -1.0, -7.0, -1.0);
        let mvec_res = mvec1 ^ mvec2;
        // 16+12e1​+28e2​+44e3​+42e12​−23e13​+59e23​+169e123
        assert_relative_eq!(mvec_res.scalar(), 16.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), 12.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), 28.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), 44.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 42.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 23.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 59.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 169.0, max_relative = 0.000001);
    }
}
