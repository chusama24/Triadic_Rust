use std::collections::HashMap;
pub mod helper_function;
use helper_function::*;
use triadic_rust::{
    data_structures::{
        t_row::{RDataType, TRow},
        t_table::Ttable,
    },
    data_types::{t_f32::TF32, t_string::TString, triadic::Triadic, triadic_type::Ttypes},
    t_enum::Degree,
    t_print::Print, type_cast::convertto_string,
};

pub fn model_traning(
    data: &Ttable,
    class_label: TString,
    document_label: TString,
) -> (
    HashMap<TString, TF32>,
    HashMap<TString, HashMap<TString, TF32>>,
    HashMap<TString, HashMap<TString, i32>>,
    HashMap<TString, i32>,
) {

    let mut degrees = Vec::<Triadic>::new();
    degrees.push(Triadic::new(Degree::T));
    degrees.push(Triadic::new(Degree::F));
    degrees.push(Triadic::new(Degree::L));

    //Check the degree of class
    if class_label.get_degree() != degrees[0].clone() {
        print!("Class_label is not true");
    }

    let class_col = data.get_column_by_name(&class_label);
    let mut labels = Vec::<TString>::new(); //contain name of all the class lables i.e spam, ham

    //Getting class labels
    for c in class_col.iter() {
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
            _ => print!("Label value other than string"),
        }
    }

    //To get the indexes of doc related to a specific class

    let mut data_points = HashMap::<TString, Vec<i32>>::new();
    for l in labels.iter() {
        let temp = get_data_points(class_col.clone(), l.clone());
        data_points.insert(l.clone(), temp);
    }

    //calculating class probabilities
    let mut class_probabilities = HashMap::<TString, TF32>::new();
    // let total_rows = class_col.len();
    let mut total_rows = HashMap::<Triadic, i32>::new();

    for deg in degrees.iter() {
        let count = get_total_of_document_in_degree(class_col.clone(), deg.clone());
        total_rows.insert(deg.clone(), count);
    }

    let (vocab, class_total_words) = get_vocab(&data, document_label.clone(), class_label);
    let mut vocab_size = 0 as i32;

    for (_k, val) in vocab.iter() {
        for (_k1, v) in val {
            vocab_size += v;
        }
    }
    //print!("{}",vocab_size);
    for j in labels.iter() {
        let no_of_doc = data_points.get(j);
        match no_of_doc {
            Some(no_of_doc) => {
                let num = no_of_doc.len() as f32;
                let total = total_rows
                    .get(&j.get_degree())
                    .expect("Unable to find total number of documents");
                let prob_of_class = TF32::new(num / (*total as f32), j.get_degree());
                class_probabilities.insert(j.clone(), prob_of_class);
            }
            None => print!("Failed to get data points"),
        }
    }
   let prob_w_c = calculate_word_probabilities(vocab.clone(), vocab_size, class_total_words.clone());
    //P(w∣C)= WordCount[C][w]+1/TotalWords[C]+Vocabulary size ;
    return (class_probabilities, prob_w_c, vocab, class_total_words);
}

pub fn read_data(training: String, class_label: TString) -> Ttable {
    let data = Ttable::read_data(training, class_label.clone()).expect("Failed to retrieve data");
    println!("Data successfully read");
    data
}

pub fn model_testing(
    data: TRow,
    mut trained_model: (
        HashMap<TString, TF32>,
        HashMap<TString, HashMap<TString, TF32>>,
        HashMap<TString, HashMap<TString, i32>>,
        HashMap<TString, i32>,
    ),
    document_label: TString,
) -> Result<(TString,(HashMap<TString, TF32>,
    HashMap<TString, HashMap<TString, TF32>>,
    HashMap<TString, HashMap<TString, i32>>,
    HashMap<TString, i32>)),String> {
    let doc = data.get_value(&document_label);
    let stop_words = stop_words::get(stop_words::LANGUAGE::English);
    let mut doc_words = Vec::<TString>::new();

    let mut degrees = Vec::<Triadic>::new();
    degrees.push(Triadic::new(Degree::T));
    degrees.push(Triadic::new(Degree::F));
    degrees.push(Triadic::new(Degree::L));

    match doc {
        Some(doc) => {
            match doc {
                RDataType::String(doc) => {
                    if doc.get_degree() == degrees[1].clone() {
                        return Err("Degree of the document is false.".to_string());
                    }
                    let str = doc.get_value();
                    print!("Doc value {}",str);
                    if str==""{
                        return Err("Doc value was empty".to_string());
                    }
                    //print!("{} ",str);
                    let words: Vec<&str> = str.split_whitespace().collect();
                    for w in words {
                        let w_store = w.to_lowercase();
                        let cleaned_text = remove_non_alphanumeric(&w_store);
                        if stop_words.contains(&cleaned_text) || cleaned_text.len()==0 {
                            continue;
                        }
                        let temp = TString::new(cleaned_text, doc.get_degree());
                        doc_words.push(temp);
                    }
                },
                RDataType::Empty => {
                    return Err("Document value not found.".to_string());
                }
                _ => println!("Doc value is not string"),
            }
        }
        None => {return Err("Document not found".to_string());}
    }

    let prior_probabilities: HashMap<TString, TF32> = trained_model.0.clone();
    let class_labels: Vec<&TString> = prior_probabilities.keys().collect();
    let mut p_w_c: HashMap<TString, HashMap<TString, TF32>> = trained_model.1.clone();
    let mut vocab = trained_model.2.clone();
    let total_words_in_class = trained_model.3.clone();
    let mut probabilities = HashMap::<TString, TF32>::new();
    let mut vocab_size = 0 as i32;
    let (require_retraining, new_vocab, new_total_words_in_class) = check_unseen_words(
        doc_words.clone(),
        vocab.clone(),
        total_words_in_class,
        class_labels.clone(),
    );
    if require_retraining {
        // Retrain model
        for (_k, val) in vocab.iter() {
            for (_k1, v) in val {
                vocab_size += v;
            }
            
        }
        p_w_c = calculate_word_probabilities(new_vocab.clone(), vocab_size, new_total_words_in_class.clone());
       
        vocab = new_vocab;
        trained_model.1=p_w_c.clone();
        trained_model.2 = vocab.clone();
        trained_model.3=new_total_words_in_class;
    }
    for c in class_labels.clone() {
        let mut c_prob = prior_probabilities
            .get(c)
            .expect("Value should not be none").clone();
        let val=c_prob.get_value();
        let log_prob = val.ln();
        c_prob.set_value(log_prob);

        let class_probabilities = p_w_c.get(c);

        match class_probabilities {
            Some(prob) =>{
                let prob= calculate_prob(prob,doc_words.clone(),vocab.clone(),c_prob);
                // println!("Prob:");
                // prob.t_print();
                probabilities.insert(c.clone(), prob);
            },
            None => return Err("Class probabilities cant be found".to_string()),
        } 
    }

    let mut max =f32::MIN;
    let mut selected_class = TString::default();
    for (k, v) in probabilities.iter() {
        // println!("class name");
        // k.t_print();
        // println!("probabilities val ");
        // v.t_print();
        
        if v.get_value() > max {
            max = v.get_value();
            selected_class = k.clone();
        }
    }

    return Ok((selected_class,trained_model));
}
fn main() {
    let training: String = "training_triadicv3_dataset.csv".to_owned();
    let class_label = TString::new("Class".to_owned(), Triadic::new(Degree::T));
    let document_label = TString::new("Sentence".to_owned(), Triadic::new(Degree::T));

    let data = read_data(training, class_label.clone());

    let mut trained_model = model_traning(&data, class_label.clone(), document_label.clone());

    let testing: String = "testing_triadicv3_dataset.csv".to_owned();
    let testing_data = read_data(testing, class_label.clone());

    let size = testing_data.get_size();
    if size==0{
        println!("File is empty");
    }
    // let prior_prob = trained_model.0.clone();
    // for (k,v) in prior_prob{
    //     println!("class:");
    //     k.t_print();
    //     println!("prob:{} ",v.get_value());
    // }
    // let word =TString::new("sad".to_string(),Triadic::new(Degree::T) );
    // print_specific_word(trained_model.1.clone(),&word);
    for i in 0..size {
        let result= model_testing(
            testing_data.get_row(i as i32),
            trained_model.clone(),
            document_label.clone(),
        );
        
        println!("Row number: {}",i);
        match result {
            Ok(result) => {
                let selected_class = result.0;
                trained_model=result.1;
                println!("Selected class: ");
                selected_class.t_print();
                //print_prob(trained_model.1.clone());

            },
            Err(str) => {
                println!("{}",str);
            },
        }

       //break;
    }
}
