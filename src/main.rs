pub mod helper_functions;
use helper_functions::*;
use std::collections::HashMap;

use triadic_rust::{data_structures::{t_row::{RDataType, TRow}, t_table::Ttable}, data_types::{t_f32::TF32, t_string::TString, triadic::Triadic, triadic_type::Ttypes}, t_enum::Degree, t_print::Print};

fn bernoulli_naive_bayes(
 data: &Ttable,
    class_label: TString,
    document_label: TString,
    alpha:f32,
)->(HashMap<TString, TF32>, HashMap<TString, HashMap<TString, f32>>){
    let mut probabilities: HashMap<TString, TF32>= HashMap::<TString, TF32>::new();
    type InnerMap = HashMap<TString, f32>;
    type OuterMap = HashMap<TString, InnerMap>;
    let mut prob_w_c: OuterMap = HashMap::new();
    let true_check=Triadic::new(Degree::T);
    if true_check!=class_label.get_degree(){
        return(probabilities,prob_w_c);
    }
    let class = data.get_column_by_name(&class_label);
    let total_rows=class.len();
    let mut labels = Vec::<TString>::new();
    for c in class.iter() {
        match c {
            RDataType::String(c) => {
                if labels.contains(&c) != true {
                    labels.push(c.clone());
                }
            }
            _ => println!("Value other than string."),
        }
    }

    let mut data_points: HashMap<TString, Vec<i32>> = HashMap::<TString, Vec<i32>>::new();
    for j in labels.iter() {
        let temp = get_data_points(class.clone(), j.clone());
        data_points.insert(j.clone(), temp);
    }

    let vocab =get_vocab(&data,document_label.clone());
    println!("bernoulli_naive_bayes: Vocab saved.");
    for j in labels.iter(){
        let mut temp_map: InnerMap = HashMap::new();
        let mut count =0.0;
        let number_of_documents: Option<&Vec<i32>>=data_points.get(j);
        match number_of_documents{
            Some(number_of_documents) => {
            let num=number_of_documents.len()as f32;
            let p=TF32::new (num/total_rows as f32,j.get_degree());
            probabilities.insert(j.clone(),p);
            for v in vocab.iter(){
                if v.get_degree()!=true_check{
                    continue;
                }
                count = get_no_of_document_containing_words(&data,number_of_documents.clone(),v.clone(),document_label.clone());
                
                let p_w_c= (count+alpha)as f32/(num + 2.0*alpha)as f32;
                println!("value{}  probability:{}",p_w_c,v.get_value());
                temp_map.insert(v.clone(),p_w_c);
            }
            prob_w_c.insert(j.clone(),temp_map);
            },
            None => print!("Failed to get number of documets"),
        }

    }
    return(probabilities,prob_w_c);
}

pub fn train_bnb(
    training: String,
    class_label: TString,
    document_label:TString,
    alpha:f32,
)->(HashMap<TString, TF32>, HashMap<TString, HashMap<TString, f32>>){
    let mut trained_model: (
        HashMap<TString, TF32>,
        HashMap<TString, HashMap<TString, f32>>,
    ) = (HashMap::new(), HashMap::new());
let data = Ttable::read_data(training, class_label.clone());
    match data {
        Ok(data) => {
            println!("Data sucessfully read");
           // data.print_table_row_wise();
           trained_model =bernoulli_naive_bayes(&data, class_label,document_label,alpha);
        }
        Err(error) => {
            println!("Failed to retrieve data: {}", error);
        }
    }
    trained_model
}
fn test_bnb(file_name:String,trained_model:(HashMap<TString, TF32>, HashMap<TString, HashMap<TString, f32>>),document_label: TString,class_label: TString){
    let data = Ttable::read_data(file_name, class_label.clone());
    match data {
        Ok(data) => {
            let size=data.get_size();
            for i in 0..size{
                testing_bnb(data.get_row(i as i32),trained_model.clone(),document_label.clone());
            }
            println!("Data sucessfully read");
           // data.print_table_row_wise();
        }
        Err(error) => {
            println!("Failed to retrieve data: {}", error);
        }
    }
}
fn testing_bnb(instance:TRow,trained_model:(HashMap<TString, TF32>, HashMap<TString, HashMap<TString, f32>>),document_label:TString){
let mut doc_words= Vec::<TString>::new();
let mut probabilities=HashMap::<TString,f32>::new();
let prior_probabilities: HashMap<TString, TF32>=trained_model.0;
let word_probabilities=trained_model.1;
let class_labels:Vec<&TString>=prior_probabilities.keys().collect();
let doc: Option<RDataType> =instance.get_value(&document_label);
let stop_words = stop_words::get(stop_words::LANGUAGE::English);

match doc{
    Some(doc) =>{
        match doc{
            RDataType::String(doc) =>{
                let text=doc.get_value();
                let words: Vec<&str> = text.split_whitespace().collect();
                    for w in words.clone().into_iter(){
                        let w_store =w.to_lowercase();
                        let cleaned_text= remove_non_alphanumeric(&w_store); 
                        if cleaned_text.len() == 0{
                            continue;
                        }
                        if stop_words.contains(&cleaned_text){
                            continue;
                        }
                        let temp=TString::new(cleaned_text,doc.get_degree());
                        doc_words.push(temp);
                }
            },
            _=>print!{"testing_bnb: doc is not in TString"},
        }
    },
    None => print!{"testing_bnb: Document not found"},
}
let true_check=Triadic::new(Degree::T);
for c in class_labels{
    if c.get_degree()!=true_check{
        continue;
    }
    let class_probabilities=word_probabilities.get(c);
    let mut c_prob: f32=prior_probabilities.get(c).expect("Value should not be none").get_value();
    match class_probabilities{
        Some(class_probabilities) =>{
            for (key,val) in class_probabilities{
                if doc_words.contains(key){
                    c_prob*=*val;
                }else{
                    c_prob*=1.0-*val;
                }
            }
        },
        None => print!{"testing_bnb: class_probabilities not found"},
    }
    probabilities.insert(c.clone(), c_prob);
}
let mut max=0.0 as f32;
let mut selected_class=TString::default();
for (k,v) in probabilities.iter(){
if *v >max{
    max=*v;
    selected_class=k.clone();
}
}
print!{"Selected class: "};
selected_class.t_print();
}

fn main() {
    let training: String = "training_triadicv1_dataset.csv".to_owned();
    let testing: String = "testing_triadicv1_dataset.csv".to_owned();
    let class_label = TString::new("Class".to_owned(), Triadic::new(Degree::T));
    let document_label = TString::new("Sentence".to_owned(), Triadic::new(Degree::T));
    let alpha=1.0 as f32; 
    let trained_model=train_bnb(training,class_label.clone(),document_label.clone(),alpha);
    
    test_bnb(testing,trained_model,document_label,class_label);
}


