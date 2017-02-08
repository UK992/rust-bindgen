/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


/**
 * <div rustbindgen opaque></div>
 */
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct D {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_D() {
    assert_eq!(::std::mem::size_of::<D>() , 4usize , concat ! (
               "Size of: " , stringify ! ( D ) ));
    assert_eq! (::std::mem::align_of::<D>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( D ) ));
}
impl Clone for D {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct NotAnnotated {
    pub f: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NotAnnotated() {
    assert_eq!(::std::mem::size_of::<NotAnnotated>() , 4usize , concat ! (
               "Size of: " , stringify ! ( NotAnnotated ) ));
    assert_eq! (::std::mem::align_of::<NotAnnotated>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( NotAnnotated ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const NotAnnotated ) ) . f as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( NotAnnotated ) , "::" ,
                stringify ! ( f ) ));
}
impl Clone for NotAnnotated {
    fn clone(&self) -> Self { *self }
}
