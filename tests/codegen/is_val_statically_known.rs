//@ compile-flags: --crate-type=lib -Zmerge-functions=disabled -O

#![feature(core_intrinsics)]

use std::intrinsics::is_val_statically_known;

pub struct A(u32);
pub enum B {
    Ye(u32),
}

#[inline]
pub fn _u32(a: u32) -> i32 {
    if is_val_statically_known(a) { 1 } else { 0 }
}

// CHECK-LABEL: @_u32_true(
#[no_mangle]
pub fn _u32_true() -> i32 {
    // CHECK: ret i32 1
    _u32(1)
}

// CHECK-LABEL: @_u32_false(
#[no_mangle]
pub fn _u32_false(a: u32) -> i32 {
    // CHECK: ret i32 0
    _u32(a)
}

#[inline]
pub fn _bool(b: bool) -> i32 {
    if is_val_statically_known(b) { 3 } else { 2 }
}

// CHECK-LABEL: @_bool_true(
#[no_mangle]
pub fn _bool_true() -> i32 {
    // CHECK: ret i32 3
    _bool(true)
}

// CHECK-LABEL: @_bool_false(
#[no_mangle]
pub fn _bool_false(b: bool) -> i32 {
    // CHECK: ret i32 2
    _bool(b)
}

#[inline]
pub fn _iref(a: &u8) -> i32 {
    if unsafe { is_val_statically_known(a) } { 5 } else { 4 }
}

// CHECK-LABEL: @_iref_borrow(
#[no_mangle]
pub fn _iref_borrow() -> i32 {
    // CHECK: ret i32 4
    _iref(&0)
}

// CHECK-LABEL: @_iref_arg(
#[no_mangle]
pub fn _iref_arg(a: &u8) -> i32 {
    // CHECK: ret i32 4
    _iref(a)
}

#[inline]
pub fn _slice_ref(a: &[u8]) -> i32 {
    if unsafe { is_val_statically_known(a) } { 7 } else { 6 }
}

// CHECK-LABEL: @_slice_ref_borrow(
#[no_mangle]
pub fn _slice_ref_borrow() -> i32 {
    // CHECK: ret i32 6
    _slice_ref(&[0;3])
}

// CHECK-LABEL: @_slice_ref_arg(
#[no_mangle]
pub fn _slice_ref_arg(a: &[u8]) -> i32 {
    // CHECK: ret i32 6
    _slice_ref(a)
}
