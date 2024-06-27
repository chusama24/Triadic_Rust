/*
This is one of the MOST IMPORTANT file. It contains references of each file in the library.
Make sure you add any module(file) you create, here. Otherwise you won't be able to use that module
in any other file. 
File under #[cfg(test)] is for testing purposes. 
Use tests.rs for testing.

How to add? 
If your file is directly inside src folder, simply pub mod "filename"
If it is inside another folder .i.e. src/folder, then do 
pub mod folder{
    pub mod "filename"
}
Look at already added files for more clarification.

Note: There is no main.rs as this is a library(Don't make one). 
        Please read about main.rs and lib.rs if you haven't already.
*/

pub mod t_enum;
pub mod t_print;
pub mod t_scan;
pub mod type_cast;
pub mod data_types{
    pub mod triadic_type;
    pub mod t_i8;
    pub mod t_i16;
    pub mod t_i32;
    pub mod t_i64;
    pub mod t_i128;
    pub mod t_u8;
    pub mod t_u16;
    pub mod t_u32;
    pub mod t_u64;
    pub mod t_u128;
    pub mod t_f32;
    pub mod t_f64;
    pub mod t_char;
    pub mod t_bool;
    pub mod t_string;
 pub mod triadic;
}
pub mod operators{
    pub mod triadic_op;
    pub mod  op_i8;
    pub mod op_i16;
    pub mod op_i32;
    pub mod op_i64;
    pub mod op_i128;
    pub mod  op_u8;
    pub mod op_u16;
    pub mod op_u32;
    pub mod op_u64;
    pub mod op_u128;
    pub mod op_f32;
    pub mod op_f64;
    pub mod op_tchar;
    pub mod op_string;
}
pub mod data_structures{
    pub mod t_array;
    pub mod t_table;
    pub mod t_row;
}
#[cfg(test)]
pub mod tests;

