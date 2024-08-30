use std::{collections::HashMap, io::stdin};

use triadic_rust::{
    data_structures::{t_row::RDataType, t_table::Ttable},
    data_types::{t_f32::TF32, t_string::TString, triadic::Triadic, triadic_type::Ttypes},
    t_enum::Degree,
    t_print::Print,
};

pub fn get_data_points(data: Vec<RDataType>, key: TString) -> Vec<i32> {
    let mut examples = Vec::<i32>::new();
    let mut temp = TString::default();
    for j in 0..data.len() {
        let val = data[j].clone();
        match val {
            RDataType::Integer(ti32) => {
                temp.set_value(ti32.get_value().to_string());
                temp.set_degree(ti32.get_degree());
            }
            RDataType::String(str) => {
                temp = str;
            }
            RDataType::Char(ch) => {
                temp.set_value(ch.get_value().to_string());
                temp.set_degree(ch.get_degree());
            }
            RDataType::Float(tf32) => {
                temp.set_value(tf32.get_value().to_string());
                temp.set_degree(tf32.get_degree());
            }
            _ => {
                temp.set_value("Empty".to_string());
                temp.set_degree(Triadic::default());
            }
        }
        if temp.get_degree().get_value() == key.get_degree().get_value() {
            if temp.get_value().eq(&key.get_value()) {
                examples.push(j as i32);
            }
        }
    }
    examples
}

pub fn remove_non_alphanumeric(text: &str) -> String {
    text.chars().filter(|c| c.is_alphanumeric()).collect()
}

pub fn convert_rdatatype_to_string(src: Vec<RDataType>) -> Vec<TString> {
    let mut new_vec = Vec::<TString>::new();
    for val in src.into_iter() {
        match val {
            RDataType::String(temp) => {
                new_vec.push(temp.clone());
            }
            _ => println!(
                "convert_RDatatype_to_String: Cannot convert because datatype is not string"
            ),
        }
    }
    new_vec
}

pub fn get_vocab(
    data: &Ttable,
    col_name: TString,
    class_label: TString,
) -> (HashMap<TString, HashMap<TString, i32>>, HashMap<TString, i32>) {
    let stop_words = stop_words::get(stop_words::LANGUAGE::English);
    let col = data.get_column_by_name(&col_name);
    //col_name.t_print();
    let mut total_words_in_class = HashMap::<TString, i32>::new();
    let mut words_count: HashMap<TString, HashMap<TString, i32>> = HashMap::new();

    let class_col = data.get_column_by_name(&class_label);
    let class_labels = convert_rdatatype_to_string(class_col);

    for (j, i) in col.iter().enumerate() {
        let label = &class_labels[j];
        match i {
            RDataType::String(line) => {
                let str = line.get_value();
                //print!("{} ",str);
                let words: Vec<&str> = str.split_whitespace().collect();
                for w in words {
                    let w_store = w.to_lowercase();
                    let cleaned_text = remove_non_alphanumeric(&w_store);
                    if stop_words.contains(&cleaned_text) {
                        continue;
                    }
                    if cleaned_text.len() > 0 {
                        let temp = TString::new(cleaned_text.to_lowercase(), line.get_degree());
                        words_count = insert_word_in_vocab(words_count, &temp, label);
                    }
                    //println!("{}",cleaned_text);
                    if total_words_in_class.contains_key(&label) {
                        let val = total_words_in_class.get_mut(&label);
                        match val {
                            Some(val) => {
                                *val += 1;
                            }
                            None => {}
                        }
                    } else {
                        total_words_in_class.insert(label.clone(), 1);
                    }
                }
            }
            _ => print!("get_vocab Data type was not in string"),
        }
    }
    return (words_count, total_words_in_class);
}

pub fn get_no_of_document_containing_words(
    data: &Ttable,
    document_indices: Vec<i32>,
    word: TString,
    document_label: TString,
) -> f32 {
    let stop_words = stop_words::get(stop_words::LANGUAGE::English);
    let mut count = 0.0;
    let true_check = Triadic::new(Degree::T);
    for di in document_indices.iter() {
        let row = data.get_row(*di);
        let text = row.get_value(&document_label);
        match text {
            Some(text) => {
                match text {
                    RDataType::String(text) => {
                        if text.get_degree() != true_check {
                            continue;
                        }

                        let str = text.get_value();
                        //print!("{} ",str);
                        let words: Vec<&str> = str.split_whitespace().collect();
                        for w in words {
                            let w_store = w.to_lowercase();
                            let cleaned_text = remove_non_alphanumeric(&w_store);
                            if stop_words.contains(&cleaned_text) {
                                continue;
                            }

                            if cleaned_text.eq(&word.get_value()) {
                                count = count + 1.0;
                                break;
                            }
                        }
                    }
                    _ => print! {"get_no_of_document_containing_words: Data type not found"},
                }
            }
            None => print! {"get_no_of_document_containing_words:Text not found "},
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
            }
            RDataType::Char(ch) => {
                if ch.get_degree() == deg {
                    count = count + 1;
                }
            }
            _ => println!("get_total_of_document_in_degree: Invalid Class label datatype"),
        }
    }
    count
}

pub fn get_class_labels_without_degree(class_labels: Vec<&TString>) -> Vec<String> {
    let mut labels = Vec::<String>::new();
    for cl in class_labels.iter() {
        if !labels.contains(&cl.get_value()) {
            labels.push(cl.get_value());
        }
    }
    labels
}

pub fn total_words_in_a_class(data: &Ttable, no_of_doc: Vec<i32>, document_label: TString) -> f32 {
    let total_words = 0.0;
    let stop_words = stop_words::get(stop_words::LANGUAGE::English);
    let true_check = Triadic::new(Degree::T);
    for i in no_of_doc {
        let row = data.get_row(i);
        let text = row.get_value(&document_label);
        let mut total_words = 0.0;
        match text {
            Some(text) => match text {
                RDataType::String(text) => {
                    if text.get_degree() != true_check {
                        continue;
                    }

                    let str = text.get_value();
                    let words: Vec<&str> = str.split_whitespace().collect();
                    for w in words {
                        let w_store = w.to_lowercase();
                        let cleaned_text = remove_non_alphanumeric(&w_store);
                        if stop_words.contains(&cleaned_text) {
                            continue;
                        }

                        total_words = total_words + 1.0;
                    }
                }
                _ => print! {"get_no_of_document_containing_words: Data type not found"},
            },
            None => print! {"get_no_of_document_containing_words:Text not found "},
        }
    }
    total_words
}


pub fn check_word_in_vocab(vocab: &HashMap<TString, HashMap<TString, i32>>, word: &TString) -> bool {
    for (_k,val) in vocab.into_iter(){
       let temp: Vec<&TString>= val.keys().collect();
       if temp.contains(&word){
        return true;
       }
    }
    return false;
}


pub fn check_unseen_words(
    doc_words: Vec<TString>,
    mut vocab: HashMap<TString, HashMap<TString, i32>>,
    mut total_words_in_class: HashMap<TString, i32>,
    class_labels: Vec<&TString>,
) -> (bool, HashMap<TString, HashMap<TString, i32>>, HashMap<TString, i32>) {
    let mut require_retraining = false;
    for word in doc_words.iter() {
        if !check_word_in_vocab(&vocab, word) {
            require_retraining = true;
            // let labels = get_class_labels_without_degree(class_labels.clone());
            println!("Following word does not exist in the vocabulary, please assign one of the following labels to it: ");
            word.t_print();
            println!("Labels: ");
            for (i, name) in class_labels.iter().enumerate() {
                println!("{}: ", i);
                name.t_print();
            }
            let mut input = String::new();
            loop {
                stdin().read_line(&mut input).expect("Failed to read line");
                input = input.trim().to_string();

                if let Ok(parsed_number) = input.parse::<i32>() {
                    if parsed_number < 0 || parsed_number >= (class_labels.len() as i32) {
                        println!("Invalid input. Please enter the number in range");
                    } else {
                        let label =class_labels[parsed_number as usize]; 
                        let temp = total_words_in_class
                            .get_mut(label)
                            .expect("Class count should exist in total_words_in_class");
                        *temp += 1;
                      vocab = insert_word_in_vocab(vocab, &word, label);

                        break;
                    }
                } else {
                    println!("Invalid input. Please enter an integer.");
                    input.clear();
                }
            }
    }
    }
    (require_retraining, vocab, total_words_in_class)
}

pub fn insert_word_in_vocab<'a>(mut vocab: HashMap<TString, HashMap<TString, i32>>, word: &'a TString, label: &'a TString) ->  HashMap<TString, HashMap<TString, i32>>{

    let wc = vocab.get_mut(label);
    match wc {
        Some(temp_map) => {
            let val = temp_map.get_mut(&word);
            match val {
                Some(val) => {
                    *val += 1;
                }
                None => {
                    temp_map.insert(word.clone(), 1);
                }
            }
        }
        None => {
            let mut new_map = HashMap::<TString, i32>::new();
            new_map.insert(word.clone(), 1);
            vocab.insert(label.clone(), new_map);
        }
    }
    vocab
}

pub fn calculate_word_probabilities(
    vocab: HashMap<TString, HashMap<TString, i32>>,
    vocab_size: i32,
    total_words_in_class: HashMap<TString, i32>,
) -> HashMap<TString, HashMap<TString, TF32>> {
    type InnerMap = HashMap<TString, TF32>;
    type OuterMap = HashMap<TString, InnerMap>;
    let mut prob_w_c: OuterMap = HashMap::new();
    let false_triadic = Triadic::new(Degree::F);
    for (k, v) in vocab.iter() {
        let mut temp_map: InnerMap = HashMap::new(); 
        let total_words = *total_words_in_class.get(k).expect("calculate_word_probabilities: total words cannot be undefined") as f32;
        for (word, count) in v {
            if word.get_degree() == false_triadic {
                continue;
            }
            let p_w_c = (*count as f32 + 1.0) / (total_words + (vocab_size as f32));
            temp_map.insert(word.clone(), TF32::new(p_w_c, k.get_degree()));
        }
        prob_w_c.insert(k.clone(), temp_map);
        //println!("word: {} ,Count: {},total word {} prob{}",k.get_value(),word_count,total_words,p_w_c);
    }
    prob_w_c
}
