use std::env;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io::BufWriter;
use std::io::Write as IoWrite;
use std::ops::AddAssign;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Default, Clone, Copy)]
struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vec4 {
    #[inline]
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    fn sin(self) -> Self {
        let Self { x, y, z, w } = self;
        Self {
            x: x.sin(),
            y: y.sin(),
            z: z.sin(),
            w: w.sin(),
        }
    }

    #[inline]
    fn exp(self) -> Self {
        let Self { x, y, z, w } = self;
        Self {
            x: x.exp(),
            y: y.exp(),
            z: z.exp(),
            w: w.exp(),
        }
    }

    #[inline]
    fn tanh(self) -> Self {
        let Self { x, y, z, w } = self;
        Self {
            x: x.tanh(),
            y: y.tanh(),
            z: z.tanh(),
            w: w.tanh(),
        }
    }
}

impl Add<f64> for Vec4 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        let Self { x, y, z, w } = self;
        Self {
            x: x + rhs,
            y: y + rhs,
            z: z + rhs,
            w: w + rhs,
        }
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Self;

    fn add(self, rhs: Vec4) -> Self::Output {
        let Self { x, y, z, w } = self;
        let Self {
            x: rx,
            y: ry,
            z: rz,
            w: rw,
        } = rhs;
        Self {
            x: x + rx,
            y: y + ry,
            z: z + rz,
            w: w + rw,
        }
    }
}

impl Mul<f64> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let Self { x, y, z, w } = self;
        Self {
            x: x * rhs,
            y: y * rhs,
            z: z * rhs,
            w: w * rhs,
        }
    }
}

impl Mul<Vec4> for f64 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        rhs * self
    }
}

impl AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: Vec4) {
        *self = *self + rhs;
    }
}

impl Sub<Vec4> for f64 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        let Vec4 { x, y, z, w } = rhs;
        Vec4 {
            x: self - x,
            y: self - y,
            z: self - z,
            w: self - w,
        }
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Self;

    fn div(self, rhs: Vec4) -> Self::Output {
        let Self { x, y, z, w } = self;
        let Self {
            x: rx,
            y: ry,
            z: rz,
            w: rw,
        } = rhs;
        Self {
            x: x / rx,
            y: y / ry,
            z: z / rz,
            w: w / rw,
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn yx(self) -> Self {
        let Self { x, y } = self;
        Self { x: y, y: x }
    }

    fn xyyx(self) -> Vec4 {
        let Self { x, y } = self;
        Vec4 {
            x: x,
            y: y,
            z: y,
            w: x,
        }
    }

    fn dot(self, rhs: Self) -> f64 {
        let Self { x, y } = self;
        let Self { x: rx, y: ry } = rhs;
        x * rx + y * ry
    }

    fn cos(self) -> Self {
        let Self { x, y } = self;
        Self {
            x: x.cos(),
            y: y.cos(),
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let Self { x, y } = self;
        Self {
            x: x * rhs,
            y: y * rhs,
        }
    }
}

impl Add<f64> for Vec2 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        let Self { x, y } = self;
        Self {
            x: x + rhs,
            y: y + rhs,
        }
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let Self { x, y } = self;
        Self {
            x: x / rhs,
            y: y / rhs,
        }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs * self
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        let Self { x, y } = self;
        let Self { x: rx, y: ry } = rhs;
        Self {
            x: x - rx,
            y: y - ry,
        }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        let Self { x, y } = self;
        let Self { x: rx, y: ry } = rhs;
        Self {
            x: x + rx,
            y: y + ry,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Vec2) -> Self::Output {
        let Self { x, y } = self;
        let Self { x: rx, y: ry } = rhs;
        Self {
            x: x * rx,
            y: y * ry,
        }
    }
}

impl AddAssign<f64> for Vec2 {
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs;
    }
}

pub fn generate() {
    let scale = [16, 9];
    let fps = 60;
    let cell = fps;
    let w = scale[0] * cell;
    let h = scale[1] * cell;
    let rgb_max = 0xFF;
    let r = Vec2::new(w as f64, h as f64);
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_dir = root + "/output/shader";
    if !fs::exists(&output_dir).unwrap() {
        fs::create_dir_all(&output_dir).unwrap();
    }
    let mut buf = String::new();
    let header = format!("P6\n{} {}\n{}\n", w, h, rgb_max);
    let mut handles = Vec::with_capacity(fps);
    for i in 0..fps {
        FmtWrite::write_fmt(&mut buf, format_args!("{}/frame-{:02}.ppm", output_dir, i)).unwrap();
        let header = header.clone();
        let output_path = buf.clone();
        let handle = std::thread::spawn(move || {
            let file = fs::File::create(&output_path).unwrap();
            let mut file = BufWriter::new(file);
            IoWrite::write_all(&mut file, header.as_bytes()).unwrap();
            let t = i as f64 / fps as f64;
            for y in 0..h {
                for x in 0..w {
                    let fc = Vec2::new(x as f64, y as f64);
                    let p = (fc * 2. - r) / r.y;
                    let l = Vec2::default() + (4. - 4. * (0.7 - p.dot(p)).abs());
                    let mut v = p * l;
                    let mut o = Vec4::default();
                    for i in 1..=8 {
                        let i = Vec2::new(0., i as f64);
                        v += (v.yx() * i.y + i + t).cos() / i.y + 0.7;
                        o += (v.xyyx().sin() + 1.) * (v.x - v.y).abs();
                    }
                    o = (5. * (l.x - 4. - p.y * Vec4::new(-1., 1., 2., 0.)).exp() / o).tanh();
                    let buf = [
                        (o.x * rgb_max as f64) as u8,
                        (o.y * rgb_max as f64) as u8,
                        (o.z * rgb_max as f64) as u8,
                    ];
                    IoWrite::write_all(&mut file, &buf).unwrap();
                }
            }
            println!("Generated {}", output_path);
        });
        handles.push(handle);
        buf.clear();
    }
    for handler in handles {
        handler.join().unwrap();
    }
}

/// # Single-Threaded Implement
pub fn st_impl() {
    let scale = [16, 9];
    let fps = 60;
    let cell = fps;
    let w = scale[0] * cell;
    let h = scale[1] * cell;
    let rgb_max = 0xFF;
    let r = Vec2::new(w as f64, h as f64);
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_dir = root + "/shader";
    if !fs::exists(&output_dir).unwrap() {
        fs::create_dir(&output_dir).unwrap();
    }
    let mut buf = String::new();
    let header = format!("P6\n{} {}\n{}\n", w, h, rgb_max);
    for i in 0..fps {
        FmtWrite::write_fmt(&mut buf, format_args!("{}/frame-{:02}.ppm", output_dir, i)).unwrap();
        let output_path = &buf;
        let file = fs::File::create(output_path).unwrap();
        let mut file = BufWriter::new(file);
        IoWrite::write_all(&mut file, header.as_bytes()).unwrap();
        let t = i as f64 / fps as f64;
        for y in 0..h {
            for x in 0..w {
                let fc = Vec2::new(x as f64, y as f64);
                let p = (fc * 2. - r) / r.y;
                let l = Vec2::default() + (4. - 4. * (0.7 - p.dot(p)).abs());
                let mut o = Vec4::default();
                let mut v = p * l;
                for i in 1..=8 {
                    let i = Vec2::new(0., i as f64);
                    v += (v.yx() * i.y + i + t).cos() / i.y + 0.7;
                    o += (v.xyyx().sin() + 1.) * (v.x - v.y).abs();
                }
                o = (5. * (l.x - 4. - p.y * Vec4::new(-1., 1., 2., 0.)).exp() / o).tanh();
                let buf = [
                    (o.x * rgb_max as f64) as u8,
                    (o.y * rgb_max as f64) as u8,
                    (o.z * rgb_max as f64) as u8,
                ];
                IoWrite::write_all(&mut file, &buf).unwrap();
            }
        }
        println!("Generated {}", output_path);
        buf.clear();
    }
}
