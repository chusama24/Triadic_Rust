use crate::data_types::{t_i32::TI32, t_i8::TI8, t_i16::TI16, t_i128::TI128, t_i64::TI64};
use crate::data_types::{t_u32::TU32, t_u8::TU8, t_u16::TU16, t_u128::TU128, t_u64::TU64};
use crate::data_types::triadic::Triadic;
use crate::t_enum;
use t_enum::Degree;
use std::io;

/*
use crate:: and use super:: are used to import different modules(files) in this file. 
crate is used if the targeted module(file) is in the root(src folder) or another folder.
super is used if the targeted module(file) is in the same parent folder.
*/


/*
Welcome to Scan Trait!
Scan functions for all Traidic dataTypes need to be defined here.
*/

pub trait Scan{
    fn t_scan(&mut self);
}

impl Scan for Triadic {
fn t_scan(&mut self) {
    let mut d:Degree = Degree::L;
    t_enum::t_scan(&mut d);
    self.set_value(d);
}
}

impl Scan for TI8{
    fn t_scan(&mut self) {

        let mut d = Triadic::default();
      println!("Value: ");
/*
Rust always take input in String type, So we take input in String variable and then trim any white spaces
and then parse into the required dataType in this case, i32 by using "let num: i32"
*/
      let mut val = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");

    let num: i8 = val.trim().parse()
        .expect("Please enter a valid integer");
    self.set_value(num);

    d.t_scan();
        
    }
}

impl Scan for TI16{
    fn t_scan(&mut self) {

        let mut d = Triadic::default();
      println!("Value: ");
/*
Rust always take input in String type, So we take input in String variable and then trim any white spaces
and then parse into the required dataType in this case, i32 by using "let num: i32"
*/
      let mut val = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");

    let num: i16 = val.trim().parse()
        .expect("Please enter a valid integer");
    self.set_value(num);

    d.t_scan();
        
    }
}

impl Scan for TI32{
    fn t_scan(&mut self) {

        let mut d = Triadic::default();
      println!("Value: ");
/*
Rust always take input in String type, So we take input in String variable and then trim any white spaces
and then parse into the required dataType in this case, i32 by using "let num: i32"
*/
      let mut val = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");

    let num: i32 = val.trim().parse()
        .expect("Please enter a valid integer");
    self.set_value(num);

    d.t_scan();
        
    }
}

impl Scan for TI64{
    fn t_scan(&mut self) {

        let mut d = Triadic::default();
      println!("Value: ");
/*
Rust always take input in String type, So we take input in String variable and then trim any white spaces
and then parse into the required dataType in this case, i32 by using "let num: i32"
*/
      let mut val = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");

    let num: i64 = val.trim().parse()
        .expect("Please enter a valid integer");
    self.set_value(num);

    d.t_scan();
        
    }
}

impl Scan for TI128{
    fn t_scan(&mut self) {

        let mut d = Triadic::default();
      println!("Value: ");
/*
Rust always take input in String type, So we take input in String variable and then trim any white spaces
and then parse into the required dataType in this case, i32 by using "let num: i32"
*/
      let mut val = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");

    let num: i128 = val.trim().parse()
        .expect("Please enter a valid integer");
    self.set_value(num);

    d.t_scan();
        
    }
}


impl Scan for TU8{
    fn t_scan(&mut self) {

        let mut d = Triadic::default();
      println!("Value: ");
/*
Rust always take input in String type, So we take input in String variable and then trim any white spaces
and then parse into the required dataType in this case, i32 by using "let num: i32"
*/
      let mut val = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");

    let num: u8 = val.trim().parse()
        .expect("Please enter a valid integer");
    self.set_value(num);

    d.t_scan();
        
    }
}

impl Scan for TU16{
    fn t_scan(&mut self) {

        let mut d = Triadic::default();
      println!("Value: ");
/*
Rust always take input in String type, So we take input in String variable and then trim any white spaces
and then parse into the required dataType in this case, i32 by using "let num: i32"
*/
      let mut val = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");

    let num: u16 = val.trim().parse()
        .expect("Please enter a valid integer");
    self.set_value(num);

    d.t_scan();
        
    }
}

impl Scan for TU32{
    fn t_scan(&mut self) {

        let mut d = Triadic::default();
      println!("Value: ");
/*
Rust always take input in String type, So we take input in String variable and then trim any white spaces
and then parse into the required dataType in this case, i32 by using "let num: i32"
*/
      let mut val = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");

    let num: u32 = val.trim().parse()
        .expect("Please enter a valid integer");
    self.set_value(num);

    d.t_scan();
        
    }
}

impl Scan for TU64{
    fn t_scan(&mut self) {

        let mut d = Triadic::default();
      println!("Value: ");
/*
Rust always take input in String type, So we take input in String variable and then trim any white spaces
and then parse into the required dataType in this case, i32 by using "let num: i32"
*/
      let mut val = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");

    let num: u64 = val.trim().parse()
        .expect("Please enter a valid integer");
    self.set_value(num);

    d.t_scan();
        
    }
}

impl Scan for TU128{
    fn t_scan(&mut self) {

        let mut d = Triadic::default();
      println!("Value: ");
/*
Rust always take input in String type, So we take input in String variable and then trim any white spaces
and then parse into the required dataType in this case, i32 by using "let num: i32"
*/
      let mut val = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");

    let num: u128 = val.trim().parse()
        .expect("Please enter a valid integer");
    self.set_value(num);

    d.t_scan();
        
    }
}