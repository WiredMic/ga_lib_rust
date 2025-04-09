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

#[doc(hidden)]
#[macro_export]
macro_rules! forward_ref_binop {
    // Version for when both types share the same generic parameter
    (impl<$param:ident : $bound:path> $imp:ident, $method:ident for $t:ty, $t2:ty) => {
        impl<'a, $param: $bound> $imp<$t2> for &'a $t {
            type Output = <$t as $imp<$t2>>::Output;

            #[inline]
            fn $method(self, other: $t2) -> <$t as $imp<$t2>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<$param: $bound> $imp<&$t2> for $t {
            type Output = <$t as $imp<$t2>>::Output;

            #[inline]
            fn $method(self, other: &$t2) -> <$t as $imp<$t2>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b, $param: $bound> $imp<&'b $t2> for &'a $t {
            type Output = <$t as $imp<$t2>>::Output;

            #[inline]
            fn $method(self, other: &'b $t2) -> <$t as $imp<$t2>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };

    // Original version without generics
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b> $imp<&'b $u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'b $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}
