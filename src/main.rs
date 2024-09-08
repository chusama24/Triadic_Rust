pub mod helper_functions;
use helper_functions::*;
use std::{collections::HashMap, f32::consts::E};

use triadic_rust::{
    data_structures::{
        t_row::{RDataType, TRow},
        t_table::Ttable,
    }, data_types::{t_f32::TF32, t_string::TString, triadic::Triadic, triadic_type::Ttypes}, operators::triadic_op::TriadicArithmeticOp, t_enum::Degree, t_print::Print, type_cast::convertto_string
};

fn bernoulli_naive_bayes(
    data: &Ttable,
    class_label: TString,
    document_label: TString,
    alpha: f32,
) -> (
    HashMap<TString, TF32>,
    HashMap<TString, HashMap<TString, TF32>>,
) {
    let mut probabilities: HashMap<TString, TF32> = HashMap::<TString, TF32>::new();
    type InnerMap = HashMap<TString, TF32>;
    type OuterMap = HashMap<TString, InnerMap>;
    let mut prob_w_c: OuterMap = HashMap::new();
    let mut degrees = Vec::<Triadic>::new();
    degrees.push(Triadic::new(Degree::T));
    degrees.push(Triadic::new(Degree::F));
    degrees.push(Triadic::new(Degree::L));
    
    let class = data.get_column_by_name(&class_label);
    
    let mut total_rows = HashMap::<Triadic, i32>::new();
    for deg in degrees.iter(){
        let count = get_total_of_document_in_degree(class.clone(), deg.clone());
        total_rows.insert(deg.clone(), count);
    }

    let mut labels = Vec::<TString>::new();
    for c in class.iter() {
        match c {
            RDataType::String(c) => {
                if labels.contains(&c) != true {
                    labels.push(c.clone());
                }
            }
            RDataType::Char(c) => {
                let temp= convertto_string(c.clone());
                if labels.contains(&temp) != true {

                    labels.push(temp.clone());
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

    let vocab = get_vocab(&data, document_label.clone());
    println!("bernoulli_naive_bayes: Vocab saved.");
    for j in labels.iter() {
        let mut temp_map: InnerMap = HashMap::new();
        let number_of_documents: Option<&Vec<i32>> = data_points.get(j);
        match number_of_documents {
            Some(number_of_documents) => {
                let num = number_of_documents.len() as f32;
                let total = total_rows
                .get(&j.get_degree())
                .expect("Unable to find total number of documents");
                let p = TF32::new(num / (*total as f32), j.get_degree());
                probabilities.insert(j.clone(), p);
                for v in vocab.iter() {
                    // degrees[1] means false
                    if v.get_degree() == degrees[1].clone() {
                        continue;
                    }
                    let count = get_no_of_document_containing_words(
                        &data,
                        number_of_documents.clone(),
                        v.clone(),
                        document_label.clone(),
                    );

                    let p_w_c = (count + alpha) as f32 / (num + 2.0 * alpha) as f32;
                    //println!("value{}  probability:{}", p_w_c, v.get_value());
                    temp_map.insert(v.clone(), TF32::new(p_w_c, v.get_degree()));
                }
                prob_w_c.insert(j.clone(), temp_map);
            }
            None => print!("Failed to get number of documets"),
        }
    }
    return (probabilities, prob_w_c);
}

pub fn train_bnb(
    training: String,
    class_label: TString,
    document_label: TString,
    alpha: f32, 
) -> (
    HashMap<TString, TF32>,
    HashMap<TString, HashMap<TString, TF32>>,
) {
    let mut trained_model: (
        HashMap<TString, TF32>,
        HashMap<TString, HashMap<TString, TF32>>,
    ) = (HashMap::new(), HashMap::new());
    let data = Ttable::read_data(training, class_label.clone());
    match data {
        Ok(data) => {
            println!("Data sucessfully read");
            // data.print_table_row_wise();
            trained_model = bernoulli_naive_bayes(&data, class_label, document_label, alpha);
        }
        Err(error) => {
            println!("Failed to retrieve data: {}", error);
        }
    }
    trained_model
}
fn test_bnb(
    file_name: String,
    trained_model: (
        HashMap<TString, TF32>,
        HashMap<TString, HashMap<TString, TF32>>,
    ),
    document_label: TString,
    class_label: TString,
) {
    let data = Ttable::read_data(file_name, class_label.clone());
    match data {
        Ok(data) => {
            let size = data.get_size();
            for i in 0..size {
                let result=testing_bnb(
                    data.get_row(i as i32),
                    trained_model.clone(),
                    document_label.clone(),
                );
                println!("Row number: {}",i);
                match result {
                    Ok(result) => {
                        print! {"Selected class: "};
                        result.t_print();
                    },
                    Err(str) => {
                        println!("{}",str);
                    },
                }
            }
            println!("Data sucessfully read");
        }
        Err(error) => {
            println!("Failed to retrieve data: {}", error);
        }
    }
}
fn testing_bnb(
    instance: TRow,
    trained_model: (
        HashMap<TString, TF32>,
        HashMap<TString, HashMap<TString, TF32>>,
    ),
    document_label: TString,
) -> Result<TString,String> {
    let mut doc_words = Vec::<TString>::new();
    let mut probabilities = HashMap::<TString, TF32>::new();
    let prior_probabilities: HashMap<TString, TF32> = trained_model.0;
    let word_probabilities = trained_model.1;
    let class_labels: Vec<&TString> = prior_probabilities.keys().collect();
    let doc: Option<RDataType> = instance.get_value(&document_label);
    let stop_words = stop_words::get(stop_words::LANGUAGE::English);

    let mut degrees = Vec::<Triadic>::new();
    degrees.push(Triadic::new(Degree::T));
    degrees.push(Triadic::new(Degree::F));
    degrees.push(Triadic::new(Degree::L));
    match doc {
        Some(doc) => match doc {
            RDataType::String(doc) => {
                println!("Doc value");
                doc.t_print();
                if doc.get_degree() == degrees[1].clone() {
                    return Err("Degree of the document is false.".to_string());
                }
                let text = doc.get_value();
                let words: Vec<&str> = text.split_whitespace().collect();
                for w in words.clone().into_iter() {
                    let w_store = w.to_lowercase();
                    let cleaned_text = remove_non_alphanumeric(&w_store);
                    if cleaned_text.len() == 0 {
                        continue;
                    }
                    if stop_words.contains(&cleaned_text) {
                        continue;
                    }
                    let temp = TString::new(cleaned_text, doc.get_degree());
                    doc_words.push(temp);
                }
                if doc_words.len() == 0{
                    return Err("Document is empty.".to_string());
                }
            }
            RDataType::Empty => {
                return Err("testing_bnb: Document not found".to_string());
            }
            _ => print! {"testing_bnb: doc is not in TString"},
        },
        None => return Err("testing_bnb: Document not found".to_string()), 
    }
    let mut t = TF32::default();
    for c in class_labels {
        let class_probabilities = word_probabilities.get(c);
        let mut c_prob = prior_probabilities
            .get(c)
            .expect("Value should not be none");
        match class_probabilities {
            Some(class_probabilities) => {
                for (key, val) in class_probabilities {
                    if doc_words.contains(key) {
                        t = c_prob.tmul_1(*val);
                        c_prob = &t;
                    } else {
                        let temp = TF32::new(1.0, Triadic::new(Degree::T));
                        t = c_prob.tmul_1(temp.tsub_1(*val));
                        c_prob = &t;
                    }
                }
            }
            None => return Err("testing_bnb: class_probabilities not found".to_string()),
        }
        probabilities.insert(c.clone(), c_prob.clone());
    }
    let mut max = 0.0 as f32;
    let mut selected_class = TString::default();
    for (k, v) in probabilities.iter() {
        if v.get_value() > max {
            max = v.get_value();
            selected_class = k.clone();
        }
    }

    return Ok(selected_class);
}

fn main() {
    let training: String = "training_triadicv3_dataset.csv".to_owned();
    let testing: String = "testing_triadicv3_dataset.csv".to_owned();
    let class_label = TString::new("Class".to_owned(), Triadic::new(Degree::T));
    let document_label = TString::new("Sentence".to_owned(), Triadic::new(Degree::T));
    let alpha = 1.0 as f32;
    let trained_model = train_bnb(training, class_label.clone(), document_label.clone(), alpha);

    test_bnb(testing, trained_model, document_label, class_label);
}
