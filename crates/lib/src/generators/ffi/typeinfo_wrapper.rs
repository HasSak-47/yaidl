use crate::parser::definitions::TypeInformation;
use crate::parser::types::Type as TypeDef;

use super::{string_view, string_view_opt, TypeWrapper, YaildStringView};

/// Wrapper around `TypeInformation` for FFI boundaries.
#[repr(C)]
pub struct TypeInfoWrapper {
    pub defs: *const TypeInformation,
}

#[unsafe(no_mangle)]
extern "C" fn yaild_typeinfo_has_name(wrapper: *const TypeInfoWrapper) -> bool {
    unsafe { (*(*wrapper).defs).name.is_some() }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_typeinfo_get_name(wrapper: *const TypeInfoWrapper) -> YaildStringView {
    unsafe { string_view_opt((*(*wrapper).defs).name.as_deref()) }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_typeinfo_get_type(wrapper: *const TypeInfoWrapper) -> TypeWrapper {
    unsafe {
        TypeWrapper {
            defs: &(*(*wrapper).defs).ty as *const TypeDef,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_typeinfo_get_domain_type(wrapper: *const TypeInfoWrapper) -> TypeWrapper {
    unsafe {
        TypeWrapper {
            defs: (*(*wrapper).defs).get_domain_type() as *const TypeDef,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_typeinfo_get_wire_type(wrapper: *const TypeInfoWrapper) -> TypeWrapper {
    unsafe {
        TypeWrapper {
            defs: (*(*wrapper).defs).get_wire_type() as *const TypeDef,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_typeinfo_has_conversion(wrapper: *const TypeInfoWrapper) -> bool {
    unsafe { (*(*wrapper).defs).has_conversion() }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_typeinfo_get_wire_name(wrapper: *const TypeInfoWrapper) -> YaildStringView {
    unsafe { string_view_opt((*(*wrapper).defs).get_wire_name().as_deref()) }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_typeinfo_get_line(wrapper: *const TypeInfoWrapper) -> usize {
    unsafe { (*(*wrapper).defs).line }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_typeinfo_get_col(wrapper: *const TypeInfoWrapper) -> usize {
    unsafe { (*(*wrapper).defs).col }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_typeinfo_get_path(wrapper: *const TypeInfoWrapper) -> YaildStringView {
    unsafe {
        let path = (*(*wrapper).defs).path.to_str();
        match path {
            Some(path) => string_view(path),
            None => YaildStringView {
                ptr: std::ptr::null(),
                len: 0,
            },
        }
    }
}
