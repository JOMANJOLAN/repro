use crate::vector::{Vec3f64, Vector};

#[derive(Clone, Copy)]
pub struct CoordSys {
    o: Vec3f64,
    x: Vec3f64,
    y: Vec3f64,
    z: Vec3f64,
}

impl CoordSys {
    pub const fn global() -> Self {
        Self {
            o: [0.; 3],
            x: [1., 0., 0.],
            y: [0., 1., 0.],
            z: [0., 0., 1.],
        }
    }

    #[inline]
    pub fn origin(&self) -> Vec3f64 {
        self.o
    }

    #[inline]
    pub fn x(&self) -> Vec3f64 {
        self.x.subv(&self.o)
    }

    #[inline]
    pub fn right(&self) -> Vec3f64 {
        self.x()
    }

    #[inline]
    pub fn left(&self) -> Vec3f64 {
        self.x().neg()
    }

    #[inline]
    pub fn y(&self) -> Vec3f64 {
        self.y.subv(&self.o)
    }

    #[inline]
    pub fn up(&self) -> Vec3f64 {
        self.y()
    }

    #[inline]
    pub fn down(&self) -> Vec3f64 {
        self.y().neg()
    }

    #[inline]
    pub fn z(&self) -> Vec3f64 {
        self.z.subv(&self.o)
    }

    #[inline]
    pub fn front(&self) -> Vec3f64 {
        self.z()
    }

    #[inline]
    pub fn back(&self) -> Vec3f64 {
        self.z().neg()
    }

    #[inline]
    pub fn from_mat(mat: &[Vec3f64; 4]) -> &Self {
        let ptr = mat.as_ptr().cast::<CoordSys>();
        unsafe { &*ptr }
    }

    #[inline]
    pub fn from_mat_mut(mat: &mut [Vec3f64; 4]) -> &mut Self {
        let ptr = mat.as_mut_ptr().cast::<CoordSys>();
        unsafe { &mut *ptr }
    }

    pub fn as_mat(&self) -> &[Vec3f64; 4] {
        let ptr = (self as *const CoordSys).cast::<[Vec3f64; 4]>();
        unsafe { &*ptr }
    }

    pub fn as_mat_mut(&mut self) -> &mut [Vec3f64; 4] {
        let ptr = (self as *mut CoordSys).cast::<[Vec3f64; 4]>();
        unsafe { &mut *ptr }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_align_test() {
        let s1 = size_of::<CoordSys>();
        let a1 = align_of::<CoordSys>();
        let s2 = size_of::<[Vec3f64; 4]>();
        let a2 = align_of::<[Vec3f64; 4]>();
        println!("{} {}", s1, a1);
        println!("{} {}", s2, a2);
        assert_eq!(s1, s2);
        assert_eq!(a1, a2);
    }
}
