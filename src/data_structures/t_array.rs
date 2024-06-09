use crate::data_types::{triadic::Triadic, triadic_type::Ttypes};


#[derive(Clone)]
pub struct TArray<T: Ttypes + Clone>{
    t_vector: Vec<T>,
    degree: Triadic
}

impl<T: Ttypes + Clone> TArray<T>{

   pub fn new(t_vec: Vec<T>, deg: Triadic) -> TArray<T>{
        TArray{t_vector: t_vec, degree: deg}
    }
    
    pub fn set_degree(&mut self, d: Triadic){
        self.degree = d;
    }

    pub fn get_degree(&self)-> Triadic{
        self.degree
    }

    pub fn get_vector(&self) -> Vec<T> {
        self.t_vector.clone()
    }

    pub fn push(&mut self, val: T){
        self.t_vector.push(val);
    }
    
    pub fn pop(&mut self) -> Option<T> {
       self.t_vector.pop()
    }

    pub fn insert_at(&mut self, val: T, index: usize){
        self.t_vector.insert(index, val);
    }

    pub fn remove(&mut self, index: usize)-> T {
        self.t_vector.remove(index)
    }

}

impl<T: Ttypes + Clone> Default for TArray<T>{
    fn default() -> Self {
        Self{t_vector: Vec::new(), degree: Triadic::default()}
    }
}

impl<T: Ttypes + Clone> Ttypes for TArray<T>{
    type ValType = Vec<T>;

    fn get_value(&self) -> Self::ValType {
        self.get_vector()
    }

    fn get_degree(&self) -> Triadic {
        self.get_degree()
    }
}


