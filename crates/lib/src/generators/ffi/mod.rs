mod definitions_wrapper;
mod endpoint_wrapper;
mod type_wrapper;
mod typeinfo_wrapper;

pub use definitions_wrapper::DefinitionsWrapper;
pub use endpoint_wrapper::{
    EndpointParamWrapper, EndpointWrapper, YaildEndpointMethod, YaildEndpointParamKind,
};
pub use type_wrapper::TypeWrapper;
pub use typeinfo_wrapper::TypeInfoWrapper;

use std::{
    ffi::{c_char, c_void},
    ptr::null,
};

use crate::{
    builder::{Code, ffi::CodeFFI},
    parser::{
        definitions::{Definitons, Generator, TypeInformation},
        endpoint::EndPoint as EndpointDef,
        types::Type as TypeDef,
    },
};

#[repr(C)]
pub struct YaildStringView {
    pub ptr: *const c_char,
    pub len: usize,
}

pub(crate) fn string_view(value: &str) -> YaildStringView {
    YaildStringView {
        ptr: value.as_ptr() as *const c_char,
        len: value.len(),
    }
}

pub(crate) fn string_view_opt(value: Option<&str>) -> YaildStringView {
    match value {
        Some(value) => string_view(value),
        None => YaildStringView {
            ptr: std::ptr::null(),
            len: 0,
        },
    }
}

/// Signature for callbacks that emit type headers (imports, etc).
pub type TypeHeaderSign = Option<extern "C" fn(*const c_void, DefinitionsWrapper) -> CodeFFI>;
/// Signature for callbacks that emit endpoint headers.
pub type EndpointHeaderSign = Option<extern "C" fn(*const c_void, DefinitionsWrapper) -> CodeFFI>;
/// Signature for callbacks that emit type definitions.
pub type TypeSign = Option<
    extern "C" fn(*const c_void, *const c_char, TypeWrapper, c_char, DefinitionsWrapper) -> CodeFFI,
>;
/// Signature for callbacks that emit domain <-> wire translation helpers.
pub type TypeTranslationSign =
    Option<extern "C" fn(*const c_void, c_char, TypeInfoWrapper, DefinitionsWrapper) -> CodeFFI>;
/// Signature for callbacks that emit endpoint functions.
pub type EndpointSign = Option<
    extern "C" fn(*const c_void, *const c_char, EndpointWrapper, DefinitionsWrapper) -> CodeFFI,
>;

/// Trait object adapter that lets native generators be exposed via FFI.
#[repr(C)]
#[allow(dead_code)]
pub struct GeneratorFFI {
    /// Opaque foreign context pointer forwarded to every callback.
    this: *const c_void,
    header_type: TypeHeaderSign,
    header_endpoint: EndpointHeaderSign,
    ty: TypeSign,
    wire_translation: TypeTranslationSign,
    domain_translation: TypeTranslationSign,
    endpoint: EndpointSign,
}

impl GeneratorFFI {
    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    pub extern "C" fn yaidl_generator_new() -> GeneratorFFI {
        return GeneratorFFI {
            this: null(),
            header_type: None,
            header_endpoint: None,
            ty: None,
            wire_translation: None,
            domain_translation: None,
            endpoint: None,
        };
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    pub extern "C" fn yaidl_generator_set_header_type(mut self, t: TypeHeaderSign) -> GeneratorFFI {
        self.header_type = t;
        self
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    pub extern "C" fn yaidl_generator_set_header_endpoint(
        mut self,
        t: EndpointHeaderSign,
    ) -> GeneratorFFI {
        self.header_endpoint = t;
        self
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    pub extern "C" fn yaidl_generator_set_type(mut self, t: TypeSign) -> GeneratorFFI {
        self.ty = t;
        self
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    pub extern "C" fn yaidl_generator_set_wire_translation(
        mut self,
        t: TypeTranslationSign,
    ) -> GeneratorFFI {
        self.wire_translation = t;
        self
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    pub extern "C" fn yaidl_generator_set_domain_translation(
        mut self,
        t: TypeTranslationSign,
    ) -> GeneratorFFI {
        self.domain_translation = t;
        self
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    pub extern "C" fn yaidl_generator_set_endpoint(mut self, t: EndpointSign) -> GeneratorFFI {
        self.endpoint = t;
        self
    }

    #[allow(dead_code)]
    #[unsafe(no_mangle)]
    pub extern "C" fn yaidl_generator_set_this(mut self, t: *const c_void) -> GeneratorFFI {
        self.this = t;
        self
    }
}

impl Generator for GeneratorFFI {
    fn generate_type(&self, name: &str, model: &TypeDef, public: bool, defs: &Definitons) -> Code {
        assert!(!self.this.is_null());
        assert!(self.ty.is_some());
        let model = TypeWrapper {
            defs: model as *const TypeDef,
        };
        let defs = DefinitionsWrapper {
            defs: defs as *const Definitons,
        };
        let codeffi = self.ty.unwrap()(
            self.this,
            name.as_ptr() as *const i8,
            model,
            public as c_char,
            defs,
        );

        match codeffi {
            CodeFFI::CodeBox(code) => *code,
            _ => unreachable!(),
        }
    }
    fn generate_to_wire_translation(
        &self,
        public: bool,
        tyinfo: &TypeInformation,
        defs: &Definitons,
    ) -> Code {
        assert!(!self.this.is_null());
        assert!(self.wire_translation.is_some());
        let model = TypeInfoWrapper {
            defs: tyinfo as *const TypeInformation,
        };
        let defs = DefinitionsWrapper {
            defs: defs as *const Definitons,
        };
        let codeffi = self.wire_translation.unwrap()(self.this, public as c_char, model, defs);

        match codeffi {
            CodeFFI::CodeBox(code) => *code,
            _ => unreachable!(),
        }
    }

    fn generate_to_domain_translation(
        &self,
        public: bool,
        tyinfo: &TypeInformation,
        defs: &Definitons,
    ) -> Code {
        assert!(!self.this.is_null());
        assert!(self.domain_translation.is_some());
        let model = TypeInfoWrapper {
            defs: tyinfo as *const TypeInformation,
        };
        let defs = DefinitionsWrapper {
            defs: defs as *const Definitons,
        };
        let codeffi = self.domain_translation.unwrap()(self.this, public as c_char, model, defs);

        match codeffi {
            CodeFFI::CodeBox(code) => *code,
            _ => unreachable!(),
        }
    }

    fn generate_endpoint(&self, name: &str, endpoint: &EndpointDef, defs: &Definitons) -> Code {
        assert!(!self.this.is_null());
        assert!(self.endpoint.is_some());
        let model = EndpointWrapper {
            defs: endpoint as *const EndpointDef,
        };
        let defs = DefinitionsWrapper {
            defs: defs as *const Definitons,
        };
        let codeffi =
            self.endpoint.unwrap()(self.this, name.as_ptr() as *const c_char, model, defs);

        match codeffi {
            CodeFFI::CodeBox(code) => *code,
            _ => unreachable!(),
        }
    }

    fn generate_type_header(&self, defs: &Definitons) -> Code {
        assert!(!self.this.is_null());
        assert!(self.header_type.is_some());
        let defs = DefinitionsWrapper {
            defs: defs as *const Definitons,
        };
        let codeffi = self.header_type.unwrap()(self.this, defs);

        match codeffi {
            CodeFFI::CodeBox(code) => *code,
            _ => unreachable!(),
        }
    }

    fn generate_endpoint_header(&self, defs: &Definitons) -> Code {
        assert!(!self.this.is_null());
        assert!(self.header_endpoint.is_some());
        let defs = DefinitionsWrapper {
            defs: defs as *const Definitons,
        };
        let codeffi = self.header_endpoint.unwrap()(self.this, defs);

        match codeffi {
            CodeFFI::CodeBox(code) => *code,
            _ => unreachable!(),
        }
    }
}
