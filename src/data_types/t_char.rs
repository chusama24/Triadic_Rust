use crate::data_types::triadic_type::Ttypes;
use super::triadic::Triadic;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TChar {
    value: char,
    deg: Triadic,
}

impl TChar {
    pub fn new(val: char, d: Triadic) -> Self {
        TChar { value: val, deg: d }
    }

    pub fn set_value(&mut self, val: char) {
        self.value = val;
    }

    pub fn set_degree(&mut self, d: Triadic) {
        self.deg = d;
    }

    pub fn copy(&self) -> Self {
        TChar { value: self.value, deg: self.deg }
    }
}

impl Default for TChar {
    fn default() -> Self {
        Self {
            value: '\0', // Default character
            deg: Triadic::default(),
        }
    }
}

impl Ttypes for TChar {
    type ValType = char;

    fn get_value(&self) -> Self::ValType {
        self.value
    }

    fn get_degree(&self) -> Triadic {
        self.deg
    }
}
