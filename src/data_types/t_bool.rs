use crate::data_types::triadic_type::Ttypes;
use super::triadic::Triadic;

#[derive(Clone, Copy)]
pub struct TBool {
    value: bool,
    deg: Triadic,
}

impl TBool {
    pub fn new(val: bool, d: Triadic) -> Self {
        TBool { value: val, deg: d }
    }

    pub fn set_value(&mut self, val: bool) {
        self.value = val;
    }

    pub fn set_degree(&mut self, d: Triadic) {
        self.deg = d;
    }

    pub fn copy(&self) -> Self {
        TBool { value: self.value, deg: self.deg }
    }
}

impl Default for TBool {
    fn default() -> Self {
        Self {
            value: false, // Default boolean value
            deg: Triadic::default(),
        }
    }
}

impl Ttypes for TBool {
    type ValType = bool;

    fn get_value(&self) -> Self::ValType {
        self.value
    }

    fn get_degree(&self) -> Triadic {
        self.deg
    }
}
