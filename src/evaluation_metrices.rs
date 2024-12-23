use crate::{data_structures::t_row::RDataType, data_types::{t_f32::TF32, triadic::Triadic, triadic_type::Ttypes}, operators::triadic_op::TriadicArithmeticOp, t_enum::Degree};

pub fn mean_absolute_error(actual:Vec<RDataType>,predicted:Vec<(TF32,bool)>)-> TF32{
    let n = TF32::new(actual.len() as f32, Triadic::new(Degree::T));
    let mut sum= TF32::new(0.0, Triadic::new(Degree::T));
    for (index,value) in actual.iter().enumerate(){
        if predicted[index].1 == false{
            continue;
        }
        let actual_val= match value {
            RDataType::Float(tf32) => {
                *tf32
            },
            RDataType::Integer(ti32) =>{
                TF32::new(ti32.get_value() as f32, ti32.get_degree())
            },
            _ =>{
                TF32::default()
            }
        };
        let mut sub = actual_val.tsub_1(predicted[index].0);
        sub.set_value(sub.get_value().abs());
        sum= sum.tplus_1(sub);
    }
    sum.tdiv_1(n)
}

pub fn mean_squared_error(actual:Vec<RDataType>,predicted:Vec<(TF32,bool)>)-> TF32{
    let n = TF32::new(actual.len() as f32, Triadic::new(Degree::T));
    let mut sum= TF32::new(0.0, Triadic::new(Degree::T));
    for (index,value) in actual.iter().enumerate(){
        if predicted[index].1 == false{
            continue;
        }
        let actual_val= match value {
            RDataType::Float(tf32) => {
                *tf32
            },
            RDataType::Integer(ti32) =>{
                TF32::new(ti32.get_value() as f32, ti32.get_degree())
            },
            _ =>{
                TF32::default()
            }
        };
        let mut sub = actual_val.tsub_1(predicted[index].0);
        sub= sub.tmul_1(sub);
        sum= sum.tplus_1(sub);
    }
    sum.tdiv_1(n)
}

pub fn r_square(actual:Vec<RDataType>,predicted:Vec<(TF32,bool)>)-> TF32{
    let n = TF32::new(actual.len() as f32, Triadic::new(Degree::T));
    let mut sum= TF32::new(0.0, Triadic::new(Degree::T));
    let mut actual_mean =TF32::new(0.0, Triadic::new(Degree::T));
    for (index,value) in actual.iter().enumerate(){
        if predicted[index].1 == false{
            continue;
        }
        let actual_val= match value {
            RDataType::Float(tf32) => {
                *tf32
            },
            RDataType::Integer(ti32) =>{
                TF32::new(ti32.get_value() as f32, ti32.get_degree())
            },
            _ =>{
                TF32::default()
            }
        };
        let mut sub = actual_val.tsub_1(predicted[index].0);
        sub= sub.tmul_1(sub);
        sum= sum.tplus_1(sub);
        actual_mean = actual_mean.tplus_1(actual_val);

    }
    actual_mean=actual_mean.tdiv_1(n);
    let numrator = sum.tdiv_1(n);
    let mut denominator = TF32::new(0.0, Triadic::new(Degree::T));
    for value in actual.iter(){
        let actual_val= match value {
            RDataType::Float(tf32) => {
                *tf32
            },
            RDataType::Integer(ti32) =>{
                TF32::new(ti32.get_value() as f32, ti32.get_degree())
            },
            _ =>{
                TF32::default()
            }
        };
        let sub = actual_val.tsub_1(actual_mean);
        denominator = denominator.tplus_1(sub.tmul_1(sub));
    }

    let mut res = numrator.tdiv_1(denominator);
    res.set_value(1.0-res.get_value());
    res
}