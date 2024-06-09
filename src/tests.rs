
use crate::data_structures::t_array::TArray;
use crate::data_types::t_i32::TI32;
use crate::data_types::t_i8::TI8;
use crate::data_types::t_string::TString;
use crate::data_types::triadic::Triadic;
use crate::t_enum::Degree::*;
use crate::t_print::Print;
use crate::operators::triadic_op::*;
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

#[test]
fn test3(){
  let data: Box<i32> = Box::new(20);
  let t_data: Box<TI32> = Box::new(TI32::new(10, Triadic::new(T)));
  println!("Normal Smart Pointer: {}", *data);
  t_data.t_print();

}

#[test]
fn test4(){
  let mut vec:TArray<TI32> = TArray::default();
  let mut x = TI32::new(23, Triadic::new(T));

  vec.push(x);
  vec.t_print();

  x.set_value(24);

  vec.insert_at(x, 0);
  println!("Testing triadic vectors: "); 
  vec.t_print();

  let a = vec.pop().expect("Vector is Empty!");
a.t_print();

}

#[test]
fn test5(){
  let s = TString::new("hello".to_string(), Triadic::new(T));
  s.t_print();
}


