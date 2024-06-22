
use crate::data_types::triadic_type::Ttypes;
use super::triadic::Triadic;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TString {
    value: String,
    deg: Triadic,
}

impl TString {
    pub fn new(value: String, deg: Triadic) -> Self {
        TString { value, deg }
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn set_degree(&mut self, deg: Triadic) {
        self.deg = deg;
    }

    pub fn copy(&self) -> Self {
        TString {
            value: self.value.clone(),
            deg: self.deg,
        }
    }
}

impl Default for TString {
    fn default() -> Self {
        Self {
            value: String::new(),
            deg: Triadic::default(),
        }
    }
}

impl Ttypes for TString {
    type ValType = String;

    fn get_value(&self) -> Self::ValType {
        self.value.clone()
    }

    fn get_degree(&self) -> Triadic {
        self.deg
    }
}