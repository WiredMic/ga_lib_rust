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

use core::ops::BitXor;

// // Scalar-Scalar
// // \[ a\wedge b\]
// impl<F:Float> BitXor for F {
//     type Output = F;
//     fn bitxor(self: F, b: Vector<F>) -> F {
//         self * b
//     }
// }

// // Scalar-Vector
// // \[ a\wedge \vec{b}\]
// impl<F:Float> BitXor<Vector<F>> for F {
//     type Output = Vector<F>;
//     fn bitxor(self: F, b: Vector<F>) -> Vector<F> {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for F, Vector<F>);

// // Vector-Scalar
// // \[ \vec{a}\wedge b\]
// impl<F:Float> BitXor<F> for Vector<F> {
//     type Output = Vector<F>;
//     fn bitxor(self: Vector<F>, b: F) -> Vector<F> {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Vector<F>, F);

// // Scalar-Bivector
// // \[ s\wedge \overset\Rightarrow{b}\]
// impl<F:Float> BitXor<Bivector<F>> for F {
//     type Output = Bivector<F>;
//     fn bitxor(self: F, b: Bivector<F>) -> Bivector<F> {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for F, Bivector<F>);

// // Bivector-Scalar
// // \[ \overset\Rightarrow{a}\wedge b\]
// impl<F:Float> BitXor<F> for Bivector<F> {
//     type Output = Bivector<F>;
//     fn bitxor(self: Bivector<F>, b: F) -> Bivector<F> {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Bivector<F>, F);

// // Scalar-Trivector
// // \[ s\wedge \overset\Rrightarrow{b}\]
// impl<F:Float> BitXor<Trivector<F>> for F {
//     type Output = Trivector<F>;
//     fn bitxor(self: F, b: Trivector<F>) -> Trivector<F> {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for F, Trivector<F>);

// // Trivector-Scalar
// // \[ \overset\Rrightarrow{a}\wedge b\]
// impl<F:Float> BitXor<F> for Trivector<F> {
//     type Output = Trivector<F>;
//     fn bitxor(self: Trivector<F>, b: F) -> Trivector<F> {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Trivector<F>, F);

// // Scalar-Multivector
// // \[ s\wedge B\]
// impl<F:Float> BitXor<Multivector<F>> for F {
//     type Output = Multivector<F>;
//     fn bitxor(self: F, b: Multivector<F>) -> Multivector<F> {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for F, Multivector<F>);

// // Multivector-Scalar
// // \[ B\wedge b\]
// impl<F:Float> BitXor<F> for Multivector<F> {
//     type Output = Multivector<F>;
//     fn bitxor(self: Multivector<F>, b: F) -> Multivector<F> {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Multivector<F>, F);

// // Scalar-Rotor
// // \[ s\wedge R\]
// impl<F:Float> BitXor<Rotor<F>> for F {
//     type Output = Multivector<F>;
//     fn bitxor(self: F, b: Rotor<F>) -> Multivector<F> {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for F, Rotor<F>);

// //Rotor-Scalar
// // \[ R\wedge b\]
// impl<F:Float> BitXor<F> for Rotor<F> {
//     type Output = Multivector<F>;
//     fn bitxor(self: Rotor<F>, b: F) -> Multivector<F> {
//         self * b
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Rotor<F>, F);

// Vector-Vector
impl<F: Float> BitXor for Vector<F> {
    type Output = Bivector<F>;
    fn bitxor(self: Vector<F>, b: Vector<F>) -> Bivector<F> {
        Bivector::new(
            self.e1() * b.e2() - self.e2() * b.e1(),
            self.e3() * b.e1() - self.e1() * b.e3(),
            self.e2() * b.e3() - self.e3() * b.e2(),
        )
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Vector<F>, Vector<F>);

// Vector-Bivector
// \[ \vec{a}\wedge \overset\Rightarrow{b}\]
impl<F: Float> BitXor<Bivector<F>> for Vector<F> {
    type Output = Trivector<F>;
    fn bitxor(self: Vector<F>, b: Bivector<F>) -> Trivector<F> {
        Trivector::new(self.e1() * b.e23() + self.e2() * b.e31() + self.e3() * b.e12())
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Vector<F>, Bivector<F>);

// Bivector-Vector
// \[ \overset\Rightarrow{b}\wedge \vec{b}\]
impl<F: Float> BitXor<Vector<F>> for Bivector<F> {
    type Output = Trivector<F>;
    fn bitxor(self: Bivector<F>, b: Vector<F>) -> Trivector<F> {
        Trivector::new(self.e23() * b.e1() + self.e31() * b.e2() + self.e12() * b.e3())
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Bivector<F>, Vector<F>);

// Vector-Trivector
// \[ \vec{a}\wedge \overset\Rrightarrow{b}\]
impl<F: Float> BitXor<Trivector<F>> for Vector<F> {
    type Output = Scalar<F>;
    fn bitxor(self: Vector<F>, _b: Trivector<F>) -> Scalar<F> {
        Scalar::zero()
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Vector<F>, Trivector<F>);

// Trivector-Vector
// \[ \overset\Rrightarrow{a}\wedge \vec{b}\]
impl<F: Float> BitXor<Vector<F>> for Trivector<F> {
    type Output = Scalar<F>;
    fn bitxor(self: Trivector<F>, _b: Vector<F>) -> Scalar<F> {
        Scalar::zero()
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Trivector<F>, Vector<F>);

// Vector-Multivector
// \[ \vec{a}\wedge B\]
impl<F: Float> BitXor<Multivector<F>> for Vector<F> {
    type Output = Multivector<F>;
    fn bitxor(self: Vector<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector {
            scalar: self ^ b.trivector(),
            vector: self * b.scalar,
            bivector: self ^ b.vector,
            trivector: self ^ b.bivector,
        }
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Vector<F>, Multivector<F>);

// Multivector-Vector
// \[ A\wedge \vec{b}\]
impl<F: Float> BitXor<Vector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn bitxor(self: Multivector<F>, b: Vector<F>) -> Multivector<F> {
        Multivector {
            scalar: self.trivector() ^ b,
            vector: self.scalar * b,
            bivector: self.vector() ^ b,
            trivector: self.bivector() ^ b,
        }
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Multivector<F>, Vector<F>);

// // Vector-Rotor
// // \[ \vec{a} \wedge R\]
// impl<F:Float> BitXor<Rotor<F>> for Vector<F> {
//     type Output = Multivector<F>;
//     fn bitxor(self: Vector<F>, b: Rotor<F>) -> Multivector<F> {
//         Multivector::new(
//             F::zero(),
//             self ^ b.scalar(),
//             Bivector::zero(),
//             self ^ b.bivector(),
//         )
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Vector<F>, Rotor<F>);

// // Rotor-Vector
// // \[ R\wedge \vec{b}\]
// impl<F:Float> BitXor<Vector<F>> for Rotor<F> {
//     type Output = Multivector<F>;
//     fn bitxor(self: Rotor<F>, b: Vector<F>) -> Multivector<F> {
//         Multivector::new(
//             F::zero(),
//             self.scalar() ^ b,
//             Bivector::zero(),
//             self.bivector() ^ b,
//         )
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Rotor<F>, Vector<F>);

// Bivector-Bivector
// \[ \overset\Rightarrow{a} \wedge \overset\Rightarrow{b} = \left <\overset\Rightarrow{a} \overset\Rightarrow{b} \right>_4 \]
// There is no object of grade 4 in 3D VGA
impl<F: Float> BitXor for Bivector<F> {
    type Output = Scalar<F>;
    fn bitxor(self: Bivector<F>, _b: Bivector<F>) -> Scalar<F> {
        Scalar::zero()
    }
}

// Bivector-Trivector
// \[ \overset\Rightarrow{a}\wedge \overset\Rrightarrow{b}\]
impl<F: Float> BitXor<Trivector<F>> for Bivector<F> {
    type Output = Scalar<F>;
    fn bitxor(self: Bivector<F>, _b: Trivector<F>) -> Scalar<F> {
        Scalar::zero()
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Bivector<F>, Trivector<F>);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a}\wedge \overset\Rightarrow{b}\]
impl<F: Float> BitXor<Bivector<F>> for Trivector<F> {
    type Output = Scalar<F>;
    fn bitxor(self: Trivector<F>, _b: Bivector<F>) -> Scalar<F> {
        Scalar::zero()
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Trivector<F>, Bivector<F>);

// Bivector-Multivector
// \[ \overset\Rightarrow{a}\wedge B\]
impl<F: Float> BitXor<Multivector<F>> for Bivector<F> {
    type Output = Multivector<F>;
    fn bitxor(self: Bivector<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(
            (self ^ b.bivector()) + (self ^ b.trivector()),
            Vector::zero(),
            self * Scalar(b.scalar()),
            self ^ b.vector(),
        )
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Bivector<F>, Multivector<F>);

// Multivector-Bivector
// \[ A\wedge \overset\Rightarrow{b}\]
impl<F: Float> BitXor<Bivector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn bitxor(self: Multivector<F>, b: Bivector<F>) -> Multivector<F> {
        Multivector::new(
            (self.bivector() ^ b) + (self.trivector() ^ b),
            Vector::zero(),
            Scalar(self.scalar()) * b,
            self.vector() ^ b,
        )
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Multivector<F>, Bivector<F>);

// // Bivector-Rotor
// // \[ \overset\Rightarrow{a}\wedge R\]
// impl<F:Float> BitXor<Rotor<F>> for Bivector<F> {
//     type Output = Multivector<F>;
//     fn bitxor(self: Bivector<F>, b: Rotor<F>) -> Multivector<F> {
//         Multivector::new(
//             self ^ b.bivector(),
//             Vector::zero(),
//             self * b.scalar(),
//             Trivector::zero(),
//         )
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Bivector<F>, Rotor<F>);

// // Rotor-Bivector
// // \[ R\wedge \overset\Rightarrow{b}\]
// impl<F:Float> BitXor<Bivector<F>> for Rotor<F> {
//     type Output = Multivector<F>;
//     fn bitxor(self: Rotor<F>, b: Bivector<F>) -> Multivector<F> {
//         Multivector::new(
//             self.bivector() ^ b,
//             Vector::zero(),
//             self.scalar() * b,
//             Trivector::zero(),
//         )
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Rotor<F>, Bivector<F>);

// Trivector-Trivector
// \[ \overset\Rrightarrow{a} \wedge \overset\Rrightarrow{b} = \left <\overset\Rrightarrow{a} \overset\Rrightarrow{b} \right>_4 \]
// There is no object of grade 6 in 3D VGA
impl<F: Float> BitXor for Trivector<F> {
    type Output = Scalar<F>;
    fn bitxor(self: Trivector<F>, _b: Trivector<F>) -> Scalar<F> {
        Scalar::zero()
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Trivector<F>, Trivector<F>);

// Trivector-Multivector
// \[ \overset\Rrightarrow{a}\wedge B\]
impl<F: Float> BitXor<Multivector<F>> for Trivector<F> {
    type Output = Multivector<F>;
    fn bitxor(self: Trivector<F>, b: Multivector<F>) -> Multivector<F> {
        Multivector::new(
            (self ^ b.vector()) + (self ^ b.bivector()) + (self ^ b.trivector).0,
            Vector::zero(),
            Bivector::zero(),
            self * Scalar(b.scalar()),
        )
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Trivector<F>, Multivector<F>);

// Multivector-Trivector
// \[ A\wedge \overset\Rrightarrow{b}\]
impl<F: Float> BitXor<Trivector<F>> for Multivector<F> {
    type Output = Multivector<F>;
    fn bitxor(self: Multivector<F>, b: Trivector<F>) -> Multivector<F> {
        Multivector::new(
            (self.vector() ^ b) + (self.bivector() ^ b) + (self.trivector ^ b).0,
            Vector::zero(),
            Bivector::zero(),
            Scalar(self.scalar()) * b,
        )
    }
}
forward_ref_binop!(impl<F:Float> BitXor, bitxor for Multivector<F>, Trivector<F>);

// // Trivector-Rotor
// // \[ \overset\Rrightarrow{a}\wedge R\]
// impl<F:Float> BitXor<Rotor<F>> for Trivector<F> {
//     type Output = Multivector<F>;
//     fn bitxor(self: Trivector<F>, b: Rotor<F>) -> Multivector<F> {
//         let scalar = self ^ b.bivector();
//         let vector = Vector::zero();
//         let bivector = Bivector::zero();
//         let trivector = self ^ b.scalar();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Trivector<F>, Rotor<F>);

// // Rotor-Trivector
// // \[ R\wedge \overset\Rrightarrow{b}\]
// impl<F:Float> BitXor<Trivector<F>> for Rotor<F> {
//     type Output = Multivector<F>;
//     fn bitxor(self: Rotor<F>, b: Trivector<F>) -> Multivector<F> {
//         Multivector::new(
//             self.bivector() ^ b,
//             Vector::zero(),
//             Bivector::zero(),
//             self.scalar() ^ b,
//         )
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Rotor<F>, Trivector<F>);

// Multivector-Multivector
// \[ A \wedge B = \left <A B \right>_{a+b} \]
impl<F: Float> BitXor for Multivector<F> {
    type Output = Multivector<F>;
    fn bitxor(self: Multivector<F>, b: Multivector<F>) -> Multivector<F> {
        (self * Scalar(b.scalar()))
            + (self ^ b.vector())
            + (self ^ b.bivector())
            + (self ^ b.trivector())
    }
}

// // Multivector-Rotor
// // \[ A\wedge R\]
// impl<F:Float> BitXor<Rotor<F>> for Multivector<F> {
//     type Output = Multivector<F>;
//     fn bitxor(self: Multivector<F>, b: Rotor<F>) -> Multivector<F> {
//         let scalar = (self.scalar() * b.scalar())
//             + (self.bivector() ^ b.bivector())
//             + (self.trivector() ^ b.bivector());
//         let vector = self.vector() ^ b.scalar();
//         let bivector = (self.bivector() ^ b.scalar()) + (self.scalar() ^ b.bivector());
//         let trivector = (self.trivector() ^ b.scalar()) + (self.vector() ^ b.bivector());
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Multivector<F>, Rotor<F>);

// // Rotor-Multivector
// // \[ R\wedge B\]
// impl<F:Float> BitXor<Multivector<F>> for Rotor<F> {
//     type Output = Multivector<F>;
//     fn bitxor(self: Rotor<F>, b: Multivector<F>) -> Multivector<F> {
//         let scalar = (self.scalar() * b.scalar())
//             + (self.bivector() ^ b.bivector())
//             + (self.bivector() ^ b.trivector());
//         let vector = self.scalar() ^ b.vector();
//         let bivector = (self.scalar() ^ b.bivector()) + (self.bivector() ^ b.scalar());
//         let trivector = (self.scalar() ^ b.trivector()) + (self.bivector() ^ b.vector());
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Rotor<F>, Multivector<F>);

// // Rotor-Rotor
// // \[ R_1 \wedge R_2\]
// impl<F:Float> BitXor for Rotor<F> {
//     type Output = Multivector<F>;
//     fn bitxor(self: Rotor<F>, b: Rotor<F>) -> Multivector<F> {
//         let scalar = (self.scalar() * b.scalar()) + (self.bivector() ^ b.bivector());
//         let vector = Vector::zero();
//         let bivector = (self.scalar() ^ b.bivector()) + (self.bivector() ^ b.scalar());
//         let trivector = Trivector::zero();
//         Multivector::new(scalar, vector, bivector, trivector)
//     }
// }
// forward_ref_binop!(impl<F:Float> BitXor, bitxor for Rotor<F>, Rotor<F>);

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
        let res = (vector ^ trivector).scalar();
        // 0.0
        assert_relative_eq!(res, 0.0, max_relative = 0.000001);
    }

    #[test]
    fn trivector_vector() {
        // 3e123
        let trivector = Trivector::new(3.0);
        // 2e1+e2+6e3
        let vector = Vector::new(2.0, 1.0, 6.0);
        let res = (trivector ^ vector).scalar();
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
        assert_relative_eq!(
            (bivector1 ^ bivector2).scalar(),
            0.0,
            max_relative = 0.000001
        );
    }

    #[test]
    fn bivector_trivector() {
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        // 3e123
        let trivector = Trivector::new(3.0);
        let res = (bivector ^ trivector).scalar();
        // 0
        assert_relative_eq!(res, 0.0, max_relative = 0.000001);
    }

    #[test]
    fn trivector_bivector() {
        // 3e123
        let trivector = Trivector::new(3.0);
        // 2e12+e31+6e23
        let bivector = Bivector::new(2.0, 1.0, 6.0);
        let res = (trivector ^ bivector).scalar();
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
        let res = (trivector1 ^ trivector2).scalar();
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
