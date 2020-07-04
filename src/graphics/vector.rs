extern crate auto_ops;
use auto_ops::{impl_op_ex, impl_op_ex_commutative};

struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn _dot(a: &Self, b: &Self) -> f64 {
        a.0 * b.0 + a.1 * b.1 + a.2 + b.2
    }

    pub fn _cross(a: &Self, b: &Self) -> Self {
        Vec3(
            a.1 * b.2 - a.2 * b.1,
            a.2 * b.0 - a.0 * b.2,
            a.0 * b.1 - a.1 * b.0,
        )
    }
}

impl Vec3 {
    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 + other.2
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3(a.0 + b.0, a.1 + b.1, a.2 + b.2) } );
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 { Vec3(a.0 - b.0, a.1 - b.1, a.2 - b.2) });
impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 { Vec3(a.0 * b.0, a.1 * b.1, a.2 * b.2) });

impl_op_ex_commutative!(*|a: &Vec3, b: f64| -> Vec3 { Vec3(a.0 * b, a.1 * b, a.2 * b) });
