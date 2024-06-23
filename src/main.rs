pub mod node;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
use csv::Reader;
use node::Node;
use triadic_rust::data_structures::t_array::TArray;
use triadic_rust::data_types::t_f32::TF32;
use triadic_rust::data_types::t_string::TString;
use triadic_rust::data_types::triadic::Triadic;
use triadic_rust::data_types::triadic_type::Ttypes;
use triadic_rust::t_enum::ConvertToDegree;
use triadic_rust::operators::triadic_op::{ TriadicArithmeticOp, TriadicStringOp};
use triadic_rust::t_print::Print;


fn read_data(file_path: String) -> Result<TArray<TArray<TString>>, std::io::Error>{
        // Open the file
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let t = 'T';
    
        // Create a CSV reader
        let mut csv_reader = Reader::from_reader(reader);
    
        let mut attributes: TArray<TString> = TArray::<TString>::default();
        attributes.set_degree(Triadic::new(t.enum_convert()));
        let headers = csv_reader.headers()?;
        let mut i = 0;
            for header in headers{
                if i % 2 == 1{
                    i += 1;
                    continue;
                }
                i += 1;
                let names= header.split('/');
                let mut name = String::new();
                let mut temp_deg = char::default();
                for (j,w) in names.enumerate(){
                    if j == 0{
                        name = w.to_string();
                    }
                    else{
                        let d = w.split(' ');
                        for (k,w2) in d.enumerate(){
                            if k == 1{
                                break;
                            }
                            temp_deg = w2.to_string().trim().parse().expect("Expected a character");
                        }
                    }
                }
                attributes.push(TString::new(name, Triadic::new(temp_deg.enum_convert())))
            }
    let records = csv_reader.records();
    let mut data: TArray<TArray<TString>> = TArray::<TArray<TString>>::default();
    data.set_degree(Triadic::new(t.enum_convert()));
    
    for _i in 0..attributes.get_vector().len(){
        let mut v = TArray::<TString>::default();
        v.set_degree(Triadic::new(t.enum_convert()));
        data.push(v);
    }

    data.insert_at(attributes, 0);

     for record in records.into_iter(){
        match record {
            Ok(record) => { 
                let mut i = 0;
                let mut name = String::new();
                let mut index = 1 ;
                for r in &record{
                    if i % 2 == 0{
                        name = r.to_string();
                        }
                        else {
                            let deg: char = r.to_string().trim().parse().expect("Expected a character");
                            let t_vec = &mut data.get_value()[index];
                            t_vec.push(TString::new(name.clone(), Triadic::new(deg.enum_convert())));
                            data.insert_at(t_vec.clone(), index);
                            data.remove(index+1);
                            index += 1;
                    }
                    i += 1;
                }
            }
            Err(error) => {
                println!("Error while reading record: {}", error); 
            }
        }
     }    
        Ok(data)
}

fn calculate_entropy(num: TArray<TF32>) -> TF32 {
    let t = 'T';
    let true_triadic = Triadic::new(t.enum_convert());
    let mut res = TF32::new(0.0, true_triadic);
    let mut temp = TArray::<TF32>::default();
    temp.set_degree(true_triadic);
    for n in num.get_value(){
    let v1 = f32::ln(n.get_value())/f32::ln(2.0);
    let log_val1 = TF32::new(v1, n.get_degree());
    temp.push(n.tmul_1(log_val1));
    }
    for val in temp.get_value() {
       res = res.tplus_1(val);
    }
    return res.tmul_1(TF32::new(-1.0,true_triadic));
}

fn calculate_gain(attribute: TArray<TString>, class: TArray<TString>, entropy: TF32, examples: Vec<i32>) -> TF32 {
    // True triadic
    let t = 'T';
    let true_triadic = Triadic::new(t.enum_convert());

let mut values = HashMap::<String, HashMap::<String, i32>>::new();
let c = class.get_value();

let mut i = 0;
for a in attribute.get_value().iter(){
    if examples.contains(&i) == false{
        i += 1;
        continue;
    }
    let temp = a.get_value();
    let c1 = &c[i as usize];

    if let Some(val) = values.get_mut(&temp){
        if let Some(num) = val.get_mut(&c1.get_value()){
            *num += 1;
        }
        else{
            val.insert(c1.get_value(), 1);
        }

    }
    else {
        let mut temp_map = HashMap::<String, i32>::new();
        temp_map.insert(c1.get_value(), 1);
        values.insert(temp, temp_map);
    }
    i += 1;
}

let t1: f32 = examples.len() as f32;
let total_size = TF32::new(t1, true_triadic);

let mut res = TF32::new(0.0, true_triadic);
for (_v,h) in values.clone(){
    let mut tv = 0.0;
    for (_s, n) in h.clone() {
        tv += n as f32;
    }
    let mut temp_values = TArray::<TF32>::default();
    temp_values.set_degree(true_triadic);
    for(_s, n) in h.clone() {
        let x = n as f32 / tv;
        temp_values.push(TF32::new(x,true_triadic));
    }



    let en = calculate_entropy(temp_values);
    let t2 = TF32::new(tv, true_triadic);
    let mut t3 = t2.tdiv_1(total_size);
    t3 = t3.tmul_1(en);
    res = res.tplus_1(t3);
}

return entropy.tsub_1(res);
}

fn choose_feature(data: &TArray<TArray<TString>>, features_checked: Vec<i32>, examples: &Vec<i32>) -> i32{
    let size = data.get_value().len();
    let t = 'T';
let true_triadic = Triadic::new(t.enum_convert());
    let class = &data.get_value()[size-1];
    let size_val = size as i32;
    let mut index = 0 as i32;
    let mut gains = TArray::<TF32>::default();
    gains.set_degree(true_triadic);

    // Calculate Entropy
    let mut label_map = HashMap::<String, i32>::new();
    let mut i = 0;
for c in class.get_value().clone(){
    if examples.contains(&i) == false {
        i += 1;
        continue;
    }
    let temp = c.get_value();
    if let Some(val) = label_map.get_mut(&temp){
    *val += 1;
    }
    else{
        label_map.insert(temp, 1);
    }
    i += 1;
}
let mut temp_values = TArray::<TF32>::default();
for (_k,v) in label_map {
let x = v as f32 / examples.len() as f32;
temp_values.push(TF32::new(x, true_triadic));
}

let entropy = calculate_entropy(temp_values);

    for f in data.get_value(){
        if index == 0 || index == size_val-1 || features_checked.contains(&index){
        index += 1;
            continue;
        }
        let g = calculate_gain(f, class.clone(), entropy, examples.clone());
        gains.push(g);
        index += 1;
    }
    let mut max_index = 0 as i32;
    let mut max: f32 = 0.0; 
    index = 0;
    for g in gains.get_value() {
        if max < g.get_value(){
            max_index = index;
            max = g.get_value();
        }
        index += 1;
    }
return max_index; 
}

fn build_tree(data: TArray<TArray<TString>>) -> Node{
    // Check all attributes' degree
    let t: char = 'T';
    let true_triadic = Triadic::new(t.enum_convert());
    let mut root = Node::new();


    let mut features_checked = Vec::<i32>::new();
    let mut temp = TString::new(" ".to_owned(), true_triadic);
    let attributes = &data.get_value()[0];
    for val in attributes.get_value().iter() {
        temp.set_degree(temp.tand_1(val));
    }
    if temp.get_degree() != true_triadic{
        println!("Degrees of all attributes are not true");
        return root;
    }

    // Check degree of class
    let size = attributes.get_value().len();
    if attributes.get_value()[size-1].get_degree() != true_triadic{
        println!("Degrees of class is not true");
        return root;
    }
    // Check if all examples are in same class
    let mut examples = Vec::<i32>::default();
    for i in 0..data.get_value()[1].get_value().len(){
        examples.push(i as i32);
    }
    let data_size = data.get_value().len();

    let class_labels = &data.get_value()[data_size-1];
        let labels = get_labels_map(class_labels.clone(), &examples);
        if labels.len() == 1{
            for (key,_value) in labels.iter(){
                root.set_class_label(key.clone());
                return root;
            }
        }
        // Check if no features left
        if features_checked.len() == (size-1){
            let mut majority = TString::default();
            let mut max = 0;
            for (key, value) in labels.iter(){
                if *value > max{
                    max = *value;
                    majority = key.clone(); 
                }
            }
            root.set_class_label(majority);
            return root;
        }
        // Choose a feature
       
    let next_feature = choose_feature(&data, features_checked.clone(), &examples);
    features_checked.push(next_feature);
    let t2 = data.get_value()[0].get_vector()[next_feature as usize].clone();
    root.set_attribute(t2);
    let keys = get_keys(data.get_value()[(next_feature+1) as usize].clone());
    for val in keys.get_value() {
            let ex = get_examples(data.get_value()[(next_feature+1) as usize].clone(), val.clone(), examples.clone());
            let new_child = create_new_node(&data, features_checked.clone(), ex.clone(), size);
                if let Some(child) = new_child{
                    root.add_child(val.clone(), child);
                }
    } 
    return root;
}

fn create_new_node(data: &TArray<TArray<TString>>, mut features_checked:Vec<i32>,examples: Vec<i32>, total_features: usize)-> Option<Rc<Node>>{
  let mut new_node = Node::new();
  let labels = get_labels_map(data.get_value()[total_features].clone(), &examples);
    if labels.len() == 1{
        for (k,_v) in labels.iter() {
            new_node.set_class_label(k.clone());
        }
        return new_node.to_rc();
    }

    if features_checked.len() == (total_features){
        let mut majority = TString::default();
        let mut max = 0;
        for (key, value) in labels.iter(){
            if *value > max{
                max = *value;
                majority = key.clone();
            }
        }
        new_node.set_class_label(majority);
        return new_node.to_rc();
    }
    
    let next_feature = choose_feature(data, features_checked.clone(), &examples);
    features_checked.push(next_feature);
    let t2 = data.get_value()[0].get_vector()[next_feature as usize].clone();
    new_node.set_attribute(t2.clone());
    let keys = get_keys(data.get_value()[(next_feature+1) as usize].clone());

    for val in keys.get_value() {   
        let ex = get_examples(data.get_value()[(next_feature+1) as usize].clone(), val.clone(), examples.clone());
         let new_child = create_new_node(&data, features_checked.clone(), ex.clone(), total_features);
         if let Some(child) = new_child{
             new_node.add_child(val.clone(), child);
         }
     }

    new_node.to_rc()
}

fn get_examples(data: TArray<TString>, key: TString, current_examples: Vec<i32>) -> Vec<i32> {
    let mut examples = Vec::<i32>::new();
    let temp = data.get_value();
    for j in current_examples {
        let val = &temp[j as usize];
        if val.get_degree().get_value() == key.get_degree().get_value(){
            if val.get_value().eq(&key.get_value()){
                examples.push(j);
            }
        }
    }
    examples
}

fn get_keys(data: TArray<TString>) -> TArray<TString>{
let mut arr = TArray::<TString>::default();
arr.set_degree(Triadic::new('t'.enum_convert()));
for val in data.get_value() {
    if arr.get_value().contains(&val) == false {
        arr.push(val)
    }
}
arr
}

fn get_labels_map(class: TArray<TString>, examples: &Vec<i32>) -> HashMap<TString, i32> {
    let mut labels = HashMap::<TString, i32>::new();
    for i in examples {
        let label = class.get_value()[*i as usize].clone();
        if let Some(val) = labels.get_mut(&label){
            *val += 1;
        } 
        else{
            labels.insert(label, 1);
        }
    }
    labels
}
  
fn predict(root: &Node, input: HashMap<TString, TString>) -> TString {
    if let Some(attr) = root.get_attribute() {
        if let Some(next) = input.get(&attr){
            let n = next.clone();
            let mut child = root.get_child(n.clone());
            loop{
                if let Some(attr) = child.get_attribute() {
                    if let Some(next) = input.get(&attr){
                        let n = next.clone();
                        child = child.get_child(n);
                    }
                }
                else {
                    if let Some(res) = child.get_class_label(){
                        return res;
                    }
                    else{
                      println!("3");
                      return  TString::default();
                    }
                }
            }
        }
        else {
            println!("2");
            return TString::default();
        }
    }
    else {
        if let Some(res) = root.get_class_label(){
            return res;
        }
        else {
            println!("1");
            return TString::default();
        }
    }
}
fn decision_tree(file_path: String) -> Node{
    let d = read_data(file_path);
    match d {
        Ok(d)=>{
          let root = build_tree(d);
         //  root.print_children();
         return root;
        }
        Err(error)=>{
            println!("Failed to retrieve data: {}",error);
            return Node::new();
        }  
    }
}
fn main(){
    let file_path: String = "triadicv1_decision_tree_training.csv".to_owned();
     let root = decision_tree(file_path);
    let mut test = HashMap::<TString, TString>::new();
    let true_triadic = Triadic::new('t'.enum_convert());
    test.insert(TString::new("Outlook".to_owned(), true_triadic), TString::new("Rain".to_owned(), true_triadic));
    test.insert(TString::new("Wind".to_owned(), true_triadic), TString::new("Weak".to_owned(), true_triadic));
   
   let res = predict(&root, test);
   res.t_print();

}
