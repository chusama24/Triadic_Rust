use crate::t_enum::Degree;

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
#[derive(PartialEq, Eq, Hash)]
pub struct Triadic {
   val: Degree
}

impl Triadic {
    pub fn new(d: Degree) -> Self{
        Triadic{val: d}
      }

      pub fn set_value(&mut self, d: Degree){
        self.val = d;
      }

     pub fn get_value(&self) -> Degree {
        self.val
    }

      pub fn copy(&self) -> Self{
        Triadic{val: self.val}
     }
}

impl Default for Triadic{
    fn default() -> Self{
        Self{val: Degree::L}
    }
}
/*
impl Ttypes for Triadic {
    type ValType = Degree;
    fn get_value(&self) -> Self::ValType {
        self.val
    }
}
*/