/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>);
impl <T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl <T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl <T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __IncompleteArrayField<T> { }
#[repr(C, packed)]
#[derive(Debug, Default, Copy)]
pub struct header {
    pub proto: ::std::os::raw::c_char,
    pub size: ::std::os::raw::c_uint,
    pub data: __IncompleteArrayField<::std::os::raw::c_uchar>,
    pub __bindgen_padding_0: [u8; 11usize],
}
#[test]
fn bindgen_test_layout_header() {
    assert_eq!(::std::mem::size_of::<header>() , 16usize , concat ! (
               "Size of: " , stringify ! ( header ) ));
}
impl Clone for header {
    fn clone(&self) -> Self { *self }
}
