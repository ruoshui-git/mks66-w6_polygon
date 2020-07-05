#![allow(dead_code)]
/// Goes through the file named filename and performs all of the actions listed in that file.
/// The file follows the following format:
///      Every command is a single character that takes up a line
///      Any command that requires arguments must have those arguments in the second line.
///      The commands are as follows:
/// 	 circle: add a circle to the edge matrix -
/// 	         takes 4 arguments (cx, cy, cz, r)
/// 	 hermite: add a hermite curve to the edge matrix -
/// 	          takes 8 arguments (x0, y0, x1, y1, rx0, ry0, rx1, ry1)
/// 	 bezier: add a bezier curve to the edge matrix -
/// 	         takes 8 arguments (x0, y0, x1, y1, x2, y2, x3, y3)
///          line: add a line to the edge matrix -
///                takes 6 arguemnts (x0, y0, z0, x1, y1, z1)
///          ident: set the transform matrix to the identity matrix -
///          scale: create a scale matrix,
///                 then multiply the transform matrix by the scale matrix -
///                 takes 3 arguments (sx, sy, sz)
///          move: create a translation matrix,
///                then multiply the transform matrix by the translation matrix -
///                takes 3 arguments (tx, ty, tz)
///          rotate: create a rotation matrix,
///                  then multiply the transform matrix by the rotation matrix -
///                  takes 2 arguments (axis, theta) axis should be x y or z
///          apply: apply the current transformation matrix to the edge matrix
///          display: clear the screen, then
///                   draw the lines of the edge matrix to the screen
///                   display the screen
///          save: clear the screen, then
///                draw the lines of the edge matrix to the screen
///                save the screen to a file -
///                takes 1 argument (file name)
///          quit: end parsing
/// See the file script for an example of the file format
///
// :( Oh my God! This script spec is designed in a way that a parser library is generally useless!!!
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    process::Command,
};

use crate::graphics::{matrix::transform, utils, Matrix, PPMImg, Canvas};

pub struct DWScript {
    filename: String,
    edges: Matrix,
    trans: Matrix,
    polygons: Matrix,
    img: PPMImg,
    tmpfile_name: String,
}

/// Advances a line iterator and panic on error
fn getline_or_error(
    line: &mut impl Iterator<Item = (usize, io::Result<String>)>,
) -> (usize, String) {
    if let Some((num, line)) = line.next() {
        let line = line.expect("Error while reading line").trim().to_string();
        (num, line)
    } else {
        panic!("Error reading line");
    }
}

/// Parse floats from a line and return them in a vec. Panic on error.
fn parse_floats(line: String) -> Vec<f64> {
    line.split(' ')
        .map(|x| x.parse::<f64>().expect("Error parsing numbers"))
        .collect()
}

impl DWScript {
    pub fn new(filename: &str) -> Self {
        DWScript {
            filename: filename.to_string(),
            edges: Matrix::new_edge_matrix(),
            polygons: Matrix::new_polygon_matrix(),
            trans: Matrix::ident(4),
            img: PPMImg::new(500, 500, 255),
            tmpfile_name: String::from("tmp.ppm"),
        }
    }

    pub fn do_parse(&mut self) {
        let _f = File::open(&self.filename).expect("Error opening file");
        let f = BufReader::new(_f);
        let mut lines = f.lines().enumerate();
        while let Some((num, line)) = lines.next() {
            let line = line.expect("Error while reading file");
            match line.trim() {
                x if x.is_empty() || x.starts_with("\\") || x.starts_with("#") => {}
                "line" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let pts: Vec<f64> = parse_floats(dline);
                    assert_eq!(6, pts.len());
                    self.edges.append_edge(&pts);
                }
                "ident" => {
                    self.trans = Matrix::ident(4);
                }
                "scale" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let scale: Vec<f64> = parse_floats(dline);
                    assert_eq!(3, scale.len());
                    self.trans *= transform::scale(scale[0], scale[1], scale[2]);
                }
                "move" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let mv: Vec<f64> = parse_floats(dline);
                    assert_eq!(3, mv.len());
                    self.trans *= transform::mv(mv[0], mv[1], mv[2]);
                }
                "rotate" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v: Vec<&str> = dline.split(' ').collect();
                    let (scale, deg): (&str, f64) =
                        (v[0], v[1].parse().expect("Error parsing number"));
                    self.trans *= match scale {
                        "x" => transform::rotatex(deg),
                        "y" => transform::rotatey(deg),
                        "z" => transform::rotatez(deg),
                        _ => panic!("Unknown rotation axis on line {}", _dnum),
                    };
                }
                "apply" => {
                    self.edges *= &self.trans;
                    self.polygons *= &self.trans;
                }
                "display" => {
                    self.img.clear();
                    self.img.render_edge_matrix(&self.edges);
                    self.img.render_polygon_matrix(&self.polygons);
                    utils::display_ppm(&self.img);
                }
                "save" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    self.img.clear();
                    self.img.render_edge_matrix(&self.edges);
                    self.img.render_polygon_matrix(&self.polygons);
                    self.img
                        .write_binary(dline.as_str())
                        .expect("Error writing to file");

                    // if a .png is wanted, then convert to .png
                    if dline.ends_with(".png") {
                        Command::new("convert")
                            .arg(dline.as_str())
                            .arg(dline.as_str())
                            .spawn()
                            .unwrap();
                    }
                }
                "circle" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let values = parse_floats(dline);
                    assert_eq!(4, values.len());
                    self.edges
                        .add_circle((values[0], values[1], values[2]), values[3]);
                }
                "hermite" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(8, v.len());
                    self.edges
                        .add_hermite3((v[0], v[1]), (v[2], v[3]), (v[4], v[5]), (v[6], v[7]));
                }
                "bezier" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(8, v.len());
                    self.edges
                        .add_bezier3((v[0], v[1]), (v[2], v[3]), (v[4], v[5]), (v[6], v[7]));
                }
                "box" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(6, v.len());
                    self.polygons.add_box((v[0], v[1], v[2]), v[3], v[4], v[5]);
                }
                "sphere" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(4, v.len());
                    self.polygons.add_sphere((v[0], v[1], v[2]), v[3]);
                }
                "torus" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(5, v.len());
                    self.polygons.add_torus((v[0], v[1], v[2]), v[3], v[4]);
                }
                "clear" => {
                    self.edges.clear();
                    self.polygons.clear();
                }
                _ => panic!("Unrecognized command on line {}: {}", num, line),
            }
        }
        // (self.edges.clone(), self.polygons.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn script() {
        DWScript::new("script").do_parse();
    }
}
