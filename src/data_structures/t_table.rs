use crate::{data_types::t_string::TString, t_print::Print};

use super::t_row::{DataType, Row};


pub struct Ttable {
    header: Vec<TString>,
    data: Vec<Row>
}

impl Ttable {
    pub fn new() -> Ttable{
        Ttable{data: Vec::<Row>::new(), header: Vec::<TString>::new()}
    }

    pub fn insert_row(&mut self, row: &Row) {
        self.data.push(row.clone());
    }

    pub fn get_row(&self, index: i32) -> Row {
        let i = index as usize;
        self.data[i].clone()
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn get_column(&self, name: &TString) -> Vec<DataType> {
        let mut temp = Vec::<DataType>::new();
        for r in self.data.iter(){
            let val = r.get_value(&name);
            if let Some(v) = val {
                temp.push(v);
            }
        }
        temp
    }

    pub fn get_headers(&self) -> Vec<TString> {
        self.header.clone()
    }

    pub fn insert_header(&mut self, name: &TString) {
        self.header.push(name.clone());
    }

    pub fn insert_header_list(&mut self, names: &Vec<TString>) {
        self.header = names.clone();
    }

    pub fn print_table_row_wise(&self) {
        let mut i = 0;
        for r in self.data.iter(){
            println!("Row {}: ", i);
            r.print_row();
            i += 1;
        }
    }

    pub fn print_table_col_wise(&self) {
        for name in self.header.iter() {
            println!("Column Name: ");
            name.t_print();
            let temp = self.get_column(&name);

            for val in temp {
                match val {
                    DataType::Float(v) => v.t_print(),
                    DataType::String(v) => v.t_print(),
                    _ => println!("None")
                }
            }

        }
    }
}