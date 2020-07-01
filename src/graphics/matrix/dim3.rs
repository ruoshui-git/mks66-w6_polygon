//! Implements fn that add shapes to a vertex matrix

use super::Matrix;
use std::f64::consts::PI;

impl Matrix {
    pub fn add_box(&mut self, point: (f64, f64, f64), dx: f64, dy: f64, dz: f64) {
        let (x0, y0, z0) = point;
        self.append_edge(&[x0, y0, z0, x0 + dx, y0, z0]);
        self.append_edge(&[x0, y0, z0, x0, y0 - dy, z0]);
        self.append_edge(&[x0 + dx, y0, z0, x0 + dx, y0 - dy, z0]);
        self.append_edge(&[x0, y0 - dy, z0, x0 + dx, y0 - dy, z0]);

        self.append_edge(&[x0, y0, z0 - dz, x0 + dx, y0, z0 - dz]);
        self.append_edge(&[x0, y0, z0 - dz, x0, y0 - dy, z0 - dz]);
        self.append_edge(&[x0 + dx, y0, z0 - dz, x0 + dx, y0 - dy, z0 - dz]);
        self.append_edge(&[x0, y0 - dy, z0 - dz, x0 + dx, y0 - dy, z0 - dz]);

        self.append_edge(&[x0, y0, z0, x0, y0, z0 - dz]);
        self.append_edge(&[x0 + dx, y0, z0, x0 + dx, y0, z0 - dz]);
        self.append_edge(&[x0, y0 - dy, z0, x0, y0 - dy, z0 - dz]);
        self.append_edge(&[x0 + dx, y0 - dy, z0, x0 + dx, y0 - dy, z0 - dz]);
    }
    

    pub fn add_sphere(&mut self, center: (f64, f64, f64), radius: f64) {
        let mut points = vec![];
        let step = 0.03;
        let (cx, cy, cz) = center;
        //  for rot: 0 -> 1
        //   for cir: 0 -> 1
        //     x = r * cos(π * cir) + Cx
        //     y = r * sin(π * cir) * cos(2π * rot) + Cy
        //     z = r * sin(π * cir) * sin(2π * rot) + Cz
        //
        let mut rot = 0.;
        while rot <= 1. {
            let rotc = rot * 2. * PI;
            let mut cir = 0.;
            while cir <= 1. {
                let circ = cir * PI;
                let x = radius * circ.cos() + cx;
                let y = radius * circ.sin() * rotc.cos() + cy;
                let z = radius * circ.sin() * rotc.sin() + cz;
                points.push((x, y, z));
                cir += step;
            }
            rot += step;
        }

        // add all points to matrix
        for point in points {
            let (x, y, z) = point;
            self.append_edge(&[x, y, z, x + 1., y, z]);
        }
    }

    /// radius1: inner radius
    /// radius2: big radius
    pub fn add_torus(&mut self, center: (f64, f64, f64), radius1: f64, radius2: f64) {
        let mut points = vec![];
        // x = cos(p) * (rcos(t) + R) + Cx
        // y = rsin(t) + Cy
        // z = -sin(p) * (rcos(t) + R) + Cz
        let step = 0.03;
        let (cx, cy, cz) = center;
        let mut torus_angle_norm = 0.;
        while torus_angle_norm <= 1. {
            let torus_angle = torus_angle_norm * 2. * PI;
            let mut circ_angle_norm = 0.;
            while circ_angle_norm <= 1. {
                let circ_angle = circ_angle_norm * 2. * PI;
                let x = circ_angle.cos() * (radius1 * torus_angle.cos() + radius2) + cx;
                let y = radius1 * torus_angle.sin() + cy;
                let z = -circ_angle.sin() * (radius1 * torus_angle.cos() + radius2) + cz;
                points.push((x, y, z));
                circ_angle_norm += step;
            }
            torus_angle_norm += step;
        }

        // add all points to matrix
        for point in points {
            let (x, y, z) = point;
            self.append_edge(&[x, y, z, x + 1., y, z]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::{matrix::transform, utils::display_matrix};

    #[test]
    fn draw_sphere() {
        let mut m = Matrix::new_edge_matrix();
        m.add_sphere((250., 250., 0.), 100.);
        display_matrix(&m, false);
    }
    #[test]
    fn draw_torus() {
        let mut m = Matrix::new_edge_matrix();
        m.add_torus((250., 250., 0.), 10., 100.);
        display_matrix(&m, false);
    }
    #[test]
    fn draw_cube() {
        let mut m = Matrix::new_edge_matrix();
        m.add_box((10., 10., 0.), 100., 100., 100.);
        let t = transform::rotatex(20.).mul(&transform::rotatey(20.));
        display_matrix(&m.mul(&t), false);
    }
}
