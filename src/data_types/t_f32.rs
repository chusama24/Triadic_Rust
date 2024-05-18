use crate::data_types::triadic_type::Ttypes;
use super::triadic::Triadic;

#[derive(Clone, Copy)]
pub struct TF32 {
    value: f32,
    deg: Triadic,
}

impl TF32 {
    pub fn new(val: f32, d: Triadic) -> Self {
        TF32 { value: val, deg: d }
    }

    pub fn set_value(&mut self, val: f32) {
        self.value = val;
    }

    pub fn set_degree(&mut self, d: Triadic) {
        self.deg = d;
    }

    pub fn copy(&self) -> Self {
        TF32 { value: self.value, deg: self.deg }
    }
}

impl Default for TF32 {
    fn default() -> Self {
        Self { value: 0.0, deg: Triadic::default() }
    }
}

impl Ttypes for TF32 {
    type ValType = f32;

    fn get_value(&self) -> Self::ValType {
        self.value
    }

    fn get_degree(&self) -> Triadic {
        self.deg
    }
}
