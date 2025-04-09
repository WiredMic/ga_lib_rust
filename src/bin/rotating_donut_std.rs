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

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::time::{SystemTime, UNIX_EPOCH};

use libm::ceilf;

use core::f32::consts::TAU;

// Geometric Algebra
use ga_lib::vga3d::{
    Bivector, Multivector, Projectable, Rejectable, Rotatable, Rotor, Scalar, Trivector, VGA3DOps,
    VGA3DOpsRef, Vector,
};

// define constants
const ESC: char = 27 as char;

const minorRadius: f32 = 1.0;
const majorRadius: f32 = 3.0;
const ANGULARV1: f32 = 3.5;
const ANGULARV2: f32 = 2.0;
const NUM_POINT_IN_CIRC: usize = 144;
const NUM_CIRC_IN_TORUS: usize = 144;
const NUM_POINT_IN_TORUS: usize = NUM_POINT_IN_CIRC * NUM_CIRC_IN_TORUS;

// x direction
const DONUT_WIDTH: i32 = 25;
const DONUT_WIDTH_BUFFER: i32 = 10;
// y direction
const DONUT_HEIGTH: i32 = ((DONUT_WIDTH as f32) / 2.0) as i32;
const DONUT_HEIGTH_BUFFER: i32 = 5;

// Screen
const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 30;

fn main() {
    let mut start_millis = millis();
    print!("{}[?25l", ESC);
    print!("{}[2J", ESC); // turn off cursor
    print!("{}[1:1H", ESC); // place cursor at 1,1

    let point_array = make_torus();
    let mut angle3: f32 = 0.0;
    let mut angle4: f32 = 0.0;

    loop {
        let delta_time = (millis() - start_millis) as f32 / 1000.0; // seconds
        print!("{}[30;1H", ESC);
        println!("fps: {}", (1.0 / delta_time) as u32);
        start_millis = millis();

        let mut torus_array = point_array;

        // add angle form angular velocity
        angle3 += ANGULARV1 * delta_time;
        if angle3 > TAU {
            angle3 -= (angle3 as i32 % TAU as i32) as f32 * TAU;
        }

        angle4 += ANGULARV2 * delta_time;
        if angle4 > TAU {
            angle4 -= (angle4 as i32 % TAU as i32) as f32 * TAU;
        }
        rotate_torus(&mut torus_array, angle3, angle4);

        let screen = project_torus(&torus_array);

        display_torus(screen);
    }
}

fn make_torus() -> [Multivector<f32>; NUM_POINT_IN_TORUS] {
    let mut point_array: [Multivector<f32>; NUM_POINT_IN_TORUS] =
        [Multivector::zero(); NUM_POINT_IN_TORUS];

    // First point with tangent plane
    // These are rotated togther
    let first_vec = Vector::new(1.0, 0.0, 0.0);
    let first_bivec = first_vec.dual();
    let first = (first_vec * Scalar::new(minorRadius)) + first_bivec;

    // Translate vector to translate the circle from origin to the correct place in torus
    let translate_vec = Vector::new(majorRadius, 0.0, 0.0);

    // Angle and rotion plane for circle
    let angle1 = TAU / (NUM_POINT_IN_CIRC as f32);
    let rotation_plane1 = Bivector::new(1.0, 0.0, 0.0);

    // Angle and rotation plane for torus
    let angle2 = TAU / NUM_CIRC_IN_TORUS as f32;
    let rotation_plane2 = Bivector::new(0.0, 1.0, 0.0);

    for i in 0..NUM_CIRC_IN_TORUS {
        // rotation around in the e3e1 plane
        let rotor2 = Rotor::new((angle2 * i as f32) / 2.0, rotation_plane2);

        // Rotate translation vector
        // R_2^\dag\vec{r}R_2
        let translate_vec_rot = (translate_vec).rotate(rotor2);

        // println!("trans{}:", i);
        // println!(
        //     "e1: {},e2: {},e3: {}",
        //     translate_vec_rot.e1(),
        //     translate_vec_rot.e2(),
        //     translate_vec_rot.e3()
        // );
        for j in 0..NUM_POINT_IN_CIRC {
            // rotation around the e1e2 plane
            let rotor1 = Rotor::new((angle1 * j as f32) / 2.0, rotation_plane1);

            // The rotor that rotates from first point to the point in the torus
            // \[R_1R_2\]
            let rotor12 = rotor1 * rotor2;
            let multivector_rot = first.rotate(rotor12);

            // translate the rotated point to the correct place in the torus
            // add point and tangent plane
            // \[ (R_1R_2)^\dag \vec{v} R_1R_2 + R_2^\dag\vec{r}R_2\]
            let multivector_point = multivector_rot + translate_vec_rot;

            point_array[j + i * NUM_POINT_IN_CIRC] = multivector_point;
        }
    }

    point_array
}

fn rotate_torus(
    point_array: &mut [Multivector<f32>; NUM_POINT_IN_TORUS],
    angle3: f32,
    angle4: f32,
) {
    let rotation_plane3 = Bivector::new(5.0, 3.5, -4.0);
    let rotor3 = Rotor::new(angle3 / 2.0, rotation_plane3);

    let rotation_plane4 = Bivector::new(-6.0, 2.0, 7.2);
    let rotor4 = Rotor::new(angle4 / 2.0, rotation_plane4);

    let rotor5 = rotor3 * rotor4;

    for multivector in point_array.iter_mut() {
        *multivector = multivector.rotate(rotor5);
    }
}

struct Screen {
    brightness: [[char; SCREEN_WIDTH]; SCREEN_HEIGHT], // inner is x, outer is y, [y][x]
    zbuffer: [[f32; SCREEN_WIDTH]; SCREEN_HEIGHT],     // inner is x, outer is y, [y][x]
}

impl Screen {
    fn new() -> Self {
        // Create a screen with all spaces and zero zbuffer
        Screen {
            brightness: [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT],
            zbuffer: [[0.0; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    // Method to set individual characters
    fn set_char(&mut self, x: usize, y: usize, c: char) {
        if y < SCREEN_HEIGHT && x < SCREEN_WIDTH {
            self.brightness[y][x] = c;
        }
    }

    // Method to set zbuffer value
    fn set_zbuffer(&mut self, x: usize, y: usize, value: f32) {
        if y < SCREEN_HEIGHT && x < SCREEN_WIDTH {
            self.zbuffer[y][x] = value;
        }
    }
}

fn project_torus(point_array: &[Multivector<f32>; NUM_POINT_IN_TORUS]) -> Screen {
    let mut screen = Screen::new();

    // \[\vec{v}_\parallel=(\vec{v}\cdot\vec{B})\vec{B}^{-1} \]
    // plane of projecting
    let screen_plane = Bivector::new(0.0, 1.0, 0.0);

    // light vector
    let light_ray_direction = Vector::new(1.0, 0.2, 1.0);
    let light_ray_direction = light_ray_direction / light_ray_direction.norm();

    for point in point_array.iter() {
        // Project point vector onto the screen plane
        let point_proj = point.project(screen_plane);

        // Scale the x and y value fit the sceeen
        let x: usize =
            ((ceilf(point_proj.e1() * (DONUT_WIDTH as f32) / (minorRadius * 2.0 + majorRadius)))
                as i32
                + DONUT_WIDTH
                + DONUT_WIDTH_BUFFER) as usize;
        let y: usize = (DONUT_HEIGTH + DONUT_HEIGTH_BUFFER
            - (ceilf(point_proj.e3() * (DONUT_HEIGTH as f32) / (minorRadius * 2.0 + majorRadius)))
                as i32) as usize;

        // In order to find the brigtness of the pixel
        // The way to find the brigtness is to make a unit light vector and then
        // take the wedge product of the unit tangent plane the a pixel. The volume
        // of the resulting trivector is the cosinus angle between the light vector
        // and the plane. 0 > then the surface is facing the light 0 < then the
        // surface is not facing Everything in between is some inbetween brigtness

        // The tanget plane was added at the start and has been rototed the same as
        // the point vectors

        // There are 12 brightness values
        // The wedge product gives values form -1 to 1
        // ceilf((+ 1) * 6) gives decret values from 1 to 12;
        let brightness = ceilf(((light_ray_direction ^ point).e123() + 1.0) * 6.0) as i32;

        // (int)ceilf((WedgeProduct(lightRay.mvec, pointArray[i].mvec).mvec[7] + 1) * 6);

        // .,-~:;=!*#$@
        let brightness_pixel = match brightness {
            1 => '.',
            2 => ',',
            3 => '-',
            4 => '~',
            5 => ':',
            6 => ';',
            7 => '=',
            8 => '!',
            9 => '*',
            10 => '#',
            11 => '$',
            12 => '@',
            _ => ' ',
        };

        // z bufer
        // this is the rejected vector
        // \[\vec{v}_perp = (\vec{v}\wedge\vec{B})\vec{B}^{-1} \]
        let vector_rej = point.reject(screen_plane);

        let z_buffer = match vector_rej.e2() {
            0.0 => 0.01, // If exactly 0.0, use 0.01
            z => z,      // Otherwise use the actual value
        };

        if z_buffer > screen.zbuffer[y][x] || screen.zbuffer[y][x] == 0.0 {
            screen.set_zbuffer(x, y, z_buffer);
            screen.set_char(x, y, brightness_pixel);
        }
    }

    screen
}

fn display_torus(screen: Screen) {
    // Move cursor to top-left but DON'T clear the screen here
    print!("{}[1;1H", ESC);

    // Print one complete frame without clearing
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            print!("{}", screen.brightness[y][x]);
        }
        // Move cursor to beginning of next line instead of rprintln
        // This avoids potential problems with line endings
        if y < SCREEN_HEIGHT - 1 {
            print!("{}[{}H", ESC, y + 2);
        }
    }

    // Force output to be sent by finishing with a newline
    println!();
}

// Timing
fn millis() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => return n.as_millis() as u64,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
