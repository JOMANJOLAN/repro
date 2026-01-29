use crate::vector::{Vec2i32, Vector};

#[derive(Clone, Copy)]
pub struct Point {
    pub p: Vec2i32,
    pub r: usize,
}

impl Point {
    pub fn new(p: Vec2i32, r: usize) -> Self {
        Self { p, r }
    }

    pub fn iter(&self) -> impl Iterator<Item = Vec2i32> {
        let Self { p, r } = *self;
        let r = r as i32;
        (-r..=r)
            .map(move |i| {
                (-r..=r)
                    .filter(move |&j| [i, j].magnitude() <= r)
                    .map(move |j| [i, j].addv(&p))
            })
            .flatten()
    }
}

#[derive(Clone, Copy)]
pub struct Line {
    pub p1: Vec2i32,
    pub p2: Vec2i32,
}

impl Line {
    pub fn new(p1: Vec2i32, p2: Vec2i32) -> Self {
        Self { p1, p2 }
    }

    pub fn iter(&self) -> impl Iterator<Item = Vec2i32> {
        let Self { p1, p2 } = *self;
        LineIter::new(p1, p2)
    }
}

pub struct LineIter {
    p1: Vec2i32,
    p2: Vec2i32,
    p: Option<Vec2i32>,
    dx: i32,
    dy: i32,
    sx: i32,
    sy: i32,
    e: i32,
}

impl LineIter {
    pub fn new(p1: Vec2i32, p2: Vec2i32) -> Self {
        let dx = (p2[0] - p1[0]).abs();
        let dy = -(p2[1] - p1[1]).abs();
        let sx = if p2[0] > p1[0] { 1 } else { -1 };
        let sy = if p2[1] > p1[1] { 1 } else { -1 };
        Self {
            p1,
            p2,
            p: Some(p1),
            dx,
            dy,
            sx,
            sy,
            e: dx + dy,
        }
    }

    pub fn start(&self) -> Vec2i32 {
        self.p1
    }

    pub fn end(&self) -> Vec2i32 {
        self.p2
    }
}

impl Iterator for LineIter {
    type Item = Vec2i32;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            p1: _,
            p2,
            p,
            dx,
            dy,
            sx,
            sy,
            mut e,
        } = *self;
        let p = p?;
        if p == p2 {
            self.p = None;
        } else {
            let [mut x, mut y] = p;
            let e2 = 2 * e;
            if e2 >= dy {
                e += dy;
                x += sx;
            }
            if e2 <= dx {
                e += dx;
                y += sy;
            }
            self.e = e;
            self.p = Some([x, y]);
        }
        Some(p)
    }
}

pub struct Canvas {
    w: usize,
    h: usize,
    vec: Vec<[u8; 3]>,
}

impl Canvas {
    pub fn new(w: usize, h: usize, fill: [u8; 3]) -> Self {
        Self {
            w,
            h,
            vec: vec![fill; w * h],
        }
    }

    pub fn fill(&mut self, color: [u8; 3]) {
        self.vec.fill(color);
    }

    pub fn dot(&mut self, p: Vec2i32, radius: usize, rgb: [u8; 3]) {
        let Self { w, h, .. } = *self;
        for p in Point::new(p, radius).iter() {
            let [x, y] = p;
            if 0 <= x && x < w as i32 && 0 <= y && y < h as i32 {
                unsafe {
                    *self.pixel_unchecked_mut(x as usize, y as usize) = rgb;
                }
            }
        }
    }

    pub fn line(&mut self, p1: Vec2i32, p2: Vec2i32, width: usize, rgb: [u8; 3]) {
        let Self { w, h, .. } = *self;
        let ps = Point::new([0; 2], width / 2).iter().collect::<Vec<_>>();
        for p in LineIter::new(p1, p2) {
            for q in ps.iter() {
                let p = p.addv(q);
                let [x, y] = p;
                if 0 <= x && x < w as i32 && 0 <= y && y < h as i32 {
                    unsafe {
                        *self.pixel_unchecked_mut(x as usize, y as usize) = rgb;
                    }
                }
            }
        }
    }

    pub unsafe fn pixel_unchecked(&self, x: usize, y: usize) -> &[u8; 3] {
        unsafe { self.vec.get_unchecked(y * self.w + x) }
    }

    pub unsafe fn pixel_unchecked_mut(&mut self, x: usize, y: usize) -> &mut [u8; 3] {
        unsafe { self.vec.get_unchecked_mut(y * self.w + x) }
    }

    pub fn pixel(&self, x: usize, y: usize) -> Option<&[u8; 3]> {
        if x < self.w && y < self.h {
            Some(&self.vec[y * self.w + x])
        } else {
            None
        }
    }

    pub fn pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut [u8; 3]> {
        if x < self.w && y < self.h {
            Some(&mut self.vec[y * self.w + x])
        } else {
            None
        }
    }

    pub fn output_ppm(&self, file_name: &str) {
        use std::env;
        use std::fmt::Write as FmtWrite;
        use std::fs;
        use std::io::BufWriter;
        use std::io::Write as IoWrite;
        let Self { w, h, .. } = *self;
        let rgb_max = 0xFF;
        let root = env::var("CARGO_MANIFEST_DIR").unwrap();
        let output_dir = root + "/output";
        if !fs::exists(&output_dir).unwrap() {
            fs::create_dir_all(&output_dir).unwrap();
        }
        let mut buf = String::new();
        let header = format!("P6\n{} {}\n{}\n", w, h, rgb_max);
        FmtWrite::write_fmt(&mut buf, format_args!("{}/{}.ppm", output_dir, file_name)).unwrap();
        let output_path = &buf;
        let file = fs::File::create(output_path).unwrap();
        let mut file = BufWriter::new(file);
        IoWrite::write_all(&mut file, header.as_bytes()).unwrap();
        let ptr = self.vec.as_ptr().cast::<u8>();
        let slice = unsafe { std::slice::from_raw_parts(ptr, w * h * 3) };
        IoWrite::write_all(&mut file, slice).unwrap();
        println!("Generated {}", output_path);
        buf.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_test() {
        let w = 16 * 60;
        let h = 9 * 60;
        let white = [0xFF; 3];
        let red = [0xFF, 0x00, 0x00];
        let mut canvas = Canvas::new(w, h, white);
        let p1 = [18_i32, 10];
        let p2 = [879_i32, 479];
        canvas.line(p1, p2, 7, red);
        canvas.output_ppm("line");
    }
}
