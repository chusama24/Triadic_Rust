use crate::data_types::{triadic::Triadic, triadic_type::Ttypes};

/*
use crate:: and use super:: are used to import different modules(files) in this file. 
crate is used if the targeted module(file) is in the root(src folder) or another folder.
super is used if the targeted module(file) is in the same parent folder.
*/


/*
Welcome to Triadic Operators Trait!
This trait needs to be implemented for all Triadic datatypes. 
Look at op_i32.rs(or any other already implemented dataType) to understand the implementation.
*/
pub trait TriadicRelationalOp{

  fn tlt_1(self, v2: Self) -> Self;
   fn tlt_2(self, v2: Self) -> Self;
   fn tlt_3(self, v2: Self) -> Self;
   fn tlt(self, v2: Self) -> Self;

   fn teq_1(self, v2: Self) -> Self;
   fn teq_2(self, v2: Self) -> Self;
   fn teq_3(self, v2: Self) -> Self;
   fn teq(self, v2: Self) -> Self;

   fn tgt_1(self, v2: Self) -> Self;
   fn tgt_2(self, v2: Self) -> Self;
   fn tgt_3(self, v2: Self) -> Self;
   fn tgt(self, v2: Self) -> Self;

   fn tgeq_1(self, v2: Self) -> Self;
   fn tgeq_2(self, v2: Self) -> Self;
   fn tgeq_3(self, v2: Self) -> Self;
   fn tgeq(self, v2: Self) -> Self;

   fn tleq_1(self, v2: Self) -> Self;
   fn tleq_2(self, v2: Self) -> Self;
   fn tleq_3(self, v2: Self) -> Self;
   fn tleq(self, v2: Self) -> Self;

   fn tneq_1(self, v2: Self) -> Self;
   fn tneq_2(self, v2: Self) -> Self;
   fn tneq_3(self, v2: Self) -> Self;
   fn tneq(self, v2: Self) -> Self;
 
}

pub trait TriadicLogicalOp {
  fn tand_1<B: Ttypes + Copy>(self, v2: B) -> Triadic;
  fn tand_2<B: Ttypes + Copy>(self, v2: B) -> Triadic;
 fn tand_3<B: Ttypes + Copy>(self, v2: B) -> Triadic;
  fn tand<B: Ttypes + Copy>(self, v2: B) -> Triadic;

  fn tor_1<B: Ttypes + Copy>(self, v2: B) -> Triadic;
 fn tor_2<B: Ttypes + Copy>(self, v2: B) -> Triadic;
  fn tor_3<B: Ttypes + Copy>(self, v2: B) -> Triadic;
  fn tor<B: Ttypes + Copy>(self, v2: B) -> Triadic;

  fn tnot_1(self) -> Triadic;
  fn tnot_2(self) -> Triadic;
  fn tnot_3(self) -> Triadic;
  fn tnot_4(self) -> Triadic;
}

pub trait TriadicArithmeticOp {
  fn tplus_1(self, v2: Self) -> Self;
  fn tplus_2(self, v2: Self) -> Self;
  fn tplus_3(self, v2: Self) -> Self;
  fn tplus(self, v2: Self) -> Self;

  fn tsub_1(self, v2: Self) -> Self;
  fn tsub_2(self, v2: Self) -> Self;
  fn tsub_3(self, v2: Self) -> Self;
  fn tsub(self, v2: Self) -> Self;

  fn tmul_1(self, v2: Self) -> Self;
  fn tmul_2(self, v2: Self) -> Self;
  fn tmul_3(self, v2: Self) -> Self;
  fn tmul(self, v2: Self) -> Self;

  fn tdiv_1(self, v2: Self) -> Self;
  fn tdiv_2(self, v2: Self) -> Self;
  fn tdiv_3(self, v2: Self) -> Self;
  fn tdiv(self, v2: Self) -> Self;

  fn tmod_1(self, v2: Self) -> Self;
  fn tmod_2(self, v2: Self) -> Self;
  fn tmod_3(self, v2: Self) -> Self;
 fn tmod(self, v2: Self) -> Self;
}


pub trait TriadicStringOp {
  fn tand_1<B: Ttypes + Clone>(&self, v2: &B) -> Triadic;
  fn tand_2<B: Ttypes + Clone>(&self, v2: &B) -> Triadic;
 fn tand_3<B: Ttypes + Clone>(&self, v2: &B) -> Triadic;
  fn tand<B: Ttypes + Clone>(&self, v2: &B) -> Triadic;

  fn tor_1<B: Ttypes + Clone>(&self, v2: &B) -> Triadic;
 fn tor_2<B: Ttypes + Clone>(&self, v2: &B) -> Triadic;
  fn tor_3<B: Ttypes + Clone>(&self, v2: &B) -> Triadic;
  fn tor<B: Ttypes + Clone>(&self, v2: &B) -> Triadic;

  fn tnot_1(&self) -> Triadic;
  fn tnot_2(&self) -> Triadic;
  fn tnot_3(&self) -> Triadic;
  fn tnot_4(&self) -> Triadic;

  fn tplus_1(&self, v2: Self) -> Self;
  fn tplus_2(&self, v2: Self) -> Self;
  fn tplus_3(&self, v2: Self) -> Self;
  fn tplus(&self, v2: Self) -> Self;
}