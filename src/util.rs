// util

use crate::parse::*;

pub fn first_char(s: &str) -> char {
    if s.len() == 0 {
        return '\0';
    }
    return char::from((&s[0..1].as_bytes())[0]);
}

pub fn roundup(x: i32, align: i32) -> i32 {
    return (x + align - 1) & (!(align - 1));
}

pub fn ptr_to(base: Type) -> Type {
    let mut ty = alloc_type();
    ty.ty = CType::PTR;
    ty.size = 8;
    ty.align = 8;
    ty.ptr_to = Some(Box::new(base));
    return ty;
}

pub fn ary_of(base: Type, len: i32) -> Type {
    let mut ty = alloc_type();
    ty.ty = CType::ARY;
    ty.size = base.size * len;
    ty.align = base.align;
    ty.ary_of = Some(Box::new(base));
    ty.len = len;
    return ty;
}

