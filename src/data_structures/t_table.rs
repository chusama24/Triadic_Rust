use crate::{data_types::{t_string::TString, triadic::Triadic, triadic_type::Ttypes}, operators::triadic_op::TriadicStringOp, t_enum::ConvertToDegree, t_print::Print};

use super::{t_array::TArray, t_row::{RDataType, TRow}};


pub struct Ttable {
    header: TArray<TString>,
    data: Vec<TRow>
}

impl Ttable {
    pub fn new() -> Ttable{
        Ttable{data: Vec::<TRow>::new(), header: TArray::<TString>::default()}
    }

    pub fn insert_row(&mut self, row: &TRow) {
        self.data.push(row.clone());
    }

    pub fn get_row(&self, index: i32) -> TRow {
        let i = index as usize;
        self.data[i].clone()
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn get_column_by_name(&self, name: &TString) -> Vec<RDataType> {
        let mut temp = Vec::<RDataType>::new();
        for r in self.data.iter(){
            let val = r.get_value(&name);
            if let Some(v) = val {
                temp.push(v);
            }
        }
        temp
    }

    pub fn get_column_by_index(&self, index: i32) -> Vec<RDataType> {
        let mut temp = Vec::<RDataType>::new();

        let attr = self.get_header(index);
        for r in self.data.iter(){
            let val = r.get_value(&attr);
            if let Some(v) = val {
                temp.push(v);
            }
        }
        temp

    }

    pub fn get_headers_list(&self) -> TArray<TString> {
        self.header.clone()
    }

    pub fn get_header(&self, index: i32) -> TString {
        self.header.get_value()[index as usize].clone()
    }

    pub fn insert_header(&mut self, name: &TString) {
        self.header.push(name.clone());
    }

    pub fn insert_header_list(&mut self, names: &Vec<TString>) {
        let temp = Triadic::new('t'.enum_convert());
        let mut tstring = TString::new("".to_string(), temp);
        for i in names.iter(){
          let t = tstring.tand_1(i);
          tstring.set_degree(t);
        }
        self.header.set_vector(names.clone());
        self.header.set_degree(tstring.get_degree());
    }

    pub fn print_table_row_wise(&self) {
        let mut i = 0;
        for r in self.data.iter(){
            println!("TRow {}: ", i);
            r.print_row();
            i += 1;
        }
    }

    pub fn print_table_col_wise(&self) {
        for name in self.header.get_value().iter() {
            println!("Column Name: ");
            name.t_print();
            let temp = self.get_column_by_name(&name);

            for val in temp {
                match val {
                    RDataType::Float(v) => v.t_print(),
                    RDataType::String(v) => v.t_print(),
                    _ => println!("None")
                }
            }

        }
    }
}