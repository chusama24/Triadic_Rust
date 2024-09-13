
use std::collections::HashMap;

use triadic_rust::{data_structures::{t_row::RDataType, t_table::Ttable}, data_types::{t_f32::TF32, t_string::TString, triadic::Triadic, triadic_type::Ttypes}, t_enum::Degree, t_print::Print};

pub fn remove_non_alphanumeric(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphanumeric()).collect()
} 

pub fn get_data_points(data: Vec<RDataType>, key: TString) -> Vec<i32> {
    let mut examples = Vec::<i32>::new();
    let mut temp = TString::default();
    for j in 0..data.len() {
        let val = data[j].clone();
        match val {
            RDataType::Integer(ti32) =>{
                temp.set_value(ti32.get_value().to_string());
                temp.set_degree(ti32.get_degree());
            }
            RDataType::String(str) => {
                temp = str;
            },
            RDataType::Char(ch)=>{
                temp.set_value(ch.get_value().to_string());
                temp.set_degree(ch.get_degree());
            }
            RDataType::Float(tf32) => {
                temp.set_value(tf32.get_value().to_string());
                temp.set_degree(tf32.get_degree());
            }, 
            _ => {
                temp.set_value("Empty".to_string());
                temp.set_degree(Triadic::default());
            }
        }
        if temp.get_degree().get_value() == key.get_degree().get_value(){
            if temp.get_value().eq(&key.get_value()){
                examples.push(j as i32);
            }
        }
    }
    examples
}


pub fn print_trained_model(trained_model:(HashMap<TString, Vec<TString>>,HashMap<TString, HashMap<TString, (TF32, TF32)>>,))
{
    
    print!("Encoded data: ");
    for (key, values) in &trained_model.0 {
        print!("Attribute:");
        key.t_print();
        println!("    Values:");
        for j in values.iter(){
         j.t_print();
        }
    }

    print!("Mean and Standard Deviation :");

    for (outer_key, inner_map) in &trained_model.1 {
      println!("Outer key: ");
      outer_key.t_print();
      for (inner_key, (mean, sd)) in inner_map {
          println!("  Inner key:");
          inner_key.t_print();
          println!("Mean:");
          mean.t_print();
          println!("SD:");
          sd.t_print();
      }
  }


}

pub fn get_vocab(data:&Ttable,col_name:TString)->Vec<TString>{
let stop_words = stop_words::get(stop_words::LANGUAGE::English);
let col=data.get_column_by_name(&col_name);
let false_check =Triadic::new(Degree::F);
col_name.t_print();
let mut v=Vec::<TString>::new();
for i in col.iter(){
    match i {
        RDataType::String(line) => {
            let str=line.get_value();
            //print!("{} ",str);
            let words: Vec<&str> = str.split_whitespace().collect();
            for w in words{
                let w_store =w.to_lowercase();
                let cleaned_text= remove_non_alphanumeric(&w_store); 
                if cleaned_text.len() == 0{
                    continue;
                }
                if stop_words.contains(&cleaned_text){
                    continue;
                }
                let temp =TString::new(cleaned_text,line.get_degree());
                if temp.get_degree()!=false_check && v.contains(&temp)!=true{
                    v.push(temp);
                }
            }
        },
        _=>print!("get_vocab Data type was not in string")
    }
    }
return v;
}
pub fn get_no_of_document_containing_words(data:&Ttable,document_indices:Vec<i32>,word:TString,document_label:TString)->f32
{
let stop_words = stop_words::get(stop_words::LANGUAGE::English);
let mut count=0.0;
let true_check=Triadic::new(Degree::T);
for di in document_indices.iter(){
    let row=data.get_row(*di);
    let text= row.get_value(&document_label);
    match text{
        Some(text)=>{
            match text{
                RDataType::String(text) =>{
                    if text.get_degree()!=true_check{
                        continue;
                    }
                    let str=text.get_value();
                    //print!("{} ",str);
                    let words: Vec<&str> = str.split_whitespace().collect();
                    for w in words{
                        let w_store =w.to_lowercase(); 
                        let cleaned_text= remove_non_alphanumeric(&w_store); 
                        if cleaned_text.len() == 0{
                            continue;
                        }
                        if stop_words.contains(&cleaned_text){
                        continue;
                        }
                        if cleaned_text.eq(&word.get_value()){
                            count=count+1.0;
                            break;
                        }
                    }
                },
                _=>print!{"get_no_of_document_containing_words: Data type not found"},
            }
        }
        None => print!{"get_no_of_document_containing_words:Text not found "},
    }
}
count
}

pub fn get_total_of_document_in_degree(class_col: Vec<RDataType>, deg: Triadic) -> i32 {
    let mut count = 0;
    for c in class_col.iter() {
            match c {
                RDataType::String(str) => {
                    if str.get_degree() == deg {
                        count = count + 1;
                    }
                },
                RDataType::Char(ch) => {
                    if ch.get_degree() == deg {
                        count = count + 1;
                    }
                },
                _ => println!("get_total_of_document_in_degree: Invalid Class label datatype")
            }
    }
    count
}
pub fn get_max_value(values:&Vec<(Triadic,TF32)>)-> usize{
    let mut max_index:usize=0;
    let mut max= f32::MIN;
    for (i,(_deg,prob)) in values.iter().enumerate(){
        if prob.get_value()>max{
            max=prob.get_value();
            max_index=i;
        }
    } 
    return max_index;
}