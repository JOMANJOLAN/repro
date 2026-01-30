use crate::vector::{Vec3f64, Vec4f64, Vector};

/// # Transform Matrix Multiplication
fn mul(m1: &[Vec4f64; 4], m2: &[Vec4f64; 4]) -> [Vec4f64; 4] {
    let mut n = [[0.; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                n[i][j] += m1[i][k] * m2[k][j];
            }
        }
    }
    n
}

pub fn e() -> [Vec4f64; 4] {
    [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]
}

pub fn translate([x, y, z]: Vec3f64) -> [Vec4f64; 4] {
    [
        [1., 0., 0., x],
        [0., 1., 0., y],
        [0., 0., 1., z],
        [0., 0., 0., 1.],
    ]
}

pub fn zoom([x, y, z]: Vec3f64) -> [Vec4f64; 4] {
    [
        [x, 0., 0., 0.],
        [0., y, 0., 0.],
        [0., 0., z, 0.],
        [0., 0., 0., 1.],
    ]
}

pub fn rx_cossin(cos: f64, sin: f64) -> [Vec4f64; 4] {
    [
        [1., 0., 0., 0.],
        [0., cos, -sin, 0.],
        [0., sin, cos, 0.],
        [0., 0., 0., 1.],
    ]
}

pub fn ry_cossin(cos: f64, sin: f64) -> [Vec4f64; 4] {
    [
        [cos, 0., sin, 0.],
        [0., 1., 0., 0.],
        [-sin, 0., cos, 0.],
        [0., 0., 0., 1.],
    ]
}

pub fn rz_cossin(cos: f64, sin: f64) -> [Vec4f64; 4] {
    [
        [cos, -sin, 0., 0.],
        [sin, cos, 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]
}

pub fn rx_rad(rad: f64) -> [Vec4f64; 4] {
    let cos = rad.cos();
    let sin = rad.sin();
    rx_cossin(cos, sin)
}

pub fn ry_rad(rad: f64) -> [Vec4f64; 4] {
    let cos = rad.cos();
    let sin = rad.sin();
    ry_cossin(cos, sin)
}

pub fn rz_rad(rad: f64) -> [Vec4f64; 4] {
    let cos = rad.cos();
    let sin = rad.sin();
    rz_cossin(cos, sin)
}

/// # Params
/// `o` means origin, `d` means direction
pub fn rotate(o: Vec3f64, d: Vec3f64, rad: f64) -> [Vec4f64; 4] {
    let [x, y, z] = d;
    let cos_a = z / [y, z].magnitude();
    let sin_a = y / [y, z].magnitude();
    let cos_b = [y, z].magnitude() / [x, y, z].magnitude();
    let sin_b = -x / [x, y, z].magnitude();
    let t = translate(o.neg());
    let ra = rx_cossin(cos_a, sin_a);
    let rb = ry_cossin(cos_b, sin_b);
    let r = rz_rad(rad);
    let rb_i = ry_cossin(cos_b, -sin_b);
    let ra_i = rx_cossin(cos_a, -sin_a);
    let t_i = translate(o);
    let mut n = t;
    n = mul(&ra, &n);
    n = mul(&rb, &n);
    n = mul(&r, &n);
    n = mul(&rb_i, &n);
    n = mul(&ra_i, &n);
    n = mul(&t_i, &n);
    n
}

#[derive(Clone, Copy)]
pub struct Transform {
    t: [Vec4f64; 4],
}

impl Transform {
    pub fn new(t: [Vec4f64; 4]) -> Self {
        Self { t }
    }

    pub fn e() -> Self {
        Self { t: e() }
    }

    pub fn zoom(self, v: Vec3f64) -> Self {
        Self {
            t: mul(&zoom(v), &self.t),
        }
    }

    pub fn translate(self, v: Vec3f64) -> Self {
        Self {
            t: mul(&translate(v), &self.t),
        }
    }

    pub fn rx_cossin(self, cos: f64, sin: f64) -> Self {
        Self {
            t: mul(&rx_cossin(cos, sin), &self.t),
        }
    }

    pub fn ry_cossin(self, cos: f64, sin: f64) -> Self {
        Self {
            t: mul(&ry_cossin(cos, sin), &self.t),
        }
    }

    pub fn rz_cossin(self, cos: f64, sin: f64) -> Self {
        Self {
            t: mul(&rz_cossin(cos, sin), &self.t),
        }
    }

    pub fn rx_rad(self, rad: f64) -> Self {
        Self {
            t: mul(&rx_rad(rad), &self.t),
        }
    }

    pub fn ry_rad(self, rad: f64) -> Self {
        Self {
            t: mul(&ry_rad(rad), &self.t),
        }
    }

    pub fn rz_rad(self, rad: f64) -> Self {
        Self {
            t: mul(&rz_rad(rad), &self.t),
        }
    }

    pub fn rotate(self, o: Vec3f64, d: Vec3f64, rad: f64) -> Self {
        Self {
            t: mul(&rotate(o, d, rad), &self.t),
        }
    }

    pub fn transform(self, vs: &mut [Vec3f64]) {
        let t = self.t;
        let mut buf = [0.; 3];
        for i in 0..vs.len() {
            for j in 0..3 {
                for k in 0..3 {
                    buf[j] += t[j][k] * vs[i][k];
                }
                buf[j] += t[j][3];
            }
            vs[i] = buf;
            buf.fill(0.);
        }
    }
}

impl std::ops::Mul<Self> for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            t: mul(&self.t, &rhs.t),
        }
    }
}
