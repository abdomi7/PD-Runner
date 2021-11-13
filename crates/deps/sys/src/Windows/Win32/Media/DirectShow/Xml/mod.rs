#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_XMLGraphBuilder: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 464542049, data2: 24511, data3: 4562, data4: [165, 33, 68, 223, 7, 193, 0, 0] };
#[repr(transparent)]
pub struct IXMLGraphBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXMLGraphBuilder {}
impl ::core::clone::Clone for IXMLGraphBuilder {
    fn clone(&self) -> Self {
        *self
    }
}