use std::{cell:: RefCell, collections::HashMap};
use triadic_rust::{data_types::t_string::TString, t_print::Print};
use std::rc::Rc;

pub struct Node{
    attribute_name: Option<TString>,
    children: RefCell<HashMap<TString, Rc<Node>>>,
    class_label: Option<TString>,
    }
    
    impl Node {
        pub fn new() -> Self{
           Self {
            attribute_name: None,
            children:  RefCell::new(HashMap::new()),
            class_label: None,
           }
        }
    
        pub fn set_attribute(&mut self, name: TString){
            self.attribute_name = Some(name);
        }

        pub fn get_class_label(&self) -> Option<TString>{
            self.class_label.clone()
        }

        pub fn get_attribute(&self) -> Option<TString>{
            self.attribute_name.clone()
        }

        pub fn add_child(&mut self, key: TString, value: Rc<Node>){
            self.children.borrow_mut().insert(key, value);
        }

        pub fn set_class_label(&mut self, label: TString) {
            self.class_label = Some(label);
        }

        pub fn print_children(&self){
           if let Some(name) = self.attribute_name.clone() {
               print!("Attribute Name: ");
               name.t_print();
           }
           else{
            if let Some(class) = self.class_label.clone() {
                println!("Class Label: ");
                class.t_print();
                println!("----------------------------");
                return;
            }
           }
            for (k,v) in self.children.borrow().iter() {
                    print!("Class: ");
                    k.t_print();
                    v.print_children();
            }
        }

        pub fn get_child(&self, key: TString) -> Rc<Node> {
            let binding = self.children.borrow();
            let child = binding.get(&key);
            if let Some(val) = child {
                return val.clone();
            }
            else{
                let new_node = Node::new();
                return new_node.into();
            }
        }

       pub fn to_rc(self) -> Option<Rc<Node>> {
            Some(Rc::new(self)) 
          }
    }