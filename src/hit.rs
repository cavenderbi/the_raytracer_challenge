use crate::shapes::Shapes;

#[derive(Debug, PartialEq)]
pub struct Hit<'a> {
    shape: &'a Shapes,
    xs: Vec<f64>,
}

impl<'a> Hit<'a> {
    pub fn new(shape: &'a Shapes, xs: Vec<f64>) -> Self {
        Self { shape, xs }
    }
    pub fn xs(&self) -> &[f64] {
        self.xs.as_ref()
    }
    pub fn shape(&self) -> &Shapes {
        self.shape
    }

}
