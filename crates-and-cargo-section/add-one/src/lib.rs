//! # my_crate
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = add_one::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(value: i32) -> i32 {
    value + 1
}

//Section commonly used on documentations
//
//Panics: Call situations examples where the code of the API can cause a PANIC.
//---
//Errors: If a function returns a Result, can be helpful document wich kind
//of error it returns, so callers can handle these errors more properly.
//
//Safety: If a function is unsafe, there should be a section explaining
//why the function is unsafe and covering the invariants that the function
//expects callers to uphold.
