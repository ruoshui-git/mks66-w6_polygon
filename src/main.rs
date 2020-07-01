mod graphics;

use graphics::{
    matrix::{projections, transform, Matrix},
    PPMImg,
};

// # compilation:
// cargo run --release
// convert -delay 10 img{1..9}.ppm img{8..2}.ppm perspectives.gif

fn main() {
    let mut img = PPMImg::new(500, 500, 225);
    let total = 9;
    let mut mv = 150.;
    for i in 1..=total {
        let mut model = Matrix::new_edge_matrix();
        model.add_sphere((110., 110., 100.), 75.);
        model.add_sphere((-110., 100., 100.), 75.);
        model.add_box((-80., -120., 100.), 75., 75., 75.);
        model.add_torus((-30., -335., 100.), 25., 175.);
        
        if i < 6 { mv += 150. } else { mv -= 150. };

        let t = Matrix::ident(4)
            .mul(&transform::rotatey(10. * i as f64 - total as f64 * 5.))
            .mul(&transform::mv(0., 0., mv))
            ;
        let model = model.mul(&t);
        // now apply perspective
        let mut model = model.mul(&projections::perspective(90., 1., 1., 600.));
        model.correct_projection();
        img.render_ndc_edges_n1to1(&model);
        img.write_binary(format!("img{}.ppm", i).as_str()).expect("Error writing to file");
        img.clear();
    }
}
