use std::collections::HashMap;
pub mod helper_function;
use helper_function::*;
use triadic_rust::{
    data_structures::{
        t_row::{RDataType, TRow},
        t_table::Ttable,
    },
    data_types::{t_f32::TF32, t_string::TString, triadic::Triadic, triadic_type::Ttypes},
    operators::triadic_op::TriadicArithmeticOp,
    t_enum::Degree,
    t_print::Print,
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
    trained_model: (
        HashMap<TString, TF32>,
        HashMap<TString, HashMap<TString, TF32>>,
        HashMap<TString, HashMap<TString, i32>>,
        HashMap<TString, i32>,
    ),
    document_label: TString,
) {
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
                        println!("Degree of the document is false.");
                        return;
                    }
                    let str = doc.get_value();
                    //print!("{} ",str);
                    let words: Vec<&str> = str.split_whitespace().collect();
                    for w in words {
                        let w_store = w.to_lowercase();
                        let cleaned_text = remove_non_alphanumeric(&w_store);
                        if stop_words.contains(&cleaned_text) {
                            continue;
                        }
                        let temp = TString::new(cleaned_text, doc.get_degree());
                        doc_words.push(temp);
                    }
                }
                _ => println!("Doc value is not string"),
            }
        }
        None => println!("Model testing: Doc value is empty"),
    }

    let prior_probabilities: HashMap<TString, TF32> = trained_model.0;
    let class_labels: Vec<&TString> = prior_probabilities.keys().collect();
    let mut p_w_c = trained_model.1;
    let mut vocab = trained_model.2;
    let total_words_in_class = trained_model.3;
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
    }
    for c in class_labels.clone() {
        //Since multiplying many small probabilities can lead to underflow (very small numbers that are difficult to represent accurately), we use logarithms
        let mut c_prob = prior_probabilities
            .get(c)
            .expect("Value should not be none");
        //let mut log_prob = c_prob.ln();

        let class_probabilities = p_w_c
            .get(c)
            .expect("Word probabilities of certain class not found");

        let mut t = TF32::default();
        for word in doc_words.clone() {
            if vocab.contains_key(&word) {
                let temp = class_probabilities.get(&word);
                match temp {
                    Some(temp) => {
                        t = c_prob.tmul_1(temp.clone());
                        c_prob = &t;
                        //println!("word:");
                        //word.t_print();
                        //println!("prob: {}",temp);
                    }
                    None => {
                        println!("Word probability cant be calculated while testing: ");
                        word.t_print();
                    }
                }
            } 
        }
        probabilities.insert(c.clone(), c_prob.clone());
    }

    let mut max = 0.0 as f32;
    let mut selected_class = TString::default();
    //println!("Class probabilities:");
    for (k, v) in probabilities.iter() {
        //k.t_print();
        //println!("{} {}",k.get_value(),v);
        if v.get_value() > max {
            max = v.get_value();
            //println!("max value: {}",max);
            selected_class = k.clone();
            //println!("value of k: {}",k.get_value());

            //println!("CLass: ");
            //selected_class.t_print();
        }
    }
    print! {"Selected class: "};
    selected_class.t_print();
}
fn main() {
    let training: String = "training_triadicv3_dataset.csv".to_owned();
    let class_label = TString::new("Class".to_owned(), Triadic::new(Degree::T));
    let document_label = TString::new("Sentence".to_owned(), Triadic::new(Degree::T));

    let data = read_data(training, class_label.clone());

    let trained_model = model_traning(&data, class_label.clone(), document_label.clone());

    let testing: String = "testing_triadicv3_dataset.csv".to_owned();
    let testing_data = read_data(testing, class_label.clone());

    let size = testing_data.get_size();
    for i in 0..size {
        model_testing(
            testing_data.get_row(i as i32),
            trained_model.clone(),
            document_label.clone(),
        );
        break;
    }
}
