use crate::canvas::Canvas;
use crate::model::Model;
use crate::project::{f2i_v2, persp, screen};
use crate::transform::Transform;

fn main() {
    let w = 16 * 60;
    let h = 9 * 60;
    let bg = [0xC0, 0xC0, 0xC0];
    let fg = [0x80, 0x00, 0xFF];
    let line_width = 3;
    let mut canvas = Canvas::new(w, h, bg);
    let cube = Model::new(
        vec![
            [0.250, 0.250, 0.250],
            [-0.25, 0.250, 0.250],
            [-0.25, -0.25, 0.250],
            [0.250, -0.25, 0.250],
            [0.250, 0.250, -0.25],
            [-0.25, 0.250, -0.25],
            [-0.25, -0.25, -0.25],
            [0.250, -0.25, -0.25],
        ],
        vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![0, 4],
            vec![1, 5],
            vec![2, 6],
            vec![3, 7],
        ],
    );
    let fps = 60;
    let secs = 1;
    let t = [0., 0., 1.];
    for i in 0..(fps * secs) {
        let rad = (6. * i as f64).to_radians();
        let ts = Transform::new(transform::ry_rad(rad)).translate(t);
        let view = cube.transform(&ts);
        for [p1, p2] in view.lines() {
            let p1 = f2i_v2(screen(persp(p1), w, h));
            let p2 = f2i_v2(screen(persp(p2), w, h));
            canvas.line(p1, p2, line_width, fg);
        }
        canvas.output_ppm(&format!("frame-{:02}", i));
        canvas.fill(bg);
    }
}

pub mod canvas;
pub mod model;
pub mod project;
pub mod transform;
pub mod vector;
