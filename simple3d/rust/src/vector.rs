use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for i64 {
    fn sqrt(self) -> Self {
        self.isqrt()
    }
}

impl Sqrt for i32 {
    fn sqrt(self) -> Self {
        self.isqrt()
    }
}

pub trait Vector<T> {
    fn neg(&self) -> Self;

    fn addv(&self, rhs: &Self) -> Self;
    fn subv(&self, rhs: &Self) -> Self;
    fn mulv(&self, rhs: &Self) -> Self;
    fn divv(&self, rhs: &Self) -> Self;

    fn addf(&self, rhs: T) -> Self;
    fn subf(&self, rhs: T) -> Self;
    fn mulf(&self, rhs: T) -> Self;
    fn divf(&self, rhs: T) -> Self;

    fn dot(&self, rhs: &Self) -> T;
    fn magnitude(&self) -> T;
    fn distance(&self, rhs: &Self) -> T;
    fn normalized(&self) -> Self;
}

impl<const N: usize, T> Vector<T> for [T; N]
where
    T: Default
        + Copy
        + Sqrt
        + Neg<Output = T>
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>,
{
    fn neg(&self) -> Self {
        let mut ret = [T::default(); N];
        for i in 0..N {
            ret[i] = -self[i];
        }
        ret
    }

    fn addv(&self, rhs: &Self) -> Self {
        let mut ret = [T::default(); N];
        for i in 0..N {
            ret[i] = self[i] + rhs[i];
        }
        ret
    }

    fn subv(&self, rhs: &Self) -> Self {
        let mut ret = [T::default(); N];
        for i in 0..N {
            ret[i] = self[i] - rhs[i];
        }
        ret
    }

    fn mulv(&self, rhs: &Self) -> Self {
        let mut ret = [T::default(); N];
        for i in 0..N {
            ret[i] = self[i] * rhs[i];
        }
        ret
    }

    fn divv(&self, rhs: &Self) -> Self {
        let mut ret = [T::default(); N];
        for i in 0..N {
            ret[i] = self[i] / rhs[i];
        }
        ret
    }

    fn addf(&self, rhs: T) -> Self {
        let mut ret = [T::default(); N];
        for i in 0..N {
            ret[i] = self[i] + rhs;
        }
        ret
    }

    fn subf(&self, rhs: T) -> Self {
        let mut ret = [T::default(); N];
        for i in 0..N {
            ret[i] = self[i] - rhs;
        }
        ret
    }

    fn mulf(&self, rhs: T) -> Self {
        let mut ret = [T::default(); N];
        for i in 0..N {
            ret[i] = self[i] * rhs;
        }
        ret
    }

    fn divf(&self, rhs: T) -> Self {
        let mut ret = [T::default(); N];
        for i in 0..N {
            ret[i] = self[i] / rhs;
        }
        ret
    }

    fn dot(&self, rhs: &Self) -> T {
        let mut ret = T::default();
        for i in 0..N {
            ret = self[i] * rhs[i];
        }
        ret
    }

    fn magnitude(&self) -> T {
        let mut sum = T::default();
        for i in 0..N {
            sum = sum + self[i] * self[i];
        }
        sum.sqrt()
    }

    fn distance(&self, rhs: &Self) -> T {
        let mut sum = T::default();
        for i in 0..N {
            sum = sum + (self[i] - rhs[i]) * (self[i] - rhs[i]);
        }
        sum.sqrt()
    }

    fn normalized(&self) -> Self {
        let m = self.magnitude();
        self.divf(m)
    }
}

pub trait Vector3 {
    fn cross(&self, rhs: &Self) -> Self;
}

impl Vector3 for [f64; 3] {
    fn cross(&self, rhs: &Self) -> Self {
        let [x1, y1, z1] = *self;
        let [x2, y2, z2] = *rhs;
        [y1 * z2 - y2 * z1, z1 * x2 - z2 * x1, x1 * y2 - x2 * y1]
    }
}

pub type Vec4f64 = [f64; 4];
pub type Vec3f64 = [f64; 3];
pub type Vec2f64 = [f64; 2];

pub type Vec3i32 = [i32; 3];
pub type Vec2i32 = [i32; 2];

pub type Vec3usize = [usize; 3];
