use crate::vector::{Vec2f64, Vec2i32, Vec3f64, Vector};

pub fn persp(p: Vec3f64) -> Vec2f64 {
    let [x, y, z] = p;
    [x, y].divf(z)
}

pub fn screen(p: Vec2f64, w: usize, h: usize) -> Vec2f64 {
    assert!(w != 0);
    assert!(h != 0);
    let w = w as f64;
    let h = h as f64;
    let sx = if w > h { h / w } else { 1. };
    let sy = if h > w { w / h } else { 1. };
    let [x, y] = p.mulv(&[sx, sy]);
    [1. + x, 1. - y].divf(2.).mulv(&[w, h])
}

pub fn f2i_v2(p: Vec2f64) -> Vec2i32 {
    let [x, y] = p;
    [x as i32, y as i32]
}

// use crate::vector::{Vec3f64, Vector};

// pub struct CoordSys {
//     o: Vec3f64,
//     x: Vec3f64,
//     y: Vec3f64,
//     z: Vec3f64,
// }

// impl Default for CoordSys {
//     fn default() -> Self {
//         Self {
//             o: [0.; 3],
//             x: [1., 0., 0.],
//             y: [0., 1., 0.],
//             z: [0., 0., 1.],
//         }
//     }
// }

// impl CoordSys {
//     #[inline]
//     pub fn origin(&self) -> Vec3f64 {
//         self.o
//     }

//     #[inline]
//     pub fn right(&self) -> Vec3f64 {
//         self.x
//     }

//     #[inline]
//     pub fn left(&self) -> Vec3f64 {
//         self.front().neg()
//     }

//     #[inline]
//     pub fn up(&self) -> Vec3f64 {
//         self.y
//     }

//     #[inline]
//     pub fn down(&self) -> Vec3f64 {
//         self.up().neg()
//     }

//     #[inline]
//     pub fn front(&self) -> Vec3f64 {
//         self.z
//     }

//     #[inline]
//     pub fn back(&self) -> Vec3f64 {
//         self.front().neg()
//     }

//     // pub fn to(&self, dst: &Self) -> Transform {
//     //     let tf1 = {
//     //         let o = self.o;
//     //         let [x, y, z] = self.z;
//     //         let cos_a = z / [y, z].magnitude();
//     //         let sin_a = y / [y, z].magnitude();
//     //         let cos_b = [y, z].magnitude() / [x, y, z].magnitude();
//     //         let sin_b = -x / [x, y, z].magnitude();
//     //         let tf = Transform::new(translate(o.neg()))
//     //             .rx_cossin(cos_a, sin_a)
//     //             .ry_cossin(cos_b, sin_b);
//     //         let [x] = tf.transform_con(&[self.x]);
//     //         let [x, y, ..] = x;
//     //         let cos_c = x / [x, y].magnitude();
//     //         let sin_c = -y / [x, y].magnitude();
//     //         tf.rz_cossin(cos_c, sin_c)
//     //     };
//     //     let tf2 = {
//     //         let o = dst.o;
//     //         let [x, y, z] = dst.z;
//     //         let cos_a = z / [y, z].magnitude();
//     //         let sin_a = y / [y, z].magnitude();
//     //         let cos_b = [y, z].magnitude() / [x, y, z].magnitude();
//     //         let sin_b = -x / [x, y, z].magnitude();
//     //         let tf = Transform::new(translate(o.neg()))
//     //             .rx_cossin(cos_a, sin_a)
//     //             .ry_cossin(cos_b, sin_b);
//     //         let [x] = tf.transform_con(&[dst.x]);
//     //         let [x, y, ..] = x;
//     //         let cos_c = x / [x, y].magnitude();
//     //         let sin_c = y / [x, y].magnitude();
//     //         Transform::new(rz_cossin(cos_c, sin_c))
//     //             .ry_cossin(cos_b, -sin_b)
//     //             .rx_cossin(cos_a, -sin_a)
//     //             .translate(o)
//     //     };
//     //     tf2 * tf1
//     // }
// }
