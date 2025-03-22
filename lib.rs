/* automatically generated by rust-bindgen 0.71.1 */

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_CXX23: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type max_align_t = f64;
pub type va_list = *mut ::std::os::raw::c_char;
unsafe extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type __vcrt_bool = bool;
unsafe extern "C" {
    pub fn __security_init_cookie();
}
unsafe extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
unsafe extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize) -> !;
}
unsafe extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rebinding {
    pub name: *const ::std::os::raw::c_char,
    pub replacement: *mut ::std::os::raw::c_void,
    pub replaced: *mut *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rebinding"][::std::mem::size_of::<rebinding>() - 24usize];
    ["Alignment of rebinding"][::std::mem::align_of::<rebinding>() - 8usize];
    [
        "Offset of field: rebinding::name",
    ][::std::mem::offset_of!(rebinding, name) - 0usize];
    [
        "Offset of field: rebinding::replacement",
    ][::std::mem::offset_of!(rebinding, replacement) - 8usize];
    [
        "Offset of field: rebinding::replaced",
    ][::std::mem::offset_of!(rebinding, replaced) - 16usize];
};
unsafe extern "C" {
    pub fn rebind_symbols(
        rebindings: *mut rebinding,
        rebindings_nel: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn rebind_symbols_image(
        header: *mut ::std::os::raw::c_void,
        slide: isize,
        rebindings: *mut rebinding,
        rebindings_nel: usize,
    ) -> ::std::os::raw::c_int;
}
