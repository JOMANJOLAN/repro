use crate::coordsys::CoordSys;
use crate::transform as tsf;
use crate::transform::Transform;
use crate::vector::{V3f64, Vector};

pub struct Object {
    cs: CoordSys,
    vs: Vec<V3f64>,
    es: Vec<Vec<usize>>,
}

impl Object {
    pub fn new(vs: Vec<V3f64>, es: Vec<Vec<usize>>) -> Self {
        Self {
            cs: CoordSys::global(),
            vs,
            es,
        }
    }

    pub fn coordsys(&self) -> &CoordSys {
        &self.cs
    }

    pub fn vertices(&self) -> &[V3f64] {
        &self.vs
    }

    pub fn edges(&self) -> &[Vec<usize>] {
        &self.es
    }

    pub fn translate_local_tsf(&mut self, v: V3f64) -> Transform {
        let [x, y, z] = v;
        Transform::new(tsf::translate(self.cs.x().muls(x)))
            .translate(self.cs.y().muls(y))
            .translate(self.cs.z().muls(z))
    }

    pub fn translate_local(&mut self, v: V3f64) {
        let ts = self.translate_local_tsf(v);
        self.transform(&ts);
    }

    pub fn rx_local_tsf(&mut self, rad: f64) -> Transform {
        Transform::new(tsf::rotate(self.cs.origin(), self.cs.x(), rad))
    }

    pub fn ry_local_tsf(&mut self, rad: f64) -> Transform {
        Transform::new(tsf::rotate(self.cs.origin(), self.cs.y(), rad))
    }

    pub fn rz_local_tsf(&mut self, rad: f64) -> Transform {
        Transform::new(tsf::rotate(self.cs.origin(), self.cs.z(), rad))
    }

    pub fn rx_local(&mut self, rad: f64) {
        let ts = self.rx_local_tsf(rad);
        self.transform(&ts);
    }

    pub fn ry_local(&mut self, rad: f64) {
        let ts = self.ry_local_tsf(rad);
        self.transform(&ts);
    }

    pub fn rz_local(&mut self, rad: f64) {
        let ts = self.rz_local_tsf(rad);
        self.transform(&ts);
    }

    pub fn transform(&mut self, ts: &Transform) {
        ts.transform(self.cs.as_mat_mut());
        ts.transform(&mut self.vs);
    }

    pub fn lines(&self) -> impl Iterator<Item = [V3f64; 2]> {
        self.es
            .iter()
            .map(|e| (0..e.len()).map(|i| (e[i], e[(i + 1) % e.len()])))
            .flatten()
            .map(|(i, j)| [self.vs[i], self.vs[j]])
    }
}
