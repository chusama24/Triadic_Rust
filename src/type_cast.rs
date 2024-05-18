use crate::data_types::{t_f32::TF32, t_f64::TF64, t_i128::TI128, t_i16::TI16, t_i32::TI32, t_i64::TI64, t_i8::TI8, t_u128::TU128, t_u16::TU16, t_u32::TU32, t_u64::TU64, t_u8::TU8, triadic_type::Ttypes};

/*
Whenever you create a new dataType, make sure to add its type casting function here. These functions are
fairly simple. The only complex and understanding part is the restriction, 
"where i8: From <<T as Ttypes>::ValType>". To understand this restriction, you first need to know about the
Ttypes Trait(Check triadic_type.rs). In that trait, we have a variable ValType which tells us which 
classical dataType the triadic object belongs to. and "From" trait is provided by Rust itself which 
is used with "into()" function for type casting. What this restriction does is that it only allows those
Triadic dataTypes that can be converted through "into()" by checking its classical dataType(example: i32)
with i8(or whatever conversion function user called).
*/



pub fn convertto_i8<T: Ttypes>(v: T) -> TI8 where i8: From<<T as Ttypes>::ValType>{
    let temp = v.get_value();
    let new_val: i8 = temp.into();
    let obj = TI8::new(new_val, v.get_degree());
    obj
}

pub fn convertto_i16<T: Ttypes>(v: T) -> TI16 where i16: From<<T as Ttypes>::ValType>{
    let temp = v.get_value();
    let new_val: i16 = temp.into();
    let obj = TI16::new(new_val, v.get_degree());
    obj
}

pub fn convertto_i32<T: Ttypes>(v: T) -> TI32 where i32: From<<T as Ttypes>::ValType>{
    let temp = v.get_value();
    let new_val: i32 = temp.into();
    let obj = TI32::new(new_val, v.get_degree());
    obj
}

pub fn convertto_i64<T: Ttypes>(v: T) -> TI64 where i64: From<<T as Ttypes>::ValType>{
    let temp = v.get_value();
    let new_val: i64 = temp.into();
    let obj = TI64::new(new_val, v.get_degree());
    obj
}

pub fn convertto_i128<T: Ttypes>(v: T) -> TI128 where i128: From<<T as Ttypes>::ValType>{
    let temp = v.get_value();
    let new_val: i128 = temp.into();
    let obj = TI128::new(new_val, v.get_degree());
    obj
}


pub fn convertto_u8<T: Ttypes>(v: T) -> TU8 where u8: From<<T as Ttypes>::ValType>{
    let temp = v.get_value();
    let new_val: u8 = temp.into();
    let obj = TU8::new(new_val, v.get_degree());
    obj
}

pub fn convertto_u16<T: Ttypes>(v: T) -> TU16 where u16: From<<T as Ttypes>::ValType>{
    let temp = v.get_value();
    let new_val: u16 = temp.into();
    let obj = TU16::new(new_val, v.get_degree());
    obj
}

pub fn convertto_u32<T: Ttypes>(v: T) -> TU32 where u32: From<<T as Ttypes>::ValType>{
    let temp = v.get_value();
    let new_val: u32 = temp.into();
    let obj = TU32::new(new_val, v.get_degree());
    obj
}

pub fn convertto_u64<T: Ttypes>(v: T) -> TU64 where u64: From<<T as Ttypes>::ValType>{
    let temp = v.get_value();
    let new_val: u64 = temp.into();
    let obj = TU64::new(new_val, v.get_degree());
    obj
}

pub fn convertto_u128<T: Ttypes>(v: T) -> TU128 where u128: From<<T as Ttypes>::ValType>{
    let temp = v.get_value();
    let new_val: u128 = temp.into();
    let obj = TU128::new(new_val, v.get_degree());
    obj
}

pub fn convertto_f32<T: Ttypes>(v: T) -> TF32 
where f32: From<<T as Ttypes>::ValType>
{
    let temp = v.get_value();
    let new_val: f32 = temp.into();
    let obj = TF32::new(new_val, v.get_degree());
    obj
}

pub fn convertto_f64<T: Ttypes>(v: T) -> TF64 
where f64: From<<T as Ttypes>::ValType>
{
    let temp = v.get_value();
    let new_val: f64 = temp.into();
    let obj = TF64::new(new_val, v.get_degree());
    obj
}


