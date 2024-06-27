use std::collections::HashMap;

use crate::{data_types::{t_f32::TF32, t_string::TString}, t_print::Print};


#[derive(Clone)]
pub enum DataType{
    Float(TF32),
    String(TString),
    Empty
}


#[derive(Clone)]
pub struct Row {
    data: HashMap<TString, DataType>
}

impl Row {
    pub fn new() -> Row {
        Row{data: HashMap::<TString, DataType>::new()}
    }

    pub fn get_row(&self) -> HashMap<TString, DataType> {
        self.data.clone()
    }

    pub fn insert(&mut self, name: &TString, val: &DataType) {
        self.data.insert(name.clone(), val.clone());
    }

    pub fn get_value(&self, name: &TString) -> Option<DataType> {
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
                DataType::Float(tf32) => tf32.t_print(),
                DataType::String(tstring) => tstring.t_print(),
                _ => print!("None") 
            }
        }
    }
}