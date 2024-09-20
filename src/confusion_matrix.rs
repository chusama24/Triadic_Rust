use crate::{data_structures::{t_array::TArray, t_row::RDataType}, data_types::{t_string::TString, triadic::Triadic, triadic_type::Ttypes}, t_enum::{ConvertToDegree, Degree}};


#[allow(unused_assignments)]
pub fn create_confusion_matrix(
    actual: &Vec<RDataType>,
    predicted: &TArray<TString>,
) -> Vec<Vec<i32>> {
    let mut keys = Vec::<String>::new();
    //println!("Keys: ");

    for r in actual.iter() {
        match r {
            RDataType::String(v) => {
                if keys.contains(&v.get_value()) {
                    continue;
                }
                //println!("{}", v.get_value());
                keys.push(v.get_value());
            }
            _ => {
                println!("Error");
            }
        }
    }

    keys.sort();
    keys.reverse();

    let size=keys.len()*2;
    let mut cm: Vec<Vec<i32>> = Vec::with_capacity(size);
    for _ in 0..size {
        cm.push(vec![0; size]);
    }

    let mut i = 0;
    let false_check=Triadic::new(Degree::F);
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

                if ar_deg== false_check || pr_deg==false_check{
                    i += 1;
                    continue;
                }
                let index_of_ar = keys.iter().position(|word| word.eq(&ar));
                let index_of_pr = keys.iter().position(|word| word.eq(&pr));
                
                // yes/T yes/L no/T no/L 
                //(index of key*2) + degree

                let mut i1= match index_of_ar {
                    Some(i_ar) => {
                        (i_ar as i32)*2
                    },
                    None => -1 as i32,
                };

                let mut i2= match index_of_pr {
                    Some(i_pr) => {
                        (i_pr as i32)*2
                    },
                    None => -1 as i32,
                };

                let mut deg_index= match ar_deg.get_value() {
                    Degree::T => {
                        0
                    },
                    Degree::F => {
                        -1
                    },
                    Degree::L => {
                        1
                    }
                };
                
                i1+=deg_index;

                deg_index=match pr_deg.get_value() {
                    Degree::T => {
                        0
                    },
                    Degree::F => {
                     -1
                    },
                    Degree::L => {
                        1
                    }
                };
                
                i2+=deg_index;
                cm[i1 as usize][i2 as usize]+=1;
                i += 1;
            }
            _ =>
            {
                print!("Whatever");
            }
        }
    }
    cm 
}

pub fn resolve_limit(
    mut actual: Vec<RDataType>,
    mut predicted: TArray<TString>,
    d: char,
) -> Vec<Vec<i32>> {
    let mut count =0;
    let deg = Triadic::new(d.enum_convert());
    let size = actual.len();
    let mut predict = predicted.get_value();
    let limit = Triadic::default();
    for i in 0..size {
        match actual[i].clone() {
            RDataType::String(mut a) => {

                if a.get_degree() == limit {
                    a.set_degree(deg.clone());
                    actual[i] = RDataType::String(a);
                }
            }
            _ => {
                println!("Actual value is not string");
            }
        }

        
        if predict[i].get_degree() == limit {
            predict[i].set_degree(deg.clone());
            count+=1;
            
        }
    }
    //println!("limit count: {}",count);
    predicted.set_vector(predict.clone());
  
    return create_confusion_matrix(&actual, &predicted);
}

pub fn calculate_accuracy(cm: Vec<Vec<i32>>) -> f32 {
    let mut denominator = 0.0;
    let mut numerator = 0 as f32;

    for (i,r) in cm.iter().enumerate() {
        for (j,c) in r.iter().enumerate(){
            denominator += *c as f32;
            if i==j && i%2==0 {
                numerator+=*c as f32;
            }
        }
    }

    let a = numerator / denominator;
    if denominator==0.0{
        return 0.0;
    }
    a
}

pub fn calculate_precision(cm: Vec<Vec<i32>>) -> f32 {
    let numerator = cm[0][0] as f32;
    let mut denominator= 0 as f32;
    for r in cm.iter(){
        denominator+=r[0] as f32;
    }

    let p = numerator / denominator;
    if denominator==0.0{
        return 0.0;
    }
    p
}

pub fn calculate_recall(cm: Vec<Vec<i32>>) -> f32 {
    let numerator = cm[0][0] as f32;
    let mut denominator= 0 as f32;
    let row = &cm[0];
    
    for r in row{
        denominator+= *r as f32;
    }

    let r = numerator / denominator;
    if denominator==0.0{
        return 0.0;
    }
    r
}

pub fn print_confusion_matrix(cm: &Vec<Vec<i32>>,labels:Vec<String>) {
    print!("A/P ");
    for l in labels.iter(){
        print!("{}/T {}/L ",l,l);
    }
    println!("");

    for i in 0..cm.len() {
        if i%2==0  {
            print!("{}/T ",labels[i/2 as usize]);
        }
        else{
            print!("{}/L ",labels[i/2 as usize]);
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


