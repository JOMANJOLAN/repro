use crate::vector::{Vec2f64, Vec3f64, Vector};

pub fn persp(p: Vec3f64) -> Vec2f64 {
    let [x, y, mut z] = p;
    if z.abs() < 1e-6 {
        // 0.001
        z = z.signum() * 1e-6;
    }
    [x, y].divs(z)
}

pub fn screen(p: Vec2f64, w: usize, h: usize) -> Vec2f64 {
    assert!(w != 0);
    assert!(h != 0);
    let w = w as f64;
    let h = h as f64;
    let sx = if w > h { h / w } else { 1. };
    let sy = if h > w { w / h } else { 1. };
    let [x, y] = p.mulv(&[sx, sy]);
    [1. + x, 1. - y].divs(2.).mulv(&[w, h])
}
