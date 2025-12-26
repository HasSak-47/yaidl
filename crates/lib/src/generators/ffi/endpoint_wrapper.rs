use crate::parser::endpoint::{EndPoint, EndPointMethod, EndPointParamKind};
use crate::parser::types::Type as TypeDef;

use super::{string_view, TypeWrapper, YaildStringView};

#[repr(C)]
pub enum YaildEndpointMethod {
    Get = 0,
    Post = 1,
    Put = 2,
    Delete = 3,
}

#[repr(C)]
pub enum YaildEndpointParamKind {
    Body = 0,
    Path = 1,
    Query = 2,
    Unknown = 255,
}

/// Wrapper around `EndPoint` for FFI boundaries.
#[repr(C)]
pub struct EndpointWrapper {
    pub defs: *const EndPoint,
}

#[repr(C)]
pub struct EndpointParamWrapper {
    pub defs: *const (String, TypeDef),
}

#[unsafe(no_mangle)]
extern "C" fn yailde_endpoint_get_method(wrapper: *const EndpointWrapper) -> YaildEndpointMethod {
    unsafe {
        match &(*(*wrapper).defs).method {
            EndPointMethod::GET => YaildEndpointMethod::Get,
            EndPointMethod::POST => YaildEndpointMethod::Post,
            EndPointMethod::PUT => YaildEndpointMethod::Put,
            EndPointMethod::DELETE => YaildEndpointMethod::Delete,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_endpoint_get_method_name(wrapper: *const EndpointWrapper) -> YaildStringView {
    unsafe {
        let method = match &(*(*wrapper).defs).method {
            EndPointMethod::GET => "get",
            EndPointMethod::POST => "post",
            EndPointMethod::PUT => "put",
            EndPointMethod::DELETE => "delete",
        };
        string_view(method)
    }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_endpoint_get_url(wrapper: *const EndpointWrapper) -> YaildStringView {
    unsafe { string_view((*(*wrapper).defs).url.as_str()) }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_endpoint_get_return_type(wrapper: *const EndpointWrapper) -> TypeWrapper {
    unsafe {
        TypeWrapper {
            defs: &(*(*wrapper).defs).return_type as *const TypeDef,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_endpoint_get_param_len(wrapper: *const EndpointWrapper) -> usize {
    unsafe { (*(*wrapper).defs).params.len() }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_endpoint_get_param(
    wrapper: *const EndpointWrapper,
    index: usize,
) -> EndpointParamWrapper {
    unsafe {
        let params = &(*(*wrapper).defs).params;
        if index >= params.len() {
            return EndpointParamWrapper {
                defs: std::ptr::null(),
            };
        }
        EndpointParamWrapper {
            defs: &params[index] as *const _,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_endpoint_param_get_name(
    wrapper: *const EndpointParamWrapper,
) -> YaildStringView {
    unsafe {
        if (*wrapper).defs.is_null() {
            return YaildStringView {
                ptr: std::ptr::null(),
                len: 0,
            };
        }
        string_view((*(*wrapper).defs).0.as_str())
    }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_endpoint_param_get_type(
    wrapper: *const EndpointParamWrapper,
) -> TypeWrapper {
    unsafe {
        if (*wrapper).defs.is_null() {
            return TypeWrapper {
                defs: std::ptr::null(),
            };
        }
        TypeWrapper {
            defs: &(*(*wrapper).defs).1 as *const TypeDef,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yailde_endpoint_param_get_kind(
    endpoint: *const EndpointWrapper,
    wrapper: *const EndpointParamWrapper,
) -> YaildEndpointParamKind {
    unsafe {
        if (*endpoint).defs.is_null() || (*wrapper).defs.is_null() {
            return YaildEndpointParamKind::Unknown;
        }
        let name = &(*(*wrapper).defs).0;
        match (*(*endpoint).defs).get_param_type(name) {
            Some(EndPointParamKind::Body) => YaildEndpointParamKind::Body,
            Some(EndPointParamKind::Path) => YaildEndpointParamKind::Path,
            Some(EndPointParamKind::Query) => YaildEndpointParamKind::Query,
            None => YaildEndpointParamKind::Unknown,
        }
    }
}
