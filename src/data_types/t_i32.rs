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
 pub struct TI32{
    value: i32,
    deg: Triadic,
}

impl TI32 {
   pub fn new(val: i32, d: Triadic) -> Self{
    TI32{value: val, deg: d}
  }

  pub fn set_value(&mut self, val: i32){
    self.value = val;
  }
  pub fn set_degree(&mut self, d: Triadic){
    self.deg = d;
  }

pub fn copy(&self) -> Self{
   TI32{value: self.value, deg: self.deg}
}

}

impl Default for TI32{
    fn default() -> Self{
        Self{value: 0, deg: Triadic::default()}
    }
}


impl Ttypes for TI32 {
type ValType = i32;
fn get_value(&self) -> Self::ValType {
    self.value
}
  fn get_degree(&self) -> Triadic {
 self.deg
  }

}
