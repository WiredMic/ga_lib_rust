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

use core::ops::BitOr;

// Scalar-Scalar
// \[ a \cdot b\]
// impl BitOr for f32 {
//     type Output = f32;
//     fn bitor(self: f32, b: Vector) -> f32 {
//         self * b
//     }
// }

// // Scalar-Vector
// // \[ a \cdot \vec{b}\]
// impl BitOr<Vector> for f32 {
//     type Output = Vector;
//     fn bitor(self: f32, b: Vector) -> Vector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for f32, Vector);

// // Vector-Scalar
// // \[ \vec{a} \cdot b\]
// impl BitOr<f32> for Vector {
//     type Output = Vector;
//     fn bitor(self: Vector, b: f32) -> Vector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Vector, f32);

// // Scalar-Bivector
// // \[ s \cdot \overset\Rightarrow{b}\]
// impl BitOr<Bivector> for f32 {
//     type Output = Bivector;
//     fn bitor(self: f32, b: Bivector) -> Bivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for f32, Bivector);

// // Bivector-Scalar
// // \[ \overset\Rightarrow{a} \cdot b\]
// impl BitOr<f32> for Bivector {
//     type Output = Bivector;
//     fn bitor(self: Bivector, b: f32) -> Bivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Bivector,f32);

// // Scalar-Trivector
// // \[ s \cdot \overset\Rrightarrow{b}\]
// impl BitOr<Trivector> for f32 {
//     type Output = Trivector;
//     fn bitor(self: f32, b: Trivector) -> Trivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for f32, Trivector);

// // Trivector-Scalar
// // \[ \overset\Rrightarrow{a} \cdot b\]
// impl BitOr<f32> for Trivector {
//     type Output = Trivector;
//     fn bitor(self: Trivector, b: f32) -> Trivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Trivector, f32);

// // Scalar-Multivector
// // \[ s \cdot B\]
// impl BitOr<Multivector> for f32 {
//     type Output = Multivector;
//     fn bitor(self: f32, b: Multivector) -> Multivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for f32, Multivector);

// // Multivector-Scalar
// // \[ B \cdot b\]
// impl BitOr<f32> for Multivector {
//     type Output = Multivector;
//     fn bitor(self: Multivector, b: f32) -> Multivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Multivector, f32);

// // Scalar-Rotor
// // \[ s \cdot R\]
// impl BitOr<Rotor> for f32 {
//     type Output = Multivector;
//     fn bitor(self: f32, b: Rotor) -> Multivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for f32, Rotor);

// //Rotor-Scalar
// // \[ R \cdot b\]
// impl BitOr<f32> for Rotor {
//     type Output = Multivector;
//     fn bitor(self: Rotor, b: f32) -> Multivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Rotor, f32);

// Vector-Vector
// \[ \vec{u} \cdot \vec{v} = u_1 \cdot v_1 + u_2 \cdot v_2 + u_3 \cdot v_3 \]
impl BitOr for Vector {
    type Output = f32;
    fn bitor(self: Vector, b: Vector) -> f32 {
        self.e1() * b.e1() + self.e2() * b.e2() + self.e3() * b.e3()
    }
}
forward_ref_binop!(impl BitOr, bitor for Vector, Vector);

// Vector-Bivector
// \[ \vec{a} \cdot \overset\Rightarrow{b}\]
impl BitOr<Bivector> for Vector {
    type Output = Vector;
    fn bitor(self: Vector, b: Bivector) -> Vector {
        let e1 = -self.e2() * b.e12() + self.e3() * b.e31();
        let e2 = self.e1() * b.e12() - self.e3() * b.e23();
        let e3 = -self.e1() * b.e31() + self.e2() * b.e23();
        Vector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl BitOr, bitor for Vector, Bivector);

// Bivector-Vector
// \[ \overset\Rightarrow{b} \cdot \vec{b}\]
impl BitOr<Vector> for Bivector {
    type Output = Vector;
    fn bitor(self: Bivector, b: Vector) -> Vector {
        let e1 = self.e12() * b.e2() - self.e31() * b.e3();
        let e2 = -self.e12() * b.e1() + self.e23() * b.e3();
        let e3 = self.e31() * b.e1() - self.e23() * b.e2();
        Vector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl BitOr, bitor for Bivector, Vector);

// Vector-Trivector
// \[ \vec{a} \cdot \overset\Rrightarrow{b}\]
impl BitOr<Trivector> for Vector {
    type Output = Bivector;
    fn bitor(self: Vector, b: Trivector) -> Bivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for Vector, Trivector);

// Trivector-Vector
// \[ \overset\Rrightarrow{a} \cdot \vec{b}\]
impl BitOr<Vector> for Trivector {
    type Output = Bivector;
    fn bitor(self: Trivector, b: Vector) -> Bivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for Trivector, Vector);

// Vector-Multivector
// \[ \vec{a} \cdot B\]
impl BitOr<Multivector> for Vector {
    type Output = Multivector;
    fn bitor(self: Vector, b: Multivector) -> Multivector {
        Multivector::new(
            self | b.vector(),
            (self | b.bivector()) + (self * b.scalar()),
            self | b.trivector(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl BitOr, bitor for Vector, Multivector);

// Multivector-Vector
// \[ A \cdot \vec{b}\]
impl BitOr<Vector> for Multivector {
    type Output = Multivector;
    fn bitor(self: Multivector, b: Vector) -> Multivector {
        Multivector::new(
            self.vector() | b,
            (self.bivector() | b) + (self.scalar() * b),
            self.trivector() | b,
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl BitOr, bitor for Multivector, Vector);

// // Vector-Rotor
// // \[ \vec{a} \cdot R\]
// impl BitOr<Rotor> for Vector {
//     type Output = Multivector;
//     fn bitor(self: Vector, b: Rotor) -> Multivector {
//         Multivector::new(
//             0.0,
//             (self * b.scalar()) + (self | b.bivector()),
//             Bivector::zero(),
//             Trivector::zero(),
//         )
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Vector, Rotor);

// // Rotor-Vector
// // \[ R \cdot \vec{b}\]
// impl BitOr<Vector> for Rotor {
//     type Output = Multivector;
//     fn bitor(self: Rotor, b: Vector) -> Multivector {
//         Multivector::new(
//             0.0,
//             (self.scalar() * b) + (self.bivector() | b),
//             Bivector::zero(),
//             Trivector::zero(),
//         )
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Rotor, Vector);

// Bivector-Bivector
// \[ \overset\Rightarrow{a} \cdot \overset\Rightarrow{b} = \left <\overset\Rightarrow{a} \overset\Rightarrow{b} \right>_0 \]
impl BitOr for Bivector {
    type Output = f32;
    fn bitor(self: Bivector, b: Bivector) -> f32 {
        -self.e12() * b.e12() - self.e31() * b.e31() - self.e23() * b.e23()
    }
}
forward_ref_binop!(impl BitOr, bitor for Bivector, Bivector);

// Bivector-Trivector
// \[ \overset\Rightarrow{a} \cdot \overset\Rrightarrow{b}\]
impl BitOr<Trivector> for Bivector {
    type Output = Vector;
    fn bitor(self: Bivector, b: Trivector) -> Vector {
        Vector::new(
            -self.e23() * b.e123(),
            -self.e31() * b.e123(),
            -self.e12() * b.e123(),
        )
    }
}
forward_ref_binop!(impl BitOr, bitor for Bivector, Trivector);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a} \cdot \overset\Rightarrow{b}\]
impl BitOr<Bivector> for Trivector {
    type Output = Vector;
    fn bitor(self: Trivector, b: Bivector) -> Vector {
        Vector::new(
            self.e123() * -b.e23(),
            self.e123() * -b.e31(),
            self.e123() * -b.e12(),
        )
    }
}
forward_ref_binop!(impl BitOr, bitor for Trivector, Bivector);

// Bivector-Multivector
// \[ \overset\Rightarrow{a} \cdot B\]
impl BitOr<Multivector> for Bivector {
    type Output = Multivector;
    fn bitor(self: Bivector, b: Multivector) -> Multivector {
        Multivector::new(
            self | b.bivector(),
            (self | b.vector()) + (self | b.trivector()),
            self * b.scalar(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl BitOr, bitor for Bivector, Multivector);

// Multivector-Bivector
// \[ A \cdot \overset\Rightarrow{b}\]
impl BitOr<Bivector> for Multivector {
    type Output = Multivector;
    fn bitor(self: Multivector, b: Bivector) -> Multivector {
        Multivector::new(
            self.bivector() | b,
            (self.vector() | b) + (self.trivector() | b),
            self.scalar() * b,
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl BitOr, bitor for Multivector, Bivector);

// // Bivector-Rotor
// // \[ \overset\Rightarrow{a} \cdot R\]
// impl BitOr<Rotor> for Bivector {
//     type Output = Multivector;
//     fn bitor(self: Bivector, b: Rotor) -> Multivector {
//         let scalar = self | b.bivector();
//         let vector = Vector::zero();
//         let bivector = self | b.scalar();
//         let trivector = Trivector::zero();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Bivector, Rotor);

// // Rotor-Bivector
// // \[ R \cdot \overset\Rightarrow{b}\]
// impl BitOr<Bivector> for Rotor {
//     type Output = Multivector;
//     fn bitor(self: Rotor, b: Bivector) -> Multivector {
//         let scalar = self.bivector() | b;
//         let vector = Vector::zero();
//         let bivector = self.scalar() | b;
//         let trivector = Trivector::zero();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Rotor, Bivector);

// Trivector-Trivector
// In 3D there the geometric product of two trivectors is there inner product
// \[ \overset\Rrightarrow{a} \cdot \overset\Rrightarrow{b} = \left <\overset\Rrightarrow{a} \overset\Rrightarrow{b} \right>_0 = \overset\Rrightarrow{a} \overset\Rrightarrow{b} \]
impl BitOr for Trivector {
    type Output = f32;
    fn bitor(self: Trivector, b: Trivector) -> f32 {
        -self.e123() * b.e123()
    }
}

// Trivector-Multivector
// \[ \overset\Rrightarrow{a} \cdot B\]
impl BitOr<Multivector> for Trivector {
    type Output = Multivector;
    fn bitor(self: Trivector, b: Multivector) -> Multivector {
        Multivector::new(
            self | b.trivector(),
            self | b.bivector(),
            self | b.vector(),
            self * b.scalar(),
        )
    }
}
forward_ref_binop!(impl BitOr, bitor for Trivector, Multivector);

// Multivector-Trivector
// \[ A \cdot \overset\Rrightarrow{b}\]
impl BitOr<Trivector> for Multivector {
    type Output = Multivector;
    fn bitor(self: Multivector, b: Trivector) -> Multivector {
        Multivector::new(
            self.trivector() | b,
            self.bivector() | b,
            self.vector() | b,
            self.scalar() * b,
        )
    }
}
forward_ref_binop!(impl BitOr, bitor for Multivector, Trivector);

// // Trivector-Rotor
// // \[ \overset\Rrightarrow{a} \cdot R\]
// impl BitOr<Rotor> for Trivector {
//     type Output = Multivector;
//     fn bitor(self: Trivector, b: Rotor) -> Multivector {
//         let scalar = 0.0;
//         let vector = self | b.bivector();
//         let bivector = Bivector::zero();
//         let trivector = self | b.scalar();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Trivector, Rotor);

// // Rotor-Trivector
// // \[ R \cdot \overset\Rrightarrow{b}\]
// impl BitOr<Trivector> for Rotor {
//     type Output = Multivector;
//     fn bitor(self: Rotor, b: Trivector) -> Multivector {
//         let scalar = 0.0;
//         let vector = self.bivector() | b;
//         let bivector = Bivector::zero();
//         let trivector = self.scalar() | b;
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Rotor, Trivector);

// Multivector-Multivector
// // \[ A \cdot B = \left <A B \right>_{|a-b|} \]
impl BitOr for Multivector {
    type Output = Multivector;
    fn bitor(self: Multivector, b: Multivector) -> Multivector {
        (self * b.scalar()) + (self | b.vector()) + (self | b.bivector()) + (self | b.trivector())
    }
}
forward_ref_binop!(impl BitOr, bitor for Multivector, Multivector);

// // Multivector-Rotor
// // \[ A \cdot R\]
// impl BitOr<Rotor> for Multivector {
//     type Output = Multivector;
//     fn bitor(self: Multivector, b: Rotor) -> Multivector {
//         let scalar = (self.scalar() * b.scalar()) + (self.bivector() | b.bivector());
//         let vector = (self.vector() | b.scalar())
//             + (self.vector() | b.bivector())
//             + (self.trivector() | b.bivector());
//         let bivector = (self.bivector() | b.scalar()) + (self.scalar() | b.bivector());
//         let trivector = self.trivector() | b.scalar();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Multivector, Rotor);

// // Rotor-Multivector
// // \[ R \cdot B\]
// impl BitOr<Multivector> for Rotor {
//     type Output = Multivector;
//     fn bitor(self: Rotor, b: Multivector) -> Multivector {
//         let scalar = (self.scalar() * b.scalar()) + (self.bivector() | b.bivector());
//         let vector = (self.scalar() ^ b.vector())
//             + (self.bivector() | b.vector())
//             + (self.bivector() | b.trivector());
//         let bivector = (self.scalar() | b.bivector()) + (self.bivector() | b.scalar());
//         let trivector = self.scalar() | b.trivector();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Rotor, Multivector);

// // Inner Product
// // \[ R_1 \cdot R_2\]
// impl BitOr for Rotor {
//     type Output = Multivector;
//     fn bitor(self: Rotor, b: Rotor) -> Multivector {
//         let scalar = (self.scalar() * b.scalar()) + (self.bivector() | b.bivector());
//         let vector = Vector::zero();
//         let bivector = (self.scalar() | b.bivector()) + (self.bivector() | b.scalar());
//         let trivector = Trivector::zero();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl BitOr, bitor for Rotor, Rotor);

#[cfg(test)]
mod inner_product {
    use super::*;
    use approx::assert_relative_eq;

    fn vector_vector() {
        // 3e1+5e2+4e3
        let vector1 = Vector::new(3.0, 5.0, 4.0);
        // 2e1+1e2+6e3
        let vector2 = Vector::new(2.0, 1.0, 6.0);
        let scalar = vector1 | vector2;
        assert_relative_eq!(scalar, 35.0, max_relative = 0.000001);
    }

    #[test]
    fn vector_bivector() {
        // 2e1+e2+6e3
        let vector = Vector::new(2.0, 1.0, 6.0);
        // 3e12+5e31+4e23
        let bivector = Bivector::new(3.0, 5.0, 4.0);
        let vector_res = vector | bivector;
        // 27e1​-18e2​-6e3
        assert_relative_eq!(vector_res.e1(), 27.0, max_relative = 0.000001);
        assert_relative_eq!(vector_res.e2(), -18.0, max_relative = 0.000001);
        assert_relative_eq!(vector_res.e3(), -6.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_vector() {
        // 3e12+5e31+4e23
        let bivector = Bivector::new(3.0, 5.0, 4.0);
        // 2e1+e2+6e3
        let vector = Vector::new(2.0, 1.0, 6.0);
        let vector_res = bivector | vector;
        // -27e1​+18e2​+6e3
        assert_relative_eq!(vector_res.e1(), -27.0, max_relative = 0.000001);
        assert_relative_eq!(vector_res.e2(), 18.0, max_relative = 0.000001);
        assert_relative_eq!(vector_res.e3(), 6.0, max_relative = 0.000001);
    }

    #[test]
    fn vector_trivector() {
        // 2e1+e2+6e3
        let vector = Vector::new(2.0, 1.0, 6.0);
        // 3e123
        let trivector = Trivector::new(3.0);
        let res = vector | trivector;
        // 18e12​+3e31​+6e23
        assert_relative_eq!(res.e12(), 18.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 6.0, max_relative = 0.000001);
    }

    #[test]
    fn trivector_vector() {
        // 3e123
        let trivector = Trivector::new(3.0);
        // 2e1+e2+6e3
        let vector = Vector::new(2.0, 1.0, 6.0);
        let res = trivector | vector;
        // 18e12​+3e31​+6e23
        assert_relative_eq!(res.e12(), 18.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 3.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 6.0, max_relative = 0.000001);
    }

    #[test]
    fn vector_multivector() {
        // 2e1+e2+6e3
        let vector = Vector::new(2.0, 1.0, 6.0);
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let multivector = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        let res = vector | multivector;
        // 49+29e1​-28e2​+36e3​+42e12​+7e31​+14e23
        assert_relative_eq!(res.scalar(), 49.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), 29.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), -28.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 36.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 42.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 7.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 14.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 0.0, max_relative = 0.000001);
    }

    #[test]
    fn multivector_vector() {
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let multivector = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        // 2e1+e2+6e3
        let vector = Vector::new(2.0, 1.0, 6.0);
        let res = multivector | vector;
        // 49-5e1​+40e2​+36e3​+42e12​+7e31​+14e23
        assert_relative_eq!(res.scalar(), 49.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), -5.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 40.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 36.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 42.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 7.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 14.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 0.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_bivector() {
        // 3e12+5e31+4e23
        let bivector1 = Bivector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let bivector2 = Bivector::new(2.0, 1.0, 6.0);
        let scalar = bivector1 | bivector2;
        assert_relative_eq!(scalar, -35.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_trivector() {
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        // 3e123
        let trivector = Trivector::new(3.0);
        let res = bivector | trivector;
        // -18e1​-3e2-6e3
        assert_relative_eq!(res.e1(), -18.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), -3.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), -6.0, max_relative = 0.000001);
    }

    #[test]
    fn trivector_bivector() {
        // 3e123
        let trivector = Trivector::new(3.0);
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        let res = trivector | bivector;
        // -18e1-3e2-6e3
        assert_relative_eq!(res.e1(), -18.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), -3.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), -6.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_multivector() {
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let multivector = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        let res = bivector | multivector;
        // -66-32e1​-e2​-47e3​+12e12​+6e31​+36e23​
        assert_relative_eq!(res.scalar(), -66.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), -32.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), -1.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), -47.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 12.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 36.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 0.0, max_relative = 0.000001);
    }

    #[test]
    fn multivector_bivector() {
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let multivector = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, -1.0, 6.0);
        let res = multivector | bivector;
        // -58-60e1​+e2​+37e3​+12e12​-6e31​+36e23​
        assert_relative_eq!(res.scalar(), -58.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), -60.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), 1.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), 37.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 12.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), -6.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 36.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 0.0, max_relative = 0.000001);
    }

    #[test]
    fn trivec_trivec() {
        let trivector1 = Trivector::new(3.0);
        let trivector2 = Trivector::new(6.0);
        let res = trivector1 | trivector2;
        assert_relative_eq!(res, -18.0, max_relative = 0.000001);
    }

    #[test]
    fn trivector_multivector() {
        // 2e123
        let trivector = Trivector::new(2.0);
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let multivector = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        let res = trivector | multivector;
        // -14-16e1​-8e2​-14e3​+8e12​+14e31​+18e23​+12e123​
        assert_relative_eq!(res.scalar(), -14.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), -16.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), -8.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), -14.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 8.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 14.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 18.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 12.0, max_relative = 0.000001);
    }

    #[test]
    fn multivector_trivector() {
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let multivector = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        // 2e123
        let trivector = Trivector::new(2.0);
        let res = multivector | trivector;
        // -14-16e1​-8e2​-14e3​+8e12​+14e31​+18e23​+12e123​
        assert_relative_eq!(res.scalar(), -14.0, max_relative = 0.000001);
        assert_relative_eq!(res.e1(), -16.0, max_relative = 0.000001);
        assert_relative_eq!(res.e2(), -8.0, max_relative = 0.000001);
        assert_relative_eq!(res.e3(), -14.0, max_relative = 0.000001);
        assert_relative_eq!(res.e12(), 8.0, max_relative = 0.000001);
        assert_relative_eq!(res.e31(), 14.0, max_relative = 0.000001);
        assert_relative_eq!(res.e23(), 18.0, max_relative = 0.000001);
        assert_relative_eq!(res.e123(), 12.0, max_relative = 0.000001);
    }

    #[test]
    fn mvec_mvec_inner() {
        // 6.0+ 9.0e1 +7.0e2+ 4.0e3+ 7.0e12 + 4.0e31 +8.0e23+ 7.0e123
        let mvec1 = Multivector::new_components(6.0, 9.0, 7.0, 4.0, 7.0, 4.0, 8.0, 7.0);
        // 5.0+ 8.0e1+ 7.0e2+ 3.0e3+ 2.0e12+ 8.0e31+ 2.0e23+ 1.0e123
        let mvec2 = Multivector::new_components(5.0, 8.0, 7.0, 3.0, 2.0, 8.0, 2.0, 1.0);
        let mvec_res = mvec1 | mvec2;
        // 94+126e1​−5e2​−65e3​+72e12​−124e13​+117e23​+41e123
        assert_relative_eq!(mvec_res.scalar(), 94.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), 126.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), -5.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), -65.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 72.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 124.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 117.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 41.0, max_relative = 0.000001);
    }

    #[test]
    fn negetive_mvec_mvec_inner() {
        // let mvec1 = GaMultivector::new_mvec(-6.0, -8.0, -4.0, -1.0, -6.0, -4.0, -8.0, -5.0);
        let mvec1 = Multivector::new_components(-4.0, -1.0, -3.0, -2.0, -9.0, -6.0, -3.0, -10.0);
        let mvec2 = Multivector::new_components(-4.0, -2.0, -4.0, -9.0, -2.0, -1.0, -7.0, -1.0);
        let mvec_res = mvec1 | mvec2;
        // −7−83e1​+9e2​+35e3​+136e12​−71e13​+61e23​+44e123
        assert_relative_eq!(mvec_res.scalar(), -7.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e1(), -83.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e2(), 9.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e3(), 35.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e12(), 136.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e31(), 71.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e23(), 61.0, max_relative = 0.000001);
        assert_relative_eq!(mvec_res.e123(), 44.0, max_relative = 0.000001);
    }
}
