use std::ffi::{c_char, CStr};

use crate::parser::definitions::Definitons;

use super::{EndpointWrapper, TypeInfoWrapper};

/// Wrapper around the full `Definitons` map for FFI consumers.
#[repr(C)]
pub struct DefinitionsWrapper {
    pub defs: *const Definitons,
}

#[unsafe(no_mangle)]
extern "C" fn yaild_definitions_has_named_type(
    wrapper: *const DefinitionsWrapper,
    name: *const c_char,
) -> bool {
    unsafe {
        let name = CStr::from_ptr(name).to_str().unwrap();
        (*(*wrapper).defs).get_named_type(name).is_some()
    }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_definitions_get_named_type(
    wrapper: *const DefinitionsWrapper,
    name: *const c_char,
) -> TypeInfoWrapper {
    unsafe {
        let name = CStr::from_ptr(name).to_str().unwrap();
        let defs = &*(*wrapper).defs;
        match defs.get_named_type(name) {
            Some(info) => TypeInfoWrapper {
                defs: info as *const _,
            },
            None => TypeInfoWrapper {
                defs: std::ptr::null(),
            },
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_definitions_has_endpoint(
    wrapper: *const DefinitionsWrapper,
    name: *const c_char,
) -> bool {
    unsafe {
        let name = CStr::from_ptr(name).to_str().unwrap();
        let defs = &*(*wrapper).defs;
        defs.has_endpoint(name)
    }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_definitions_get_endpoint(
    wrapper: *const DefinitionsWrapper,
    name: *const c_char,
) -> EndpointWrapper {
    unsafe {
        let name = CStr::from_ptr(name).to_str().unwrap();
        let defs = &*(*wrapper).defs;
        match defs.get_endpoint(name) {
            Some(info) => EndpointWrapper {
                defs: &info.endpoint as *const _,
            },
            None => EndpointWrapper {
                defs: std::ptr::null(),
            },
        }
    }
}
