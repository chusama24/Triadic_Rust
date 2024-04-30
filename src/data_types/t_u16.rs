use crate::data_types::triadic_type::Ttypes;
use super::triadic::Triadic;

/*
use crate:: and use super:: are used to import different modules(files) in this file. 
crate is used if the targeted module(file) is in the root(src folder) or another folder.
super is used if the targeted module(file) is in the same parent folder.
*/

/*
 There are macros in Rust for Clone, Copy, etc. If we want to use functionalities with our struct, we need
 to use #[derive()]. if we do not define it in implementation(Granted that we have added it using 
 derive keyword), then Rust's default macro will be used.

 Note: impl == implementation

 For Default constructor, we need to implement Default separately as done below.

 Note: I haven't been able to create a Default constructor with parameterized values yet
 (Please give it a try and let me know if you get some success in it)
*/

#[derive(Clone, Copy)]
 pub struct TU16{
    value: u16,
    deg: Triadic,
}

impl TU16 {
   pub fn new(val: u16, d: Triadic) -> Self{
    TU16{value: val, deg: d}
  }

  pub fn set_value(&mut self, val: u16){
    self.value = val;
  }
  pub fn set_degree(&mut self, d: Triadic){
    self.deg = d;
  }

pub fn copy(&self) -> Self{
   TU16{value: self.value, deg: self.deg}
}

}

impl Default for TU16{
    fn default() -> Self{
        Self{value: 0, deg: Triadic::default()}
    }
}


impl Ttypes for TU16 {
type ValType = u16;
fn get_value(&self) -> Self::ValType {
    self.value
}
  fn get_degree(&self) -> Triadic {
 self.deg
  }

}
