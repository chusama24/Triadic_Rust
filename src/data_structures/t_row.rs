use std::collections::HashMap;

use crate::{data_types::{t_f32::TF32, t_string::TString, triadic::Triadic}, operators::triadic_op::{TriadicLogicalOp, TriadicStringOp}, t_print::Print};


#[derive(Clone)]
pub enum RDataType{
    Float(TF32),
    String(TString),
    Empty
}


#[derive(Clone)]
pub struct TRow {
    data: HashMap<TString, RDataType>,
    degree: Triadic
}

impl TRow {
    pub fn new() -> TRow {
        TRow{data: HashMap::<TString, RDataType>::new(), degree: Triadic::default()}
    }

    pub fn get_row(&self) -> HashMap<TString, RDataType> {
        self.data.clone()
    }

    pub fn get_degree(&self) -> Triadic {
        self.degree
    }

    pub fn set_degree(&mut self, d: &Triadic) {
        self.degree = *d;
    }

    pub fn insert(&mut self, name: &TString, val: &RDataType) {
        self.data.insert(name.clone(), val.clone());
        match val {
            RDataType::Float(tf32) => {
                let t = TF32::new(0.0, self.get_degree());
                self.set_degree(&t.tand_1(*tf32));
            },
            RDataType::String(tstring) => {
                let t = TString::new("".to_string(), self.get_degree());
                self.set_degree(&t.tand_1(tstring));
            },
            _ => {
                self.set_degree(&Triadic::default());                
            }
        }
    }

    pub fn get_value(&self, name: &TString) -> Option<RDataType> {
        if let Some(val) = self.data.get(&name) {
            return Some(val.clone());
        }
        return None;
    }

    pub fn print_row(&self) {
        for (k,v) in self.data.iter() {
            print!("Name: ");
            k.t_print();
            print!("Value: ");
            match v {
                RDataType::Float(tf32) => tf32.t_print(),
                RDataType::String(tstring) => tstring.t_print(),
                _ => print!("None") 
            }
        }
        print!("Degree of TRow: ");
        self.degree.t_print();
    }
}