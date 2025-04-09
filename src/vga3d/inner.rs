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
    scalar::Scalar,
    trivector::{self, Trivector},
    vector::Vector,
};

use num_traits::Float;

use crate::forward_ref_binop;

use core::ops::BitOr;

// Scalar-Scalar
// \[ a \cdot b\]
// impl<F:Float> BitOr for F {
//     type Output = Scalar<F>;
//     fn bitor(self: F, b: Vector) -> Scalar<F> {
//         self * b
//     }
// }

// // Scalar-Vector
// // \[ a \cdot \vec{b}\]
// impl<F:Float> BitOr<Vector> for F {
//     type Output = Vector;
//     fn bitor(self: F, b: Vector) -> Vector {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for F, Vector);

// // Vector-Scalar
// // \[ \vec{a} \cdot b\]
// impl<F:Float> BitOr<F> for Vector {
//     type Output = Vector;
//     fn bitor(self: Vector, b: F) -> Vector {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Vector, F);

// // Scalar-Bivector
// // \[ s \cdot \overset\Rightarrow{b}\]
// impl<F:Float> BitOr<Bivector> for F {
//     type Output = Bivector;
//     fn bitor(self: F, b: Bivector) -> Bivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for F, Bivector);

// // Bivector-Scalar
// // \[ \overset\Rightarrow{a} \cdot b\]
// impl<F:Float> BitOr<F> for Bivector {
//     type Output = Bivector;
//     fn bitor(self: Bivector, b: F) -> Bivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Bivector,F);

// // Scalar-Trivector
// // \[ s \cdot \overset\Rrightarrow{b}\]
// impl<F:Float> BitOr<Trivector> for F {
//     type Output = Trivector;
//     fn bitor(self: F, b: Trivector) -> Trivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for F, Trivector);

// // Trivector-Scalar
// // \[ \overset\Rrightarrow{a} \cdot b\]
// impl<F:Float> BitOr<F> for Trivector {
//     type Output = Trivector;
//     fn bitor(self: Trivector, b: F) -> Trivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Trivector, F);

// // Scalar-Multivector
// // \[ s \cdot B\]
// impl<F:Float> BitOr<Multivector> for F {
//     type Output = Multivector;
//     fn bitor(self: F, b: Multivector) -> Multivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for F, Multivector);

// // Multivector-Scalar
// // \[ B \cdot b\]
// impl<F:Float> BitOr<F> for Multivector {
//     type Output = Multivector;
//     fn bitor(self: Multivector, b: F) -> Multivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Multivector, F);

// // Scalar-Rotor
// // \[ s \cdot R\]
// impl<F:Float> BitOr<Rotor> for F {
//     type Output = Multivector;
//     fn bitor(self: F, b: Rotor) -> Multivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for F, Rotor);

// //Rotor-Scalar
// // \[ R \cdot b\]
// impl<F:Float> BitOr<F> for Rotor {
//     type Output = Multivector;
//     fn bitor(self: Rotor, b: F) -> Multivector {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Rotor, F);

// Vector-Vector
// \[ \vec{u} \cdot \vec{v} = u_1 \cdot v_1 + u_2 \cdot v_2 + u_3 \cdot v_3 \]
impl<F: Float> BitOr for Vector<F> {
    type Output = Scalar<F>;
    fn bitor(self: Vector<F>, b: Vector<F>) -> Scalar<F> {
        Scalar(self.e1() * b.e1() + self.e2() * b.e2() + self.e3() * b.e3())
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Vector<F>, Vector<F>);

// Vector-Bivector
// \[ \vec{a} \cdot \overset\Rightarrow{b}\]
impl<F: Float> BitOr<Bivector<F>> for Vector<F> {
    type Output = Vector<F>;
    fn bitor(self: Vector<F>, b: Bivector<F>) -> Vector<F> {
        let e1 = -self.e2() * b.e12() + self.e3() * b.e31();
        let e2 = self.e1() * b.e12() - self.e3() * b.e23();
        let e3 = -self.e1() * b.e31() + self.e2() * b.e23();
        Vector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Vector<F>, Bivector<F>);

// Bivector-Vector
// \[ \overset\Rightarrow{b} \cdot \vec{b}\]
impl<F: Float> BitOr<Vector<F>> for Bivector<F> {
    type Output = Vector<F>;
    fn bitor(self: Bivector<F>, b: Vector<F>) -> Vector<F> {
        let e1 = self.e12() * b.e2() - self.e31() * b.e3();
        let e2 = -self.e12() * b.e1() + self.e23() * b.e3();
        let e3 = self.e31() * b.e1() - self.e23() * b.e2();
        Vector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Bivector<F>, Vector<F>);

// Vector-Trivector
// \[ \vec{a} \cdot \overset\Rrightarrow{b}\]
impl<F: Float> BitOr<Trivector<F>> for Vector<F> {
    type Output = Bivector<F>;
    fn bitor(self: Vector<F>, b: Trivector<F>) -> Bivector<F> {
        self * b
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Vector<F>, Trivector<F>);

// Trivector-Vector
// \[ \overset\Rrightarrow{a} \cdot \vec{b}\]
impl<F: Float> BitOr<Vector<F>> for Trivector<F> {
    type Output = Bivector<F>;
    fn bitor(self: Trivector<F>, b: Vector<F>) -> Bivector<F> {
        self * b
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Trivector<F>, Vector<F>);

// Vector-Multivector
// \[ \vec{a} \cdot B\]
impl<F: Float> BitOr<Multivector<F>> for Vector<F> {
    type Output = Multivector<F>;
    fn bitor(self: Vector<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(
            (self | b.vector()).0,
            (self | b.bivector()) + (self * Scalar(b.scalar())),
            self | b.trivector(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Vector<F>, Multivector<F>);

// Multivector-Vector
// \[ A \cdot \vec{b}\]
impl<F: Float> BitOr<Vector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn bitor(self: Multivector<F>, b: Vector<F>) -> Multivector<F> {
        Multivector::new(
            (self.vector() | b).0,
            (self.bivector() | b) + (Scalar(self.scalar()) * b),
            self.trivector() | b,
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Multivector<F>, Vector<F>);

// // Vector-Rotor
// // \[ \vec{a} \cdot R\]
// impl<F:Float> BitOr<Rotor> for Vector<F> {
//     type Output = Multivector<F>;
//     fn bitor(self: Vector<F>, b: Rotor) -> Multivector<F> {
//         Multivector::new(
//             F::zero(),
//             (self * b.scalar()) + (self | b.bivector()),
//             Bivector::zero(),
//             Trivector::zero(),
//         )
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Vector<F>, Rotor);

// // Rotor-Vector
// // \[ R \cdot \vec{b}\]
// impl<F:Float> BitOr<Vector<F>> for Rotor {
//     type Output = Multivector<F>;
//     fn bitor(self: Rotor, b: Vector<F>) -> Multivector<F> {
//         Multivector::new(
//             F::zero(),
//             (self.scalar() * b) + (self.bivector() | b),
//             Bivector::zero(),
//             Trivector::zero(),
//         )
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Rotor, Vector<F>);

// Bivector-Bivector
// \[ \overset\Rightarrow{a} \cdot \overset\Rightarrow{b} = \left <\overset\Rightarrow{a} \overset\Rightarrow{b} \right>_0 \]
impl<F: Float> BitOr for Bivector<F> {
    type Output = Scalar<F>;
    fn bitor(self: Bivector<F>, b: Bivector<F>) -> Scalar<F> {
        Scalar(-self.e12() * b.e12() - self.e31() * b.e31() - self.e23() * b.e23())
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Bivector<F>, Bivector<F>);

// Bivector-Trivector
// \[ \overset\Rightarrow{a} \cdot \overset\Rrightarrow{b}\]
impl<F: Float> BitOr<Trivector<F>> for Bivector<F> {
    type Output = Vector<F>;
    fn bitor(self: Bivector<F>, b: Trivector<F>) -> Vector<F> {
        Vector::new(
            -self.e23() * b.e123(),
            -self.e31() * b.e123(),
            -self.e12() * b.e123(),
        )
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Bivector<F>, Trivector<F>);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a} \cdot \overset\Rightarrow{b}\]
impl<F: Float> BitOr<Bivector<F>> for Trivector<F> {
    type Output = Vector<F>;
    fn bitor(self: Trivector<F>, b: Bivector<F>) -> Vector<F> {
        Vector::new(
            self.e123() * -b.e23(),
            self.e123() * -b.e31(),
            self.e123() * -b.e12(),
        )
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Trivector<F>, Bivector<F>);

// Bivector-Multivector
// \[ \overset\Rightarrow{a} \cdot B\]
impl<F: Float> BitOr<Multivector<F>> for Bivector<F> {
    type Output = Multivector<F>;
    fn bitor(self: Bivector<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector {
            scalar: self | b.bivector,
            vector: (self | b.vector) + (self | b.trivector),
            bivector: self * b.scalar,
            trivector: Trivector::zero(),
        }
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Bivector<F>, Multivector<F>);

// Multivector-Bivector
// \[ A \cdot \overset\Rightarrow{b}\]
impl<F: Float> BitOr<Bivector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn bitor(self: Multivector<F>, b: Bivector<F>) -> Multivector<F> {
        Multivector {
            scalar: self.bivector | b,
            vector: (self.vector | b) + (self.trivector | b),
            bivector: self.scalar * b,
            trivector: Trivector::zero(),
        }
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Multivector<F>, Bivector<F>);

// // Bivector-Rotor
// // \[ \overset\Rightarrow{a} \cdot R\]
// impl<F:Float> BitOr<Rotor> for Bivector<F> {
//     type Output = Multivector<F>;
//     fn bitor(self: Bivector<F>, b: Rotor) -> Multivector<F> {
//         let scalar = self | b.bivector();
//         let vector = Vector::zero();
//         let bivector = self | b.scalar();
//         let trivector = Trivector::zero();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Bivector<F>, Rotor);

// // Rotor-Bivector
// // \[ R \cdot \overset\Rightarrow{b}\]
// impl<F:Float> BitOr<Bivector<F>> for Rotor {
//     type Output = Multivector<F>;
//     fn bitor(self: Rotor, b: Bivector<F>) -> Multivector<F> {
//         let scalar = self.bivector() | b;
//         let vector = Vector::zero();
//         let bivector = self.scalar() | b;
//         let trivector = Trivector::zero();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Rotor, Bivector<F>);

// Trivector-Trivector
// In 3D there the geometric product of two trivectors is there inner product
// \[ \overset\Rrightarrow{a} \cdot \overset\Rrightarrow{b} = \left <\overset\Rrightarrow{a} \overset\Rrightarrow{b} \right>_0 = \overset\Rrightarrow{a} \overset\Rrightarrow{b} \]
impl<F: Float> BitOr for Trivector<F> {
    type Output = Scalar<F>;
    fn bitor(self: Trivector<F>, b: Trivector<F>) -> Scalar<F> {
        Scalar(-self.e123() * b.e123())
    }
}

// Trivector-Multivector
// \[ \overset\Rrightarrow{a} \cdot B\]
impl<F: Float> BitOr<Multivector<F>> for Trivector<F> {
    type Output = Multivector<F>;
    fn bitor(self: Trivector<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(
            (self | b.trivector()).0,
            self | b.bivector(),
            self | b.vector(),
            self * Scalar(b.scalar()),
        )
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Trivector<F>, Multivector<F>);

// Multivector-Trivector
// \[ A \cdot \overset\Rrightarrow{b}\]
impl<F: Float> BitOr<Trivector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn bitor(self: Multivector<F>, b: Trivector<F>) -> Multivector<F> {
        Multivector {
            scalar: self.trivector() | b,
            vector: self.bivector() | b,
            bivector: self.vector() | b,
            trivector: self.scalar * b,
        }
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Multivector<F>, Trivector<F>);

// // Trivector-Rotor
// // \[ \overset\Rrightarrow{a} \cdot R\]
// impl<F:Float> BitOr<Rotor> for Trivector<F> {
//     type Output = Multivector<F>;
//     fn bitor(self: Trivector<F>, b: Rotor) -> Multivector<F> {
//         let scalar = F::zero();
//         let vector = self | b.bivector();
//         let bivector = Bivector::zero();
//         let trivector = self | b.scalar();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Trivector<F>, Rotor);

// // Rotor-Trivector
// // \[ R \cdot \overset\Rrightarrow{b}\]
// impl<F:Float> BitOr<Trivector<F>> for Rotor {
//     type Output = Multivector<F>;
//     fn bitor(self: Rotor, b: Trivector<F>) -> Multivector<F> {
//         let scalar = F::zero();
//         let vector = self.bivector() | b;
//         let bivector = Bivector::zero();
//         let trivector = self.scalar() | b;
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Rotor, Trivector<F>);

// Multivector-Multivector
// // \[ A \cdot B = \left <A B \right>_{|a-b|} \]
impl<F: Float> BitOr for Multivector<F> {
    type Output = Multivector<F>;
    fn bitor(self: Multivector<F>, b: Multivector<F>) -> Multivector<F> {
        (self * Scalar(b.scalar()))
            + (self | b.vector())
            + (self | b.bivector())
            + (self | b.trivector())
    }
}
forward_ref_binop!(impl<F:Float> BitOr, bitor for Multivector<F>, Multivector<F>);

// // Multivector-Rotor
// // \[ A \cdot R\]
// impl<F:Float> BitOr<Rotor> for Multivector {
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
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Multivector, Rotor);

// // Rotor-Multivector
// // \[ R \cdot B\]
// impl<F:Float> BitOr<Multivector> for Rotor {
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
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Rotor, Multivector);

// // Inner Product
// // \[ R_1 \cdot R_2\]
// impl<F:Float> BitOr for Rotor {
//     type Output = Multivector;
//     fn bitor(self: Rotor, b: Rotor) -> Multivector {
//         let scalar = (self.scalar() * b.scalar()) + (self.bivector() | b.bivector());
//         let vector = Vector::zero();
//         let bivector = (self.scalar() | b.bivector()) + (self.bivector() | b.scalar());
//         let trivector = Trivector::zero();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitOr, bitor for Rotor, Rotor);

#[cfg(test)]
mod inner_product {
    use super::*;
    use approx::assert_relative_eq;

    fn vector_vector() {
        // 3e1+5e2+4e3
        let vector1 = Vector::new(3.0, 5.0, 4.0);
        // 2e1+1e2+6e3
        let vector2 = Vector::new(2.0, 1.0, 6.0);
        let scalar = (vector1 | vector2).scalar();
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
        let scalar = (bivector1 | bivector2).0;
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
        let res = (trivector1 | trivector2).scalar();
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
