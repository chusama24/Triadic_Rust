use crate::data_structures::t_array::TArray;
use crate::data_types::t_f32::TF32;
use crate::data_types::t_f64::TF64;
use crate::data_types::triadic::Triadic;
use crate::data_types::{t_i32::TI32, t_i8::TI8, t_i16::TI16, t_i128::TI128, t_i64::TI64, triadic_type::Ttypes};
use crate::data_types::{t_u32::TU32, t_u8::TU8, t_u16::TU16, t_u128::TU128, t_u64::TU64};
use crate::t_enum;

/*
use crate:: and use super:: are used to import different modules(files) in this file. 
crate is used if the targeted module(file) is in the root(src folder) or another folder.
super is used if the targeted module(file) is in the same parent folder.
*/


/*
Welcome to Print Trait!
Print functions for all Traidic dataTypes need to be defined here.
*/
pub trait Print{
    fn t_print(&self);
}

impl Print for Triadic{
    fn t_print(&self) {
        println!("Triadic Value: ");
        t_enum::t_print(self.get_value());
    }
}

impl Print for TI8{
    fn t_print(&self) {
        println!("Value: {} -----------> Degree: ", self.get_value());
        let t = self.get_degree();
        t.t_print();
    }
}

impl Print for TI32{
    fn t_print(&self) {
        println!("Value: {} -----------> Degree: ", self.get_value());
        let t = self.get_degree();
        t.t_print();
    }
}

impl Print for TI16{
    fn t_print(&self) {
        println!("Value: {} -----------> Degree: ", self.get_value());
        let t = self.get_degree();
        t.t_print();
    }
}

impl Print for TI64{
    fn t_print(&self) {
        println!("Value: {} -----------> Degree: ", self.get_value());
        let t = self.get_degree();
        t.t_print();
    }
}
impl Print for TI128{
    fn t_print(&self) {
        println!("Value: {} -----------> Degree: ", self.get_value());
        let t = self.get_degree();
        t.t_print();
    }
}


impl Print for TU8{
    fn t_print(&self) {
        println!("Value: {} -----------> Degree: ", self.get_value());
        let t = self.get_degree();
        t.t_print();
    }
}

impl Print for TU32{
    fn t_print(&self) {
        println!("Value: {} -----------> Degree: ", self.get_value());
        let t = self.get_degree();
        t.t_print();
    }
}

impl Print for TU16{
    fn t_print(&self) {
        println!("Value: {} -----------> Degree: ", self.get_value());
        let t = self.get_degree();
        t.t_print();
    }
}

impl Print for TU64{
    fn t_print(&self) {
        println!("Value: {} -----------> Degree: ", self.get_value());
        let t = self.get_degree();
        t.t_print();
    }
}
impl Print for TU128{
    fn t_print(&self) {
        println!("Value: {} -----------> Degree: ", self.get_value());
        let t = self.get_degree();
        t.t_print();
    }
}

impl Print for TF32 {
    fn t_print(&self) {
        println!("Value: {} -----------> Degree: ", self.get_value());
        let t = self.get_degree();
        t.t_print();
    }
}

impl Print for TF64 {
    fn t_print(&self) {
        println!("Value: {} -----------> Degree: ", self.get_value());
        let t = self.get_degree();
        t.t_print();
    }
}

impl<T: Ttypes + Print> Print for TArray<T>{
    fn t_print(&self) {
      for x in self.get_vector().iter(){
        x.t_print()
      }
        let t = self.get_degree();
        t.t_print();
    }
}