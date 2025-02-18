#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq)]
pub struct nsStyleUnion {
    pub mInt: __BindgenUnionField<::std::os::raw::c_int>,
    pub mFloat: __BindgenUnionField<f32>,
    pub mPointer: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub bindgen_union_field: u64,
}
#[test]
fn bindgen_test_layout_nsStyleUnion() {
    assert_eq!(
        ::std::mem::size_of::<nsStyleUnion>(),
        8usize,
        concat!("Size of: ", stringify!(nsStyleUnion))
    );
    assert_eq!(
        ::std::mem::align_of::<nsStyleUnion>(),
        8usize,
        concat!("Alignment of ", stringify!(nsStyleUnion))
    );
    fn test_field_mInt() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<nsStyleUnion>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mInt) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(nsStyleUnion),
                "::",
                stringify!(mInt)
            )
        );
    }
    test_field_mInt();
    fn test_field_mFloat() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<nsStyleUnion>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mFloat) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(nsStyleUnion),
                "::",
                stringify!(mFloat)
            )
        );
    }
    test_field_mFloat();
    fn test_field_mPointer() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<nsStyleUnion>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mPointer) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(nsStyleUnion),
                "::",
                stringify!(mPointer)
            )
        );
    }
    test_field_mPointer();
}
impl Clone for nsStyleUnion {
    fn clone(&self) -> Self {
        *self
    }
}
