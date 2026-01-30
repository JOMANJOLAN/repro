use crate::canvas::Canvas;
use crate::object::Object;
use crate::project::{persp, screen};
use crate::vector::Convert;

fn main() {
    let w = 16 * 60;
    let h = 9 * 60;
    let bg = [0xC0, 0xC0, 0xC0];
    let fg = [0x80, 0x00, 0xFF];
    let line_width = 3;
    let mut canvas = Canvas::new(w, h, bg);
    let mut pyramid = Object::new(
        vec![
            [0., 2. * 2_f64.sqrt() * 2. / 3., 0.],              // 0
            [0., -2. * 2_f64.sqrt() * 1. / 3., 2.],             // 1
            [3_f64.sqrt(), -2. * 2_f64.sqrt() * 1. / 3., -1.],  // 2
            [-3_f64.sqrt(), -2. * 2_f64.sqrt() * 1. / 3., -1.], // 3
        ],
        vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2, 3]],
    );
    let fps = 60;
    let secs = 1;
    let rad = 6_f64.to_radians();
    pyramid.translate_local([0., 0., 5.]);
    let rtsf = pyramid.ry_local_tsf(rad);
    for i in 0..(fps * secs) {
        for [p1, p2] in pyramid.lines() {
            let p1 = screen(persp(p1), w, h).cvt();
            let p2 = screen(persp(p2), w, h).cvt();
            canvas.line(p1, p2, line_width, fg);
        }
        pyramid.transform(&rtsf);
        canvas.output_ppm(&format!("frame-{:02}", i));
        canvas.fill(bg);
    }
}

pub mod canvas;
pub mod coordsys;
pub mod object;
pub mod project;
pub mod transform;
pub mod vector;
