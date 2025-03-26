#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use ga_lib::vga3d::{Bivector, Vector};

fn main() {
    println!("No Problems");
}

// Test
#[cfg(test)]
mod triple_product {

    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn bivector() {
        // \[ \overset\Rightarrow{a}\times (\overset\Rightarrow{b}\times\overset\Rightarrow{c})=-(\overset\Rightarrow{a}\cdot \overset\Rightarrow{c})\overset\Rightarrow{b}+(\overset\Rightarrow{a}\cdot \overset\Rightarrow{b})\overset\Rightarrow{c} \]
        // 3e12+5e31+4e23
        let a = Bivector::new(3.0, 5.0, 4.0);
        // 2e12+e31+6e23
        let b = Bivector::new(2.0, 1.0, 6.0);
        // 4e12-e31-3e23
        let c = Bivector::new(4.0, -1.0, -3.0);

        let prod = a.cross(b.cross(c));
        let res = -((a | c) * b - (a | b) * c);
        assert_relative_eq!(prod.e12(), res.e12(), max_relative = 0.000001);
        assert_relative_eq!(prod.e31(), res.e31(), max_relative = 0.000001);
        assert_relative_eq!(prod.e23(), res.e23(), max_relative = 0.000001);
    }

    #[test]
    fn cross_cross() {
        // \[\overset\Rightarrow{c} =- \overset\Rightarrow{a} \times \overset\Rightarrow{b} \]
        // 3e1+5e2+4e3
        let a = Vector::new(3.0, 5.0, 4.0);
        // 2e1+e2+6e3
        let b = Vector::new(2.0, 1.0, 6.0);

        let res_vec = a.cross(b).dual();
        let res_bivec = -a.dual().cross(b.dual());
        assert_relative_eq!(res_vec.e12(), res_bivec.e12(), max_relative = 0.000001);
        assert_relative_eq!(res_vec.e31(), res_bivec.e31(), max_relative = 0.000001);
        assert_relative_eq!(res_vec.e23(), res_bivec.e23(), max_relative = 0.000001);
    }
}
