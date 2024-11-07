use crate::{data_types::{t_bool::TBool, t_char::TChar, t_f32::TF32, t_i32::TI32, t_string::TString, triadic::Triadic, triadic_type::Ttypes}, operators::triadic_op::TriadicStringOp, t_enum::{ConvertToDegree, Degree}, t_print::Print};
use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;
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

    pub fn insert_val(&mut self, val: RDataType, row_index: i32, col_name: TString) {
        let index = row_index as usize;
        if index  < self.data.len(){
            self.data[index].insert(&col_name, &val);
        } 
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
RDataType::Integer(TI32::default());
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
        
        let mut deg = self.header.get_degree();
        if self.header.get_value().len() == 0 {
            deg.set_value('t'.enum_convert());
        }

        self.header.push(name.clone());
        let mut temp = TString::default();
        temp.set_degree(deg);
        self.header.set_degree(name.tand_1(&temp));
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

    pub fn remove_header(&mut self, name: &TString) {
        let mut i = 0;
        for n in self.header.get_value() {
            if n == *name {
                self.header.remove(i);
            }
            i += 1;
        }
        self.header.set_degree(Triadic::new(Degree::T));

        for n in self.header.get_value(){
            let deg = self.header.get_degree();
            let temp = TString::new("temp".to_string(), deg);
            self.header.set_degree(n.tand_1(&temp));
        }
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
                    RDataType::Integer(v)=> v.t_print(),
                    RDataType::Char(v) => v.t_print(),
                    RDataType::Bool(v) => v.t_print(),
                    _ => println!("None")
                }
            }

        }
    }

    pub fn get_column_type(&self, index: i32) -> String {
        let attr = self.get_header(index);
        let mut i = 0;
        loop {
            if i >= self.data.len(){
                break;
            }
            let val =  self.data[i].get_value(&attr);
            if let Some(v) = val {
                match v {
                    RDataType::Char(_c) => return "char".to_string(),
                    RDataType::Float(_c) => return "float".to_string(),
                    RDataType::Integer(_c) => return "integer".to_string(),
                    RDataType::String(_c) => return "string".to_string(),
                    RDataType::Bool(_c) => return "bool".to_string(),
                    _ => return "None".to_string()
                }
            }
            else {
                i += 1;
            }
        }
        return "None".to_string();
    }

    pub fn get_column_type_by_name(&self, attr: &TString) -> String {
        let mut i = 0;
        loop {
            if i >= self.data.len(){
                break;
            }
            let val =  self.data[i].get_value(&attr);
            if let Some(v) = val {
                match v {
                    RDataType::Char(_c) => return "char".to_string(),
                    RDataType::Float(_c) => return "float".to_string(),
                    RDataType::Integer(_c) => return "integer".to_string(),
                    RDataType::String(_c) => return "string".to_string(),
                    RDataType::Bool(_c) => return "bool".to_string(),
                    _ => return "None".to_string()
                }
            }
            else {
                i += 1;
            }
        }
        return "None".to_string();
    }

   pub fn read_data(file_path: String) -> Result<Ttable, std::io::Error>{
        // Open the file
        let f: File;
        match File::open(file_path) {
            Ok(file) => {
                println!("File successfully opened!");
                f = file;
            }
            Err(err) => {
                return Err(err);
            }
        }
        let reader = BufReader::new(f);
    
        let mut table = Ttable::new();
    
        // Create a CSV reader
        let mut csv_reader = Reader::from_reader(reader);
    
        let headers = csv_reader.headers()?;
        let mut i = 0;
            for header in headers{
                if i % 2 == 1{
                    i += 1;
                    continue;
                }
                i += 1;
                // Outlook/T val
                let names= header.split('/');
                // "Outlook" , "T val"
                let mut name = String::new();
                let mut temp_deg = char::default();
                for (j,w) in names.enumerate(){
                    if j == 0{
                        name = w.to_string();
                    }
                    else{
                        let d = w.split(' ');
                        // "T" , "val"
                        for (_k,w2) in d.enumerate(){
                            let temp: Result<char, _> = w2.to_string().trim().parse();
                            if let Ok(val) = temp {
                                temp_deg = val;
                            }
                            else {
                                temp_deg = 'L';
                            }
                            break;
                        }
                    }
                };
                let n = TString::new(name.clone(), Triadic::new(temp_deg.enum_convert()));
                table.insert_header(&n);
            }

            // table.remove_header(&class_label);
            // table.insert_header(&class_label);


    
    let records = csv_reader.records();
     for record in records.into_iter(){
        match record {
            Ok(record) => { 
                let mut i = 0;
                let mut temp = RDataType::Empty;
                let mut row = TRow::new();
                for r in &record{
                    if i % 2 == 0{
                        temp = convert_input(r.to_string());
                    }
                        else {
                            let mut deg = 'L';
                            let t_deg: Result<char, _> = r.to_string().trim().parse();
                            match t_deg {
                                Ok(val) => {
                                    // Degree exists
                                deg = val;
                                },
                                Err(_err) => {
                                    match temp {
                                        RDataType::Empty => {
                                            // Value and degree both are missing
                                            let index: i32 = (i-1)/2;
                                            let attr = table.get_header(index);
                                            row.insert(&attr, &temp);
                                            continue;
                                        }, 
                                        _ => {
                                            // Value exists but degree is missing
                                        }
                                    }
                                },
                            } 
                            let d = Triadic::new(deg.enum_convert());
                            let index: i32 = (i-1)/2;
                            let attr = table.get_header(index);
                            match temp {
                                RDataType::Char(ref mut v) => v.set_degree(d),
                                RDataType::Float(ref mut v) => v.set_degree(d),
                                RDataType::Integer(ref mut v) => v.set_degree(d),
                                RDataType::String(ref mut v) => v.set_degree(d),
                                RDataType::Bool(ref mut v) => v.set_degree(d),
                                RDataType::Empty => {
                                    temp = RDataType::String(TString::new("".to_string(), Triadic::new('f'.enum_convert())));
                                }
                            }
                            row.insert(&attr, &temp);
                    }
                    i += 1;
                }
                table.insert_row(&row);
            }
            Err(error) => {
                println!("Error while reading record: {}", error); 
            }
        }
     }    
        Ok(table)
    }  

    pub fn remove_column_by_name(&mut self,name:TString){
        for row in self.data.iter_mut(){
            row.remove_val(&name);
        }
        self.remove_header(&name);
    }
}

pub fn convert_input(input: String)-> RDataType {
    let temp = input.trim();
    if temp.is_empty(){
        return RDataType::Empty;
    }
    match i32::from_str(&input){
        Ok(num) => {
           return RDataType::Integer(TI32::new(num, Triadic::default()))
        },
        Err(_err) => {
            match f32::from_str(&input){
                Ok(num) => {
                    return RDataType::Float(TF32::new(num, Triadic::default()))
                }, 
                Err(_err) => {
                    match char::from_str(&input){
                        Ok(c) => {
                            return RDataType::Char(TChar::new(c, Triadic::default()))
                        },
                        Err(_err) => {
                            let temp = input.to_ascii_lowercase();
                            match bool::from_str(&temp) {
                                Ok(b) => {
                                    return RDataType::Bool(TBool::new(b, Triadic::default()))
                                },
                                Err(_err) => {
                                    return RDataType::String(TString::new(input, Triadic::default()))
                                },
                            }
                        }
                    }
                }
            }
        },
    }
}

