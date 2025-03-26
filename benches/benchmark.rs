use std::f32::consts::TAU;

use criterion::{criterion_group, criterion_main, Criterion};
use ga_lib::vga3d::{Bivector, Multivector, Rotatable, Rotor, Trivector, Vector};

fn rotate() {
    let vector = Vector::new(3.0, 0.0, 0.0);
    let angle = TAU / 4.0;
    let bivector = Bivector::new(1.0, 0.0, 0.0);
    let rotor = Rotor::new(angle / 2.0, bivector);
    let _rot = vector.rotate(rotor);
}

fn rotor_vec_mul_impl() {
    let angle = TAU / 4.0;
    let rotation_plane = Bivector::new(4.0, 2.0, -3.0);
    let rotor = Rotor::new(angle / 2.0, rotation_plane);
    // 2e12+e31+6e23
    let vector = Vector::new(2.0, 1.0, 6.0);
    // 0.7071+0.5252e12+0.2626e31-0.3939e23
    let _res = rotor * vector;
}

fn vec_rotor_mul_impl() {
    let angle = TAU / 4.0;
    let rotation_plane = Bivector::new(4.0, 2.0, -3.0);
    let rotor = Rotor::new(angle / 2.0, rotation_plane);
    // 2e12+e31+6e23
    let vector = Vector::new(2.0, 1.0, 6.0);
    // 0.7071+0.5252e12+0.2626e31-0.3939e23
    let _res = vector * rotor;
}

fn bench(c: &mut Criterion) {
    c.bench_function("vec_rot 100", |b| b.iter(|| rotate()));
    c.bench_function("rotor_vec_impl", |b| b.iter(|| rotor_vec_mul_impl()));
    c.bench_function("vec_rotor_impl", |b| b.iter(|| vec_rotor_mul_impl()));
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(500);
    targets = bench
}
criterion_main!(benches);
