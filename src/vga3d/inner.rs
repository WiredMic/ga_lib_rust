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

use core::ops::BitOr;

// // Scalar-Scalar
// // \[ a \cdot b\]
// impl BitOr for f32 {
//     type Output = f32;
//     fn bitor(self: f32, b: Vector) -> f32 {
//         self * b
//     }
// }

// Scalar-Vector
// \[ a \cdot \vec{b}\]
impl BitOr<Vector> for f32 {
    type Output = Vector;
    fn bitor(self: f32, b: Vector) -> Vector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for f32, Vector);

// Vector-Scalar
// \[ \vec{a} \cdot b\]
impl BitOr<f32> for Vector {
    type Output = Vector;
    fn bitor(self: Vector, b: f32) -> Vector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for Vector, f32);

// Scalar-Bivector
// \[ s \cdot \overset\Rightarrow{b}\]
impl BitOr<Bivector> for f32 {
    type Output = Bivector;
    fn bitor(self: f32, b: Bivector) -> Bivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for f32, Bivector);

// Bivector-Scalar
// \[ \overset\Rightarrow{a} \cdot b\]
impl BitOr<f32> for Bivector {
    type Output = Bivector;
    fn bitor(self: Bivector, b: f32) -> Bivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for Bivector,f32);

// Scalar-Trivector
// \[ s \cdot \overset\Rrightarrow{b}\]
impl BitOr<Trivector> for f32 {
    type Output = Trivector;
    fn bitor(self: f32, b: Trivector) -> Trivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for f32, Trivector);

// Trivector-Scalar
// \[ \overset\Rrightarrow{a} \cdot b\]
impl BitOr<f32> for Trivector {
    type Output = Trivector;
    fn bitor(self: Trivector, b: f32) -> Trivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for Trivector, f32);

// Scalar-Multivector
// \[ s \cdot B\]
impl BitOr<Multivector> for f32 {
    type Output = Multivector;
    fn bitor(self: f32, b: Multivector) -> Multivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for f32, Multivector);

// Multivector-Scalar
// \[ B \cdot b\]
impl BitOr<f32> for Multivector {
    type Output = Multivector;
    fn bitor(self: Multivector, b: f32) -> Multivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for Multivector, f32);

// Scalar-Rotor
// \[ s \cdot R\]
impl BitOr<VGA3DRotor> for f32 {
    type Output = Multivector;
    fn bitor(self: f32, b: VGA3DRotor) -> Multivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for f32, VGA3DRotor);

//Rotor-Scalar
// \[ R \cdot b\]
impl BitOr<f32> for VGA3DRotor {
    type Output = Multivector;
    fn bitor(self: VGA3DRotor, b: f32) -> Multivector {
        self * b
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DRotor, f32);

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
        let scalar = self | b.vector();
        let vector = (self | b.bivector()) + (self | b.scalar());
        let bivector = self | b.trivector();
        let trivector = Trivector::zero();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for Vector, Multivector);

// Multivector-Vector
// \[ A \cdot \vec{b}\]
impl BitOr<Vector> for Multivector {
    type Output = Multivector;
    fn bitor(self: Multivector, b: Vector) -> Multivector {
        let scalar = self.vector() | b;
        let vector = (self.bivector() | b) + (self.scalar() | b);
        let bivector = self.trivector() | b;
        let trivector = Trivector::zero();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for Multivector, Vector);

// Vector-Rotor
// \[ \vec{a} \cdot R\]
impl BitOr<VGA3DRotor> for Vector {
    type Output = Multivector;
    fn bitor(self: Vector, b: VGA3DRotor) -> Multivector {
        Multivector::new(
            0.0,
            (self | b.scalar()) + (self | b.bivector()),
            Bivector::zero(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl BitOr, bitor for Vector, VGA3DRotor);

// Rotor-Vector
// \[ R \cdot \vec{b}\]
impl BitOr<Vector> for VGA3DRotor {
    type Output = Multivector;
    fn bitor(self: VGA3DRotor, b: Vector) -> Multivector {
        Multivector::new(
            0.0,
            (self.scalar() | b) + (self.bivector() | b),
            Bivector::zero(),
            Trivector::zero(),
        )
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DRotor, Vector);

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
        let e1 = -self.e23() * b.e123();
        let e2 = -self.e31() * b.e123();
        let e3 = -self.e12() * b.e123();
        Vector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl BitOr, bitor for Bivector, Trivector);

// Trivector-Bivector
// \[ \overset\Rrightarrow{a} \cdot \overset\Rightarrow{b}\]
impl BitOr<Bivector> for Trivector {
    type Output = Vector;
    fn bitor(self: Trivector, b: Bivector) -> Vector {
        let e1 = self.e123() * -b.e23();
        let e2 = self.e123() * -b.e31();
        let e3 = self.e123() * -b.e12();
        Vector::new(e1, e2, e3)
    }
}
forward_ref_binop!(impl BitOr, bitor for Trivector, Bivector);

// Bivector-Multivector
// \[ \overset\Rightarrow{a} \cdot B\]
impl BitOr<Multivector> for Bivector {
    type Output = Multivector;
    fn bitor(self: Bivector, b: Multivector) -> Multivector {
        let scalar = self | b.bivector();
        let vector = (self | b.vector()) + (self | b.trivector());
        let bivector = self | b.scalar();
        let trivector = Trivector::zero();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for Bivector, Multivector);

// Multivector-Bivector
// \[ A \cdot \overset\Rightarrow{b}\]
impl BitOr<Bivector> for Multivector {
    type Output = Multivector;
    fn bitor(self: Multivector, b: Bivector) -> Multivector {
        let scalar = self.bivector() | b;
        let vector = (self.vector() | b) + (self.trivector() | b);
        let bivector = self.scalar() | b;
        let trivector = Trivector::zero();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for Multivector, Bivector);

// Bivector-Rotor
// \[ \overset\Rightarrow{a} \cdot R\]
impl BitOr<VGA3DRotor> for Bivector {
    type Output = Multivector;
    fn bitor(self: Bivector, b: VGA3DRotor) -> Multivector {
        let scalar = self | b.bivector();
        let vector = Vector::zero();
        let bivector = self | b.scalar();
        let trivector = Trivector::zero();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for Bivector, VGA3DRotor);

// Rotor-Bivector
// \[ R \cdot \overset\Rightarrow{b}\]
impl BitOr<Bivector> for VGA3DRotor {
    type Output = Multivector;
    fn bitor(self: VGA3DRotor, b: Bivector) -> Multivector {
        let scalar = self.bivector() | b;
        let vector = Vector::zero();
        let bivector = self.scalar() | b;
        let trivector = Trivector::zero();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DRotor, Bivector);

// Trivector-Trivector
// In 3D there the geometric product of two trivectors is there inner product
// \[ \overset\Rrightarrow{a} \cdot \overset\Rrightarrow{b} = \left <\overset\Rrightarrow{a} \overset\Rrightarrow{b} \right>_0 = \overset\Rrightarrow{a} \overset\Rrightarrow{b} \]
impl BitOr for Trivector {
    type Output = f32;
    fn bitor(self: Trivector, b: Trivector) -> f32 {
        self * b
    }
}

// Trivector-Multivector
// \[ \overset\Rrightarrow{a} \cdot B\]
impl BitOr<Multivector> for Trivector {
    type Output = Multivector;
    fn bitor(self: Trivector, b: Multivector) -> Multivector {
        let scalar = self | b.trivector();
        let vector = self | b.bivector();
        let bivector = self | b.vector();
        let trivector = self | b.scalar();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for Trivector, Multivector);

// Multivector-Trivector
// \[ A \cdot \overset\Rrightarrow{b}\]
impl BitOr<Trivector> for Multivector {
    type Output = Multivector;
    fn bitor(self: Multivector, b: Trivector) -> Multivector {
        let scalar = self.trivector() | b;
        let vector = self.bivector() | b;
        let bivector = self.vector() | b;
        let trivector = self.scalar() | b;
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for Multivector, Trivector);

// Trivector-Rotor
// \[ \overset\Rrightarrow{a} \cdot R\]
impl BitOr<VGA3DRotor> for Trivector {
    type Output = Multivector;
    fn bitor(self: Trivector, b: VGA3DRotor) -> Multivector {
        let scalar = 0.0;
        let vector = self | b.bivector();
        let bivector = Bivector::zero();
        let trivector = self | b.scalar();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for Trivector, VGA3DRotor);

// Rotor-Trivector
// \[ R \cdot \overset\Rrightarrow{b}\]
impl BitOr<Trivector> for VGA3DRotor {
    type Output = Multivector;
    fn bitor(self: VGA3DRotor, b: Trivector) -> Multivector {
        let scalar = 0.0;
        let vector = self.bivector() | b;
        let bivector = Bivector::zero();
        let trivector = self.scalar() | b;
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DRotor, Trivector);

// Multivector-Multivector
// // \[ A \cdot B = \left <A B \right>_{|a-b|} \]
impl BitOr for Multivector {
    type Output = Multivector;
    fn bitor(self: Multivector, b: Multivector) -> Multivector {
        (self | b.scalar()) + (self | b.vector()) + (self | b.bivector()) + (self | b.trivector())
    }
}
forward_ref_binop!(impl BitOr, bitor for Multivector, Multivector);

// Multivector-Rotor
// \[ A \cdot R\]
impl BitOr<VGA3DRotor> for Multivector {
    type Output = Multivector;
    fn bitor(self: Multivector, b: VGA3DRotor) -> Multivector {
        let scalar = (self.scalar() * b.scalar()) + (self.bivector() | b.bivector());
        let vector = (self.vector() | b.scalar())
            + (self.vector() | b.bivector())
            + (self.trivector() | b.bivector());
        let bivector = (self.bivector() | b.scalar()) + (self.scalar() | b.bivector());
        let trivector = self.trivector() | b.scalar();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for Multivector, VGA3DRotor);

// Rotor-Multivector
// \[ R \cdot B\]
impl BitOr<Multivector> for VGA3DRotor {
    type Output = Multivector;
    fn bitor(self: VGA3DRotor, b: Multivector) -> Multivector {
        let scalar = (self.scalar() * b.scalar()) + (self.bivector() | b.bivector());
        let vector = (self.scalar() ^ b.vector())
            + (self.bivector() | b.vector())
            + (self.bivector() | b.trivector());
        let bivector = (self.scalar() | b.bivector()) + (self.bivector() | b.scalar());
        let trivector = self.scalar() | b.trivector();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DRotor, Multivector);

// Inner Product
// \[ R_1 \cdot R_2\]
impl BitOr for VGA3DRotor {
    type Output = Multivector;
    fn bitor(self: VGA3DRotor, b: VGA3DRotor) -> Multivector {
        let scalar = (self.scalar() * b.scalar()) + (self.bivector() | b.bivector());
        let vector = Vector::zero();
        let bivector = (self.scalar() | b.bivector()) + (self.bivector() | b.scalar());
        let trivector = Trivector::zero();
        Multivector::new(scalar, vector, bivector, trivector)
    }
}
forward_ref_binop!(impl BitOr, bitor for VGA3DRotor, VGA3DRotor);

#[cfg(test)]
mod inner {
    use super::*;
    use approx::assert_relative_eq;

    fn vector_vector_dot() {
        // 3e1+5e2+4e3
        let vector1 = Vector::new(3.0, 5.0, 4.0);
        // 2e1+1e2+6e3
        let vector2 = Vector::new(2.0, 1.0, 6.0);
        let scalar = vector1 | vector2;
        assert_relative_eq!(scalar, 35.0, max_relative = 0.000001);
    }

    #[test]
    fn bivector_vector_inner() {
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
    fn vector_bivector_inner() {
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
    fn bivector_bivector_inner() {
        // 3e12+5e31+4e23
        let bivector1 = Bivector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let bivector2 = Bivector::new(2.0, 1.0, 6.0);
        let scalar = bivector1 | bivector2;
        assert_relative_eq!(scalar, -35.0, max_relative = 0.000001);
    }

    #[test]
    fn trivec_trivec_inner() {
        let trivector1 = Trivector::new(3.0);
        let trivector2 = Trivector::new(6.0);
        let res = trivector1 | trivector2;
        assert_relative_eq!(res, -18.0, max_relative = 0.000001);
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
        let mvec1 =
            Multivector::new_components(-4.0, -1.0, -3.0, -2.0, -9.0, -6.0, -3.0, -10.0);
        let mvec2 =
            Multivector::new_components(-4.0, -2.0, -4.0, -9.0, -2.0, -1.0, -7.0, -1.0);
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
