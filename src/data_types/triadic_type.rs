use super::triadic::Triadic;

/*
use crate:: and use super:: are used to import different modules(files) in this file. 
crate is used if the targeted module(file) is in the root(src folder) or another folder.
super is used if the targeted module(file) is in the same parent folder.
*/

/*
Ttypes trait is created so that we can restrict our triadic generic functions to our Triadic dataTypes. 
So far, all triadic generic functions only need to read the values, hence only get_value() and get_degree
are kept in this trait.
If the need arises for updating triadic dataTypes' values in triadic generic functions,
 we can add set_value() and get_degree() here.

Note: If we don't restrict the Generic functions to Triadic dataTypes only, we will not be able to
use functions like get_value() and get_degree() in those functions.

Example: All Triadic logical operators are generic and Ttype is used in it. 

This trait needs to defined in every triadic dataType(Check out already implemented Triadic datatype for definition).
ValType refers to the dataType that we are converting from classical to triadic.
*/

pub trait Ttypes: Copy {
    type ValType;
     fn get_value(&self) -> Self::ValType;
     fn get_degree(&self) -> Triadic;
}