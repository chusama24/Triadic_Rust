use std::collections::HashMap;

use crate::{
    data_structures::{t_array::TArray, t_row::RDataType},
    data_types::{t_string::TString, triadic::Triadic, triadic_type::Ttypes},
    t_enum::{ConvertToDegree, Degree},
};
use rand::{thread_rng, Rng};

#[allow(unused_assignments)]
pub fn create_confusion_matrix(
    actual: &Vec<RDataType>,
    predicted: &TArray<TString>,
) -> (Vec<Vec<i32>>, Vec<String>) {
    let mut keys = Vec::<String>::new();
    println!("Keys: ");

    for r in actual.iter() {
        match r {
            RDataType::String(v) => {
                if keys.contains(&v.get_value()) {
                    continue;
                }
                println!("{}", v.get_value());
                keys.push(v.get_value());
            }
            _ => {
                println!("Error");
            }
        }
    }
    let size = keys.len() * 2;
    let mut cm: Vec<Vec<i32>> = Vec::with_capacity(size);
    for _ in 0..size {
        cm.push(vec![0; size]);
    }

    let mut i = 0;
    let false_check = Triadic::new(Degree::F);
    for r in actual.iter() {
        match r {
            RDataType::String(r) => {
                let x = predicted.get_value()[i as usize].clone();

                let ar = r.get_value();
                let ar_deg = r.get_degree();

                let pr = x.get_value();
                let pr_deg = x.get_degree();
                // println!("Actual deg: {} ",get_char_from_enum(ar_deg.get_value()));
                //print!("Predicted deg: {}",get_char_from_enum(pr_deg.get_value()));

                if ar_deg == false_check || pr_deg == false_check {
                    i += 1;
                    continue;
                }
                let index_of_ar = keys.iter().position(|word| word.eq(&ar));
                let index_of_pr = keys.iter().position(|word| word.eq(&pr));

                // yes/T yes/L no/T no/L
                //(index of key*2) + degree

                let mut i1 = match index_of_ar {
                    Some(i_ar) => (i_ar as i32) * 2,
                    None => -1 as i32,
                };

                let mut i2 = match index_of_pr {
                    Some(i_pr) => (i_pr as i32) * 2,
                    None => -1 as i32,
                };
                if i1 == -1 || i2 == -1 {
                    i += 1;
                    continue;
                }
                let mut deg_index = match ar_deg.get_value() {
                    Degree::T => 0,
                    Degree::F => -1,
                    Degree::L => 1,
                };

                i1 += deg_index;

                deg_index = match pr_deg.get_value() {
                    Degree::T => 0,
                    Degree::F => -1,
                    Degree::L => 1,
                };

                i2 += deg_index;
                cm[i1 as usize][i2 as usize] += 1;
                i += 1;
            }
            _ => {
                print!("Whatever");
            }
        }
    }
    (cm, keys)
}

fn get_random_value<T>(vector: &[T]) -> Option<&T> {
    if vector.is_empty() {
        return None;
    }

    let index = thread_rng().gen_range(0..vector.len());
    Some(&vector[index])
}
pub fn get_indices(
    total_count: i32,
    limit_indices: HashMap<String, Vec<i32>>,
) -> HashMap<String, Vec<i32>> {
    let mut new = HashMap::<String, Vec<i32>>::new();
    let keys: Vec<&String> = limit_indices.keys().collect();
    let mut count = 0;

    loop {
        if count == total_count {
            break;
        }

        match get_random_value(&keys) {
            Some(key) => match limit_indices.get(*key) {
                Some(val) => match get_random_value(val) {
                    Some(v) => match new.get_mut(*key) {
                        Some(vec) => {
                            vec.push(*v);
                            count += 1;
                        }
                        None => {
                            let mut n = Vec::<i32>::new();
                            n.push(*v);
                            count += 1;
                            new.insert(key.to_string(), n);
                        }
                    },
                    None => continue,
                },
                None => continue,
            },
            None => continue,
        }
    }
    new
}
pub fn resolve_limit(
    mut actual: Vec<RDataType>,
    mut predicted: TArray<TString>,
    d: char,
    percentage: f32,
) -> Result<(Vec<Vec<i32>>, Vec<String>), String> {
    if d.to_ascii_lowercase() != 't' && d.to_ascii_lowercase() != 'f' {
        return Err("Selected degree is not acceptable.".to_string());
    }
    let mut count = 0;
    let deg = Triadic::new(d.enum_convert());
    let deg2 = match d.to_ascii_lowercase() {
        't' => Triadic::new('f'.enum_convert()),
        'f' => Triadic::new('t'.enum_convert()),
        _ => Triadic::new('l'.enum_convert())
    };
    let size = actual.len();
    let mut predict = predicted.get_value();
    let limit = Triadic::default();
    let mut limit_index = HashMap::<String, Vec<i32>>::new();
    let mut actual_count = Vec::<i32>::new();
    let mut predicted_count = Vec::<i32>::new();

    for i in 0..size {
        match actual[i].clone() {
            RDataType::String(a) => {
                if a.get_degree() == limit {
                    actual_count.push(i as i32);
                }
            }
            _ => {
                println!("Actual value is not string");
            }
        }

        if predict[i].get_degree() == limit {
            predicted_count.push(i as i32);
        }
    }
    let total = actual_count.len() + predicted_count.len();
    println!("Total: {}",total);
    limit_index.insert("Actual".to_string(), actual_count);
    limit_index.insert("Predicted".to_string(), predicted_count);

    let required_count = (total as f32 * percentage).ceil();
    println!("required_count: {}",required_count);

    let indices = get_indices(required_count as i32, limit_index);
    match indices.get("Actual") {
        Some(a) => {
            actual_count = a.clone();
        }
        None => {
            actual_count = Vec::new();
        }
    }

    match indices.get("Predicted") {
        Some(a) => {
            predicted_count = a.clone();
        }
        None => {
            predicted_count = Vec::new();
        }
    }

    for i in 0..size {
        match actual[i].clone() {
            RDataType::String(mut a) => {
                if a.get_degree() == limit {
                    if actual_count.contains(&(i as i32)) {
                        a.set_degree(deg.clone());
                    }
                    else {
                        a.set_degree(deg2.clone());
                    }
                    actual[i] = RDataType::String(a);
                } 
            }
            _ => {
                println!("Actual value is not string");
            }
        }

        if predict[i].get_degree() == limit {
            if predicted_count.contains(&(i as i32)){
                predict[i].set_degree(deg.clone());
            }
            else {
                predict[i].set_degree(deg2.clone());
            }
            //count += 1;
        }
    }
    //println!("limit count: {}",count);
    predicted.set_vector(predict.clone());

    return Ok(create_confusion_matrix(&actual, &predicted));
}

pub fn calculate_accuracy(cm: Vec<Vec<i32>>) -> f32 {
    let mut denominator = 0.0;
    let mut numerator = 0 as f32;

    for (i, r) in cm.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            denominator += *c as f32;
            if i == j && i % 2 == 0 {
                numerator += *c as f32;
            }
        }
    }

    let a = numerator / denominator;
    if denominator == 0.0 {
        return 0.0;
    }
    a
}

pub fn calculate_precision(cm: Vec<Vec<i32>>) -> f32 {
    let numerator = cm[0][0] as f32;
    let mut denominator = 0 as f32;
    for r in cm.iter() {
        denominator += r[0] as f32;
    }

    let p = numerator / denominator;
    if denominator == 0.0 {
        return 0.0;
    }
    p
}

pub fn calculate_recall(cm: Vec<Vec<i32>>) -> f32 {
    let numerator = cm[0][0] as f32;
    let mut denominator = 0 as f32;
    let row = &cm[0];

    for r in row {
        denominator += *r as f32;
    }

    let r = numerator / denominator;
    if denominator == 0.0 {
        return 0.0;
    }
    r
}

pub fn print_confusion_matrix(cm: &Vec<Vec<i32>>, labels: Vec<String>) {
    print!("A/P ");
    for l in labels.iter() {
        print!("{}/T {}/L ", l, l);
    }
    println!("");

    for i in 0..cm.len() {
        if i % 2 == 0 {
            print!("{}/T ", labels[i / 2 as usize]);
        } else {
            print!("{}/L ", labels[i / 2 as usize]);
        }
        for j in 0..cm.len() {
            print!("{} ", cm[i][j]);
        }
        println!("");
        println!("--------");
    }
    let a = calculate_accuracy(cm.clone());
    let p = calculate_precision(cm.clone());
    let r = calculate_recall(cm.clone());
    println!("Accuracy: {}", a);
    println!("Precision: {}", p);
    println!("Recall: {}", r);
}
