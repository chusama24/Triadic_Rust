use std::collections::HashMap;

use triadic_rust::{data_structures::{t_row::RDataType, t_table::Ttable}, data_types::{t_string::TString, triadic::Triadic, triadic_type::Ttypes}, t_enum::Degree};

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

pub fn remove_non_alphanumeric(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphanumeric()).collect()
} 
pub fn get_vocab(data:&Ttable,col_name:TString,class_label:TString)->(HashMap<TString,i32>,HashMap<TString,i32>){
    let stop_words = stop_words::get(stop_words::LANGUAGE::English);
    let col=data.get_column_by_name(&col_name);
    //col_name.t_print();
    let mut total_words_in_class = HashMap::<TString,i32>::new();
    let mut v=HashMap::<TString,i32>::new();
    let class_col = data.get_column_by_name(&class_label);
    for (j,i) in col.iter().enumerate(){
        match i {
            RDataType::String(line) => {
                let str=line.get_value();
                //print!("{} ",str);
                let words: Vec<&str> = str.split_whitespace().collect();
                for w in words{
                    let w_store =w.to_lowercase();
                    let cleaned_text= remove_non_alphanumeric(&w_store); 
                    if stop_words.contains(&cleaned_text){
                        continue;
                    }
                    if cleaned_text.len()>0{
                        let temp =TString::new(cleaned_text.to_lowercase(),line.get_degree());
                        if v.contains_key(&temp) {
                            let val = v.get_mut(&temp);
                            match val {
                                Some(val) => {
                                    *val += 1;
                                },
                                None => {},
                            }
                        } else {
                            v.insert(temp, 1);
                        }
                    }
                    //println!("{}",cleaned_text);
                    let label=&class_col[j];
                    match label {
                        RDataType::String(label)=>{
                            if total_words_in_class.contains_key(&label) {
                                let val = total_words_in_class.get_mut(&label);
                                match val {
                                    Some(val) => {
                                        *val += 1;
                                    },
                                    None => {},
                                }
                            } else {
                                total_words_in_class.insert(label.clone(), 1);
                            }
                        },
                        _=> println!("Class label isnt string")
                    }
                }
            },
            _=>print!("get_vocab Data type was not in string")
        }
        }
    return (v,total_words_in_class);
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

pub fn total_words_in_a_class(data:&Ttable,no_of_doc:Vec<i32>,document_label:TString)->f32
{
    let total_words=0.0 ;
    let stop_words = stop_words::get(stop_words::LANGUAGE::English);
    let true_check=Triadic::new(Degree::T);
    for i in no_of_doc{
        let row=data.get_row(i);
        let text= row.get_value(&document_label);
        let mut total_words=0.0;
        match text{
            Some(text)=>{
                match text{
                    RDataType::String(text) =>{
                        if text.get_degree()!=true_check{
                            continue;
                        }

                        let str=text.get_value();
                        let words: Vec<&str> = str.split_whitespace().collect();
                        for w in words{
                            let w_store =w.to_lowercase();
                            let cleaned_text= remove_non_alphanumeric(&w_store); 
                            if stop_words.contains(&cleaned_text){
                                continue;
                            }
                            
                                total_words = total_words+ 1.0;
                        }
                    },
                    _=>print!{"get_no_of_document_containing_words: Data type not found"},
                }
            }
            None => print!{"get_no_of_document_containing_words:Text not found "},
        }
    }
total_words
}