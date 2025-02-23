#![allow(dead_code)]
#![allow(unused_imports)]

//
mod vector;
use rotor::VGA3DRotor;
pub use vector::VGA3DVector;
//
mod bivector;
pub use bivector::VGA3DBivector;
//
mod trivector;
pub use trivector::VGA3DTrivector;
//
mod multivector;
pub use multivector::VGA3DMultivector;
//
mod rotor;
// Interactions
mod addition;
mod exterior;
mod geometric;
mod inner;
mod regressive;
mod subtraction;
// Functions
mod functions;

pub trait VGA3DOps {
    fn reverse(self) -> Self;
    // fn dual(&self) -> Self;
    fn conjugate(self) -> Self;
    fn involute(self) -> Self;
    fn norm(self) -> f32;
    fn inverse(self) -> Self;
}

pub trait VGA3DOpsRef {
    fn reverse(&self) -> Self;
    // fn dual(&&self) -> Self;
    fn conjugate(&self) -> Self;
    fn involute(&self) -> Self;
    fn norm(&self) -> f32;
    fn inverse(&self) -> Self;
}
