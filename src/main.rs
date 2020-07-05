mod graphics;

use graphics::{
    matrix::{projections, transform, Matrix},
    canvas::Canvas,
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
        let mut model = Matrix::new_polygon_matrix();
        model.add_sphere((130., 110., 100.), 120.);
        model.add_sphere((-130., 100., 100.), 120.);
        model.add_box((-60., -60., 100.), 90., 90., 90.);
        model.add_torus((-30., -335., 100.), 25., 200.);

        
        let step = 150.;
        if i < 6 {
            mv += step
        } else {
            mv -= step
        };

        model *= Matrix::ident(4)
            * transform::rotatey(10. * i as f64 - total as f64 * 5.)
            * transform::mv(0., 0., mv);

        // now apply perspective
        let mut model = model._mul(&projections::perspective(90., 1., 1., 600.));
        model.perspective_divide();
        model.ndc_n1to1_to_device(img.width() as f64, img.height() as f64);
        img.render_polygon_matrix(&model);
        img.write_binary(format!("img{}.ppm", i).as_str())
            .expect("Error writing to file");
        img.clear();
    }
}
