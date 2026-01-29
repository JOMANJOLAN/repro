use crate::transform::Transform;
use crate::vector::Vec3f64;
use std::borrow::Cow;

pub struct Model {
    vs: Vec<Vec3f64>,
    es: Vec<Vec<usize>>,
}

impl Model {
    pub fn new(vs: Vec<Vec3f64>, es: Vec<Vec<usize>>) -> Self {
        Self { vs, es }
    }

    pub fn vertices(&self) -> &[Vec3f64] {
        &self.vs
    }

    pub fn edges(&self) -> &[Vec<usize>] {
        &self.es
    }

    pub fn transform(&self, ts: &Transform) -> View<'_> {
        let vs = ts.transform_vec(&self.vs);
        View {
            vs: Cow::Owned(vs),
            es: &self.es,
        }
    }
}

pub struct View<'a> {
    vs: Cow<'a, Vec<Vec3f64>>,
    es: &'a [Vec<usize>],
}

impl View<'_> {
    pub fn vertices(&self) -> &[Vec3f64] {
        &self.vs
    }

    pub fn edges(&self) -> &[Vec<usize>] {
        &self.es
    }

    pub fn lines(&self) -> impl Iterator<Item = [Vec3f64; 2]> {
        self.es
            .iter()
            .map(|e| (0..e.len()).map(|i| (e[i], e[(i + 1) % e.len()])))
            .flatten()
            .map(|(i, j)| [self.vs[i], self.vs[j]])
    }
}
