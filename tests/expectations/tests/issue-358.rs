/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JS_PersistentRooted<c> {
    pub _base: a,
    pub _phantom_0: ::std::marker::PhantomData<c>,
}
impl <c> Default for JS_PersistentRooted<c> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct a {
    pub b: *mut a,
}
impl Clone for a {
    fn clone(&self) -> Self { *self }
}
impl Default for a {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
