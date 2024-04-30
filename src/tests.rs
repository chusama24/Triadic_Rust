
use crate::data_types::t_i32::TI32;
use crate::data_types::t_i8::TI8;
use crate::data_types::triadic::Triadic;
use crate::t_enum::Degree::*;
use crate::t_print::Print;
use crate::operators::triadic_op::TriadicOp;
use crate::type_cast::convertto_i32;


/*
use crate:: and use super:: are used to import different modules(files) in this file. 
crate is used if the targeted module(file) is in the root(src folder) or another folder.
super is used if the targeted module(file) is in the same parent folder.
*/


/*
This file is purely for testing only. 

How to use it?

Write #[test] and define your function below it. If you are using visual studio code(which you hopefully are),
you should get "Run Test" between #[test] and your function to run that specific function.

*/

#[test]
fn test1(){
  let t = Triadic::new(T);
let a = TI8::new(2, t);
let b = TI32::new(4, t);
let c = b.tplus_1(convertto_i32(a));
c.t_print();

} 

#[test]
fn test2(){
let a = 45;
let b = 2;
let c = a + b;
println!("Result: {}", c);
}


