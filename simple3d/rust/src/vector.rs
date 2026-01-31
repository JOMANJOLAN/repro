use std::array;
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

pub trait Convert<T> {
    fn cvt(&self) -> T;
}

macro_rules! impl_convert_as {
    ($t:ty, $u:ty) => {
        impl Convert<$u> for $t {
            fn cvt(&self) -> $u {
                *self as $u
            }
        }
    };
}

impl_convert_as!(f64, f64);
impl_convert_as!(f64, f32);
impl_convert_as!(f64, i64);
impl_convert_as!(f64, i32);

impl_convert_as!(f32, f64);
impl_convert_as!(f32, f32);
impl_convert_as!(f32, i64);
impl_convert_as!(f32, i32);

impl_convert_as!(i64, f64);
impl_convert_as!(i64, f32);
impl_convert_as!(i64, i64);
impl_convert_as!(i64, i32);

impl_convert_as!(i32, f64);
impl_convert_as!(i32, f32);
impl_convert_as!(i32, i64);
impl_convert_as!(i32, i32);

impl<const N: usize, T: Convert<U>, U> Convert<[U; N]> for [T; N] {
    fn cvt(&self) -> [U; N] {
        array::from_fn(|i| self[i].cvt())
    }
}

pub trait Vector<T> {
    fn neg(&self) -> Self;

    fn addv(&self, rhs: &Self) -> Self;
    fn subv(&self, rhs: &Self) -> Self;
    fn mulv(&self, rhs: &Self) -> Self;
    fn divv(&self, rhs: &Self) -> Self;

    fn adds(&self, rhs: T) -> Self;
    fn subs(&self, rhs: T) -> Self;
    fn muls(&self, rhs: T) -> Self;
    fn divs(&self, rhs: T) -> Self;

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
        array::from_fn(|i| -self[i])
    }

    fn addv(&self, rhs: &Self) -> Self {
        array::from_fn(|i| self[i] + rhs[i])
    }

    fn subv(&self, rhs: &Self) -> Self {
        array::from_fn(|i| self[i] - rhs[i])
    }

    fn mulv(&self, rhs: &Self) -> Self {
        array::from_fn(|i| self[i] * rhs[i])
    }

    fn divv(&self, rhs: &Self) -> Self {
        array::from_fn(|i| self[i] / rhs[i])
    }

    fn adds(&self, rhs: T) -> Self {
        array::from_fn(|i| self[i] + rhs)
    }

    fn subs(&self, rhs: T) -> Self {
        array::from_fn(|i| self[i] - rhs)
    }

    fn muls(&self, rhs: T) -> Self {
        array::from_fn(|i| self[i] * rhs)
    }

    fn divs(&self, rhs: T) -> Self {
        array::from_fn(|i| self[i] / rhs)
    }

    fn dot(&self, rhs: &Self) -> T {
        let mut sum = T::default();
        for i in 0..N {
            sum = sum + self[i] * rhs[i];
        }
        sum
    }

    fn magnitude(&self) -> T {
        let mut sum = T::default();
        for i in 0..N {
            let n = self[i];
            sum = sum + n * n;
        }
        sum.sqrt()
    }

    fn distance(&self, rhs: &Self) -> T {
        let mut sum = T::default();
        for i in 0..N {
            let n = (self[i] - rhs[i]) * (self[i] - rhs[i]);
            sum = sum + n * n;
        }
        sum.sqrt()
    }

    fn normalized(&self) -> Self {
        let m = self.magnitude();
        self.divs(m)
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

pub type V4f64 = [f64; 4];
pub type V3f64 = [f64; 3];
pub type V2f64 = [f64; 2];

pub type V3i32 = [i32; 3];
pub type V2i32 = [i32; 2];

pub type V3usize = [usize; 3];
