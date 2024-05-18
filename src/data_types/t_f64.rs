use crate::data_types::triadic_type::Ttypes;
use super::triadic::Triadic;

#[derive(Clone, Copy)]
pub struct TF64 {
    value: f64,
    deg: Triadic,
}

impl TF64 {
    pub fn new(val: f64, d: Triadic) -> Self {
        TF64 { value: val, deg: d }
    }

    pub fn set_value(&mut self, val: f64) {
        self.value = val;
    }

    pub fn set_degree(&mut self, d: Triadic) {
        self.deg = d;
    }

    pub fn copy(&self) -> Self {
        TF64 { value: self.value, deg: self.deg }
    }
}

impl Default for TF64 {
    fn default() -> Self {
        Self { value: 0.0, deg: Triadic::default() }
    }
}

impl Ttypes for TF64 {
    type ValType = f64;

    fn get_value(&self) -> Self::ValType {
        self.value
    }

    fn get_degree(&self) -> Triadic {
        self.deg
    }
}
