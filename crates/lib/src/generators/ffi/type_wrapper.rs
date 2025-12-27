use crate::parser::types::{
    ArrayType, IntoType, LiteralType, MapType, OptionType, PrimitiveType, Repr, StructType,
    Type as TypeDef, UnionMember, UnionType,
};

use super::{YaildStringView, string_view, string_view_opt};

#[repr(C)]
pub struct LiteralWrapper(pub *const LiteralType);
#[repr(C)]
pub struct PrimitiveWrapper(pub *const PrimitiveType);
#[repr(C)]
pub struct ReprWrapper(pub *const Repr);
#[repr(C)]
pub struct OptionalWrapper(pub *const OptionType);
#[repr(C)]
pub struct ArrayWrapper(pub *const ArrayType);
#[repr(C)]
pub struct UnionWrapper(pub *const UnionType);
#[repr(C)]
pub struct StructWrapper(pub *const StructType);
#[repr(C)]
pub struct IntoWrapper(pub *const IntoType);
#[repr(C)]
pub struct MapWrapper(pub *const MapType);
#[repr(C)]
pub struct NamedWrapper(pub *const String);
#[repr(C)]
pub struct UndeterminedWrapper(pub *const String);

/// Exposes `Type` to foreign code without leaking Rust's layout.
#[repr(C)]
pub struct TypeWrapper {
    pub defs: *const crate::parser::types::Type,
}

#[repr(C)]
pub enum YaildUnionKind {
    Untagged = 0,
    Interal = 1,
    External = 2,
}

#[repr(C)]
pub struct UnionMemberWrapper(pub *const UnionMember);

#[repr(C)]
pub struct StructMemberWrapper {
    pub name: *const String,
    pub ty: *const crate::parser::types::Type,
}

macro_rules! type_create_is_and_get {
    ($a: tt(_), $ret: tt, $is: ident, $get: ident) => {
        #[unsafe(no_mangle)]
        extern "C" fn $get(wrapper: *const TypeWrapper) -> $ret {
            unsafe {
                return match &*(*wrapper).defs {
                    TypeDef::$a(a) => $ret(a as *const _),
                    _ => panic!("it is not expected type"),
                };
            }
        }

        #[unsafe(no_mangle)]
        extern "C" fn $is(wrapper: *const TypeWrapper) -> bool {
            unsafe {
                return match &*(*wrapper).defs {
                    TypeDef::$a(_) => true,
                    _ => false,
                };
            }
        }
    };
    ($a: tt, $is: ident) => {
        #[unsafe(no_mangle)]
        extern "C" fn $is(wrapper: *const TypeWrapper) -> bool {
            unsafe {
                return match &*(*wrapper).defs {
                    TypeDef::$a => true,
                    _ => false,
                };
            }
        }
    };
}
type_create_is_and_get!(Null, null);
type_create_is_and_get!(
    Literal(_),
    LiteralWrapper,
    yaild_type_is_literal,
    yaild_type_get_literal
);
type_create_is_and_get!(
    Primitive(_),
    PrimitiveWrapper,
    yaild_type_is_primitive,
    yaild_type_get_primitive
);
type_create_is_and_get!(
    Repr(_),
    ReprWrapper,
    yaild_type_is_repr,
    yaild_type_get_repr
);
type_create_is_and_get!(
    Optional(_),
    OptionalWrapper,
    yaild_type_is_optional,
    yaild_type_get_optional
);
type_create_is_and_get!(
    Array(_),
    ArrayWrapper,
    yaild_type_is_array,
    yaild_type_get_array
);
type_create_is_and_get!(
    Union(_),
    UnionWrapper,
    yaild_type_is_union,
    yaild_type_get_union
);
type_create_is_and_get!(
    Struct(_),
    StructWrapper,
    yaild_type_is_struct,
    yaild_type_get_struct
);
type_create_is_and_get!(
    Into(_),
    IntoWrapper,
    yaild_type_is_into,
    yaild_type_get_into
);
type_create_is_and_get!(Map(_), MapWrapper, yaild_type_is_map, yaild_type_get_map);
type_create_is_and_get!(
    Named(_),
    NamedWrapper,
    yaild_type_is_named,
    yaild_type_get_named
);
type_create_is_and_get!(
    Undetermined(_),
    UndeterminedWrapper,
    yaild_type_is_undetermined,
    yaild_type_get_undetermined
);

macro_rules! literal_create_is_and_get {
    ($a: tt($b: tt), $ret: tt, $val: tt, $is: ident, $get: ident) => {
        #[unsafe(no_mangle)]
        extern "C" fn $get(wrapper: *const LiteralWrapper) -> $ret {
            unsafe {
                return match &*(*wrapper).0 {
                    LiteralType::$a($b) => $val,
                    _ => panic!("it is not expected type"),
                };
            }
        }

        #[unsafe(no_mangle)]
        extern "C" fn $is(wrapper: *const LiteralWrapper) -> bool {
            unsafe {
                return match &*(*wrapper).0 {
                    LiteralType::$a(_) => true,
                    _ => false,
                };
            }
        }
    };
}

literal_create_is_and_get!(
    String(s),
    YaildStringView,
    { string_view(s.as_str()) },
    yaild_literal_is_string,
    yaild_literal_get_string
);

literal_create_is_and_get!(
    Uint(v),
    u64,
    { *v },
    yaild_literal_is_uint,
    yaild_literal_get_uint
);

literal_create_is_and_get!(
    Int(v),
    i64,
    { *v },
    yaild_literal_is_int,
    yaild_literal_get_int
);

literal_create_is_and_get!(
    Float(v),
    f64,
    { *v },
    yaild_literal_is_float,
    yaild_literal_get_float
);

literal_create_is_and_get!(
    Bool(v),
    bool,
    { *v },
    yaild_literal_is_bool,
    yaild_literal_get_bool
);

macro_rules! primitve_create_is_and_get {
    ($a: tt, $is: ident) => {
        #[unsafe(no_mangle)]
        extern "C" fn $is(wrapper: *const PrimitiveWrapper) -> bool {
            unsafe {
                return match &*(*wrapper).0 {
                    PrimitiveType::$a => true,
                    _ => false,
                };
            }
        }
    };
    ($a: tt(o), $is: ident, $has_prec: ident, $get_prec: ident) => {
        #[unsafe(no_mangle)]
        extern "C" fn $has_prec(wrapper: *const PrimitiveWrapper) -> bool {
            unsafe {
                return match &*(*wrapper).0 {
                    PrimitiveType::$a(a) => a.is_some(),
                    _ => panic!("it is not expected type"),
                };
            }
        }

        #[unsafe(no_mangle)]
        extern "C" fn $get_prec(wrapper: *const PrimitiveWrapper) -> usize {
            unsafe {
                return match &*(*wrapper).0 {
                    PrimitiveType::$a(a) => a.unwrap(),
                    _ => panic!("it is not expected type"),
                };
            }
        }

        #[unsafe(no_mangle)]
        extern "C" fn $is(wrapper: *const PrimitiveWrapper) -> bool {
            unsafe {
                return match &*(*wrapper).0 {
                    PrimitiveType::$a(_) => true,
                    _ => false,
                };
            }
        }
    };
}

primitve_create_is_and_get!(Bool, yaild_primitive_is_bool);
primitve_create_is_and_get!(
    Unsigned(o),
    yaild_primitive_is_unsigned,
    yaild_primitive_get_unsigned_len,
    yaild_primitive_has_unsigned_len
);
primitve_create_is_and_get!(
    Integer(o),
    yaild_primitive_is_integer,
    yaild_primitive_get_integer_len,
    yaild_primitive_has_integer_len
);
primitve_create_is_and_get!(
    Float(o),
    yaild_primitive_is_float,
    yaild_primitive_get_float_len,
    yaild_primitive_has_float_len
);
primitve_create_is_and_get!(
    String(o),
    yaild_primitive_is_string,
    yaild_primitive_get_string_len,
    yaild_primitive_has_string_len
);

#[unsafe(no_mangle)]
extern "C" fn yaild_repr_is_datetime(wrapper: *const ReprWrapper) -> bool {
    unsafe { matches!(&*(*wrapper).0, Repr::Datetime) }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_optional_get_type(wrapper: *const OptionalWrapper) -> TypeWrapper {
    unsafe {
        TypeWrapper {
            defs: (*(*wrapper).0).ty.as_ref() as *const TypeDef,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_array_has_len(wrapper: *const ArrayWrapper) -> bool {
    unsafe { (*(*wrapper).0).len.is_some() }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_array_get_len(wrapper: *const ArrayWrapper) -> usize {
    unsafe {
        match (*(*wrapper).0).len {
            Some(len) => len,
            None => panic!("array length is not set"),
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_array_get_type(wrapper: *const ArrayWrapper) -> TypeWrapper {
    unsafe {
        TypeWrapper {
            defs: (*(*wrapper).0).ty.as_ref() as *const TypeDef,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_union_get_kind(wrapper: *const UnionWrapper) -> YaildUnionKind {
    unsafe {
        match &(*(*wrapper).0).kind {
            crate::parser::types::UnionKind::Untagged => YaildUnionKind::Untagged,
            crate::parser::types::UnionKind::Interal => YaildUnionKind::Interal,
            crate::parser::types::UnionKind::External => YaildUnionKind::External,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_union_get_member_len(wrapper: *const UnionWrapper) -> usize {
    unsafe { (*(*wrapper).0).members.len() }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_union_get_member(
    wrapper: *const UnionWrapper,
    index: usize,
) -> UnionMemberWrapper {
    unsafe {
        let members = &(*(*wrapper).0).members;
        if index >= members.len() {
            return UnionMemberWrapper(std::ptr::null());
        }
        UnionMemberWrapper(&members[index] as *const _)
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_union_member_has_tag(wrapper: *const UnionMemberWrapper) -> bool {
    unsafe {
        if (*wrapper).0.is_null() {
            return false;
        }
        (*(*wrapper).0).tag.is_some()
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_union_member_get_tag(wrapper: *const UnionMemberWrapper) -> YaildStringView {
    unsafe {
        if (*wrapper).0.is_null() {
            return YaildStringView {
                ptr: std::ptr::null(),
                len: 0,
            };
        }
        string_view_opt((*(*wrapper).0).tag.as_deref())
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_union_member_get_type(wrapper: *const UnionMemberWrapper) -> TypeWrapper {
    unsafe {
        if (*wrapper).0.is_null() {
            return TypeWrapper {
                defs: std::ptr::null(),
            };
        }
        TypeWrapper {
            defs: &(*(*wrapper).0).ty as *const TypeDef,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_struct_get_member_len(wrapper: *const StructWrapper) -> usize {
    unsafe { (*(*wrapper).0).members.len() }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_struct_get_member(
    wrapper: *const StructWrapper,
    index: usize,
) -> StructMemberWrapper {
    unsafe {
        let members = &(*(*wrapper).0).members;
        if index >= members.len() {
            return StructMemberWrapper {
                name: std::ptr::null(),
                ty: std::ptr::null(),
            };
        }
        StructMemberWrapper {
            name: &members[index].0 as *const _,
            ty: &members[index].1 as *const _,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_struct_member_get_name(wrapper: *const StructMemberWrapper) -> YaildStringView {
    unsafe {
        if (*wrapper).name.is_null() {
            return YaildStringView {
                ptr: std::ptr::null(),
                len: 0,
            };
        }
        string_view((*(*wrapper).name).as_str())
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_struct_member_get_type(wrapper: *const StructMemberWrapper) -> TypeWrapper {
    unsafe {
        if (*wrapper).ty.is_null() {
            return TypeWrapper {
                defs: std::ptr::null(),
            };
        }
        TypeWrapper {
            defs: (*wrapper).ty,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_into_get_from(wrapper: *const IntoWrapper) -> TypeWrapper {
    unsafe {
        TypeWrapper {
            defs: (*(*wrapper).0).from.as_ref() as *const TypeDef,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_into_get_repr(wrapper: *const IntoWrapper) -> ReprWrapper {
    unsafe { ReprWrapper(&(*(*wrapper).0).into as *const Repr) }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_map_get_key(wrapper: *const MapWrapper) -> PrimitiveWrapper {
    unsafe { PrimitiveWrapper(&(*(*wrapper).0).key as *const PrimitiveType) }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_map_get_value(wrapper: *const MapWrapper) -> TypeWrapper {
    unsafe {
        TypeWrapper {
            defs: (*(*wrapper).0).val.as_ref() as *const TypeDef,
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_named_get_value(wrapper: *const NamedWrapper) -> YaildStringView {
    unsafe {
        if (*wrapper).0.is_null() {
            return YaildStringView {
                ptr: std::ptr::null(),
                len: 0,
            };
        }
        string_view((*(*wrapper).0).as_str())
    }
}

#[unsafe(no_mangle)]
extern "C" fn yaild_undetermined_get_value(wrapper: *const UndeterminedWrapper) -> YaildStringView {
    unsafe {
        if (*wrapper).0.is_null() {
            return YaildStringView {
                ptr: std::ptr::null(),
                len: 0,
            };
        }
        string_view((*(*wrapper).0).as_str())
    }
}
