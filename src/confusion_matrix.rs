use std::collections::HashMap;

use crate::{
    data_structures::{t_array::TArray, t_row::RDataType},
    data_types::{t_string::TString, triadic::Triadic, triadic_type::Ttypes},
    t_enum::{ConvertToDegree, Degree},
    type_cast::convertto_string,
};
use rand::{thread_rng, Rng};

#[allow(unused_assignments)]
pub fn create_confusion_matrix(
    actual: &Vec<RDataType>,
    predicted: &TArray<TString>,
) -> (Vec<Vec<i32>>, HashMap<String, i32>) {
    let mut key_map = HashMap::<String, i32>::new();
    println!("Keys: ");

    let mut temp_index = 0;
    for r in actual.iter() {
        match r {
            RDataType::String(v) => {
                match key_map.get(&v.get_value()) {
                    Some(_val) => {
                        continue;
                    }
                    None => {
                        key_map.insert(v.get_value(), temp_index);
                        //  println!("{}", v.get_value());
                        temp_index += 1;
                    }
                }
            }
            RDataType::Char(c) => {
                let v = c.get_value().to_string();
                match key_map.get(&v) {
                    Some(_val) => {
                        continue;
                    }
                    None => {
                        key_map.insert(v, temp_index);
                        //  println!("{}", v.get_value());
                        temp_index += 1;
                    }
                }
            }
            RDataType::Bool(b) => {
                let v = b.get_value().to_string();
                match key_map.get(&v) {
                    Some(_val) => {
                        continue;
                    }
                    None => {
                        key_map.insert(v, temp_index);
                        //  println!("{}", v.get_value());
                        temp_index += 1;
                    }
                }
            }
            _ => {
                println!("Error");
            }
        }
    }

    for (k, v) in key_map.iter() {
        println!("Key: {}, Value: {}", k, v);
    }

    //  let size = keys.len() * 2;
    let size = (temp_index as usize) * 3;
    let mut cm: Vec<Vec<i32>> = Vec::with_capacity(size);
    for _ in 0..size {
        cm.push(vec![0; size]);
    }

    let mut i = 0;

    for value in actual.iter() {
        let r = match value {
            RDataType::String(v) => v.clone(),
            RDataType::Char(c) => {
                let v = convertto_string(c.clone());
                v
            }
            RDataType::Bool(b) => {
                let v = TString::new(b.get_value().to_string(), b.get_degree());
                v
            }
            _ => TString::default(),
        };

        let x = predicted.get_value()[i as usize].clone();

        let ar = r.get_value();
        let ar_deg = r.get_degree();

        let pr = x.get_value();
        let pr_deg = x.get_degree();

        let mut i1 = match key_map.get(&ar) {
            Some(v) => *v * 3,
            None => -1,
        };

        let mut i2 = match key_map.get(&pr) {
            Some(v) => *v * 3,
            None => -1,
        };

        if i1 == -1 || i2 == -1 {
            i += 1;
            continue;
        }
        let mut deg_index = match ar_deg.get_value() {
            Degree::T => 0,
            Degree::F => 1,
            Degree::L => 2,
        };

        i1 += deg_index;

        deg_index = match pr_deg.get_value() {
            Degree::T => 0,
            Degree::F => 1,
            Degree::L => 2,
        };

        i2 += deg_index;
        cm[i1 as usize][i2 as usize] += 1;
        i += 1;
    }

    (cm, key_map)
}

fn get_random_value<T>(vector: &[T]) -> Option<(&T, usize)> {
    if vector.is_empty() {
        return None;
    }

    let index = thread_rng().gen_range(0..vector.len());
    Some((&vector[index], index))
}

pub fn get_indices(
    required_count: HashMap<String, f32>,
    mut limit_indices: HashMap<String, Vec<i32>>,
) -> HashMap<String, Vec<i32>> {
    let mut new = HashMap::<String, Vec<i32>>::new();

    let temp_map = limit_indices.clone();
    let keys: Vec<&String> = temp_map.keys().collect();

    for k in keys {
        let mut count = 0;
        match limit_indices.get_mut(k) {
            // Get Vector of the selected k from the hashmap
            Some(selected_vector) => {
                let req = match required_count.get(k) {
                    Some(v) => *v as i32,
                    None => 0,
                };
                loop {
                    if count == req {
                        break;
                    }
                    match get_random_value(&selected_vector.clone()) {
                        // Get random value from the selected vector
                        Some((v, val_index)) => match new.get_mut(k) {
                            // Get the Vector of the selected k from the new hashmap
                            Some(vec) => {
                                // Push
                                vec.push(*v);
                                count += 1;
                                selected_vector.remove(val_index);
                            }
                            None => {
                                let mut n = Vec::<i32>::new();
                                n.push(*v);
                                count += 1;
                                new.insert(k.to_string(), n);
                                selected_vector.remove(val_index);
                            }
                        },
                        None => continue,
                    }
                }
            }
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
) -> Result<(Vec<Vec<i32>>, HashMap<String, i32>, TArray<TString>), String> {
    if d.to_ascii_lowercase() != 't' && d.to_ascii_lowercase() != 'f' {
        return Err("Selected degree is not acceptable.".to_string());
    }
    // let mut count = 0;
    let deg = Triadic::new(d.enum_convert());
    let deg2 = match d.to_ascii_lowercase() {
        't' => Triadic::new('f'.enum_convert()),
        'f' => Triadic::new('t'.enum_convert()),
        _ => Triadic::new('l'.enum_convert()),
    };

    let size = actual.len();
    let mut predict = predicted.get_value();
    let limit = Triadic::default();

    let mut limit_index = HashMap::<String, Vec<i32>>::new();
    let mut limit_count = Vec::<i32>::new();
    let mut predicted_count = Vec::<i32>::new();
    let mut actual_count = Vec::<i32>::new();

    for i in 0..size {
        match actual[i].clone() {
            RDataType::String(a) => {
                if a.get_degree() == limit && predict[i].get_degree() == limit {
                    limit_count.push(i as i32);
                } else if predict[i].get_degree() == limit {
                    predicted_count.push(i as i32);
                } else if a.get_degree() == limit {
                    actual_count.push(i as i32);
                }
            }
            _ => {
                println!("Actual value is not string");
            }
        }
    }

    println!("Total Actual: {}", actual_count.len());
    println!("Total Predicted: {}", predicted_count.len());
    println!("Total Limit: {}", limit_count.len());

    limit_index.insert("Actual".to_string(), actual_count.clone());
    limit_index.insert("Predicted".to_string(), predicted_count.clone());
    limit_index.insert("Limit".to_string(), limit_count.clone());

    let mut required_count = HashMap::<String, f32>::new();
    required_count.insert(
        "Actual".to_string(),
        (actual_count.len() as f32 * percentage).ceil(),
    );
    required_count.insert(
        "Predicted".to_string(),
        (predicted_count.len() as f32 * percentage).ceil(),
    );
    required_count.insert(
        "Limit".to_string(),
        (limit_count.len() as f32 * percentage).ceil(),
    );

    for (k, v) in required_count.clone() {
        print!("{}  {}", k, v);
    }
    let mut indices = HashMap::<String, Vec<i32>>::new();

    if percentage == 1.0 {
        indices = limit_index;
    } else if percentage == 0.0 {
        indices.insert("Actual".to_string(), Vec::new());
        indices.insert("Predicted".to_string(), Vec::new());
        indices.insert("Limit".to_string(), Vec::new());
    } else {
        indices = get_indices(required_count, limit_index);
    }

    match indices.get("Actual") {
        Some(a) => {
            actual_count = a.clone();
            println!("Actual Count in Match Statement: {}", actual_count.len());
        }
        None => {
            actual_count = Vec::new();
        }
    }

    match indices.get("Predicted") {
        Some(a) => {
            predicted_count = a.clone();
            println!(
                "Predicted Count in Match Statement: {}",
                predicted_count.len()
            );
        }
        None => {
            predicted_count = Vec::new();
        }
    }

    match indices.get("Limit") {
        Some(a) => {
            limit_count = a.clone();
            println!(
                "Limit Count in Match Statement: {}",
                limit_count.len()
            );
        }
        None => {
            limit_count = Vec::new();
        }
    }
    println!("Actual count {}", actual_count.len());
    println!("Predicted count {}", predicted_count.len());
    println!("Limit count {}", limit_count.len());

    for i in 0..size {
        match actual[i].clone() {
            RDataType::String(mut a) => {
                if a.get_degree() == limit && predict[i].get_degree() == limit {
                    if limit_count.contains(&(i as i32)) {
                        a.set_degree(deg.clone());
                        predict[i].set_degree(deg.clone());
                    }
                    else{
                        a.set_degree(deg2.clone());
                        predict[i].set_degree(deg2.clone());
                    }
                    actual[i] = RDataType::String(a);
                } 

                else if predict[i].get_degree() == limit {
                    if predicted_count.contains(&(i as i32)) {
                        predict[i].set_degree(deg.clone());
                    } else {
                        predict[i].set_degree(deg2.clone());
                    }
                } 

                else if a.get_degree() == limit {
                    if actual_count.contains(&(i as i32)) {
                        a.set_degree(deg.clone());
                    } else {
                        a.set_degree(deg2.clone());
                    }
                    actual[i] = RDataType::String(a);
                }

            }
            _ => {
                println!("Actual value is not string");
            }
        }
    }
    predicted.set_vector(predict.clone());

    // println!("Actual Size: {}, Predicted Size: {}", actual.len(), predicted.length());
    let (matrix, labels) = create_confusion_matrix(&actual, &predicted);
    return Ok((matrix, labels, predicted));
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

pub fn print_confusion_matrix(cm: &Vec<Vec<i32>>, labels_map: HashMap<String, i32>) {
    let mut labels = Vec::<String>::new();
    let size = labels_map.len();
    for _i in 0..size {
        labels.push("".to_string());
    }
    for (k, v) in labels_map.iter() {
        labels[*v as usize] = k.clone();
    }
    print!("A/P ");
    for l in labels.iter() {
        print!("{}/T {}/F {}/L ", l, l, l);
    }
    println!("");

    for i in 0..cm.len() {
        if i % 3 == 0 {
            print!("{}/T ", labels[i / 3 as usize]);
        } else if i % 3 == 1 {
            print!("{}/F ", labels[i / 3 as usize]);
        } else {
            print!("{}/L ", labels[i / 3 as usize]);
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
