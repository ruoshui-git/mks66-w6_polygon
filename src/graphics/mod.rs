#![allow(dead_code)]

pub mod colors;
pub mod matrix;
pub mod parametrics;
pub mod parser;
pub mod canvas;
pub mod utils;
pub mod vector;

use std::convert::TryInto;

use std::{
    fmt::Debug,
    io::{self, prelude::Write},
};

// re-exports
pub use colors::{HSL, RGB};
pub use matrix::Matrix;
pub use canvas::Canvas;

// internal use
use utils::create_file;

pub struct PPMImg {
    height: u32,
    width: u32,
    depth: u16, // max = 2^16
    pub x_wrap: bool,
    pub y_wrap: bool,
    pub invert_y: bool,
    pub fg_color: RGB,
    pub bg_color: RGB,
    data: Vec<RGB>,
}

/// Two images are eq iff their dimensions, depth, and image data are eq
impl PartialEq for PPMImg {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
            && self.width == other.width
            && self.depth == other.depth
            && self.data == other.data
    }
}

impl Eq for PPMImg {}

impl Debug for PPMImg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PPMImg {{ {} by {}, depth={} }}",
            self.height, self.width, self.depth
        )
    }
}

// impl constructor and exporter
impl PPMImg {
    /// Createa new PPMImg
    /// Default fg color is white, bg_color is lack
    pub fn new(height: u32, width: u32, depth: u16) -> PPMImg {
        Self::new_with_bg(height, width, depth, RGB::gray(0))
    }

    pub fn new_with_bg(height: u32, width: u32, depth: u16, bg_color: RGB) -> PPMImg {
        PPMImg {
            height,
            width,
            depth,
            x_wrap: false,
            y_wrap: false,
            invert_y: false,
            fg_color: RGB::gray(depth),
            bg_color,
            data: vec![bg_color; (width * height).try_into().unwrap()],
        }
    }

    pub fn write_binary(&self, filepath: &str) -> io::Result<()> {
        let mut file = create_file(filepath);
        writeln!(file, "P6")?;
        writeln!(file, "{} {} {}", self.width, self.height, self.depth)?;
        if self.depth < 256 {
            for t in self.data.iter() {
                file.write(&[t.green as u8])?;
                file.write(&[t.green as u8])?;
                file.write(&[t.blue as u8])?;
            }
        } else {
            for t in self.data.iter() {
                file.write_all(&(t.red.to_be_bytes()))?;
                file.write_all(&(t.green.to_be_bytes()))?;
                file.write_all(&(t.blue.to_be_bytes()))?;
            }
        }

        file.flush()?;
        Ok(())
    }
    pub fn write_ascii(&self, filepath: &str) -> io::Result<()> {
        let mut file = create_file(filepath);
        writeln!(file, "P3")?;
        writeln!(file, "{} {} {}", self.width, self.height, self.depth)?;
        for t in self.data.iter() {
            writeln!(file, "{} {} {}", t.red, t.green, t.blue)?;
        }
        file.flush()?;
        Ok(())
    }
}

// clear
impl PPMImg {
    pub fn clear(&mut self) {
        let bg = self.bg_color;
        for d in self.data.iter_mut() {
            *d = bg;
        }
    }
}

impl PPMImg {
    /// Returns Some(index) if index exists. Otherwise None.
    fn index(&self, x: i32, y: i32) -> Option<usize> {
        let (width, height) = (
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap(),
        );
        if (!self.x_wrap && (x < 0 || x >= width)) || (!self.y_wrap && (y < 0 || y >= height)) {
            return None;
        }

        let x = if x >= width {
            x % width
        } else if x < 0 {
            let r = x % width;
            if r != 0 {
                r + width
            } else {
                r
            }
        } else {
            x
        };
        let y = if y >= height {
            y % height
        } else if y < 0 {
            let r = y % height;
            if r != 0 {
                r + height
            } else {
                r
            }
        } else {
            y
        };

        // invert y based on config
        let y = if self.invert_y {
            self.width as i32 - y - 1
        } else {
            y
        };

        // now we know that x and y are positive, we can cast without worry
        Some((y * self.width as i32 + x).try_into().unwrap())
    }

}

impl Canvas for PPMImg {
    /// plot a point on this PPMImg at (x, y)
    fn plot(&mut self, x: i32, y: i32) -> () {
        if let Some(index) = self.index(x, y) {
            self.data[index] = self.fg_color;
        }
    }
    fn set_fg_color(&mut self, color: RGB) {
        self.fg_color = color;
    }
    fn set_bg_color(&mut self, color: RGB) {
        self.bg_color = color;
    }
    fn get_fg_color(&self) -> RGB {
        self.fg_color
    }
    fn get_bg_color(&self) -> RGB {
        self.bg_color
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
}


// this will stay here during trait refactor, since it has assumption about the internal data structure for Img
impl PPMImg {
    /// Fill an area in img with color calculated by `fill`,
    /// starting at (x, y) and ending when encounters bound color `bound`.
    /// 
    /// Note: This function uses the fact that PPMImg is stored as a `Vec` with an `index` method.
    pub fn bound4_fill_with_fn(
        &mut self,
        x: i32,
        y: i32,
        fill: impl Fn(f64, f64) -> RGB,
        bound: RGB,
    ) {
        let mut points = vec![(x, y)];
        while let Some((x, y)) = points.pop() {
            if let Some(index) = self.index(x, y) {
                let color = self.data[index];
                if color == bound {
                    continue;
                }
                let fcolor = fill(x as f64, y as f64);
                if color == fcolor {
                    continue;
                }
                self.data[index] = fcolor;
                points.push((x + 1, y));
                points.push((x, y + 1));
                points.push((x - 1, y));
                points.push((x, y - 1));
            }
            assert!(points.len() <= (self.width * self.height).try_into().unwrap());
        }
    }
}
