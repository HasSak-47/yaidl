use crate::parser::types::{
    ArrayType, IntoType, LiteralType, MapType, OptionType, PrimitiveType, Repr, StructType,
    Type as TypeDef, UnionType,
};

#[repr(C)]
struct LiteralWrapper(*const LiteralType);
#[repr(C)]
struct PrimitiveWrapper(*const PrimitiveType);
#[repr(C)]
struct ReprWrapper(*const Repr);
#[repr(C)]
struct OptionalWrapper(*const OptionType);
#[repr(C)]
struct ArrayWrapper(*const ArrayType);
#[repr(C)]
struct UnionWrapper(*const UnionType);
#[repr(C)]
struct StructWrapper(*const StructType);
#[repr(C)]
struct IntoWrapper(*const IntoType);
#[repr(C)]
struct MapWrapper(*const MapType);
#[repr(C)]
struct NamedWrapper(*const String);
#[repr(C)]
struct UndeterminedWrapper(*const String);

/// Exposes `Type` to foreign code without leaking Rust's layout.
#[repr(C)]
pub struct TypeWrapper {
    pub defs: *const TypeDef,
}

macro_rules! create_is_and_get {
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
                return match *(*wrapper).defs {
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
                return match *(*wrapper).defs {
                    TypeDef::$a => true,
                    _ => false,
                };
            }
        }
    };
}
create_is_and_get!(Null, null);
create_is_and_get!(
    Literal(_),
    LiteralWrapper,
    yaild_type_is_literal,
    yailde_type_get_literal
);
create_is_and_get!(
    Primitive(_),
    PrimitiveWrapper,
    yaild_type_is_primitive,
    yailde_type_get_primitive
);
create_is_and_get!(
    Repr(_),
    ReprWrapper,
    yaild_type_is_repr,
    yailde_type_get_repr
);
create_is_and_get!(
    Optional(_),
    OptionalWrapper,
    yaild_type_is_optional,
    yailde_type_get_optional
);
create_is_and_get!(
    Array(_),
    ArrayWrapper,
    yaild_type_is_array,
    yailde_type_get_array
);
create_is_and_get!(
    Union(_),
    UnionWrapper,
    yaild_type_is_union,
    yailde_type_get_union
);
create_is_and_get!(
    Struct(_),
    StructWrapper,
    yaild_type_is_struct,
    yailde_type_get_struct
);
create_is_and_get!(
    Into(_),
    IntoWrapper,
    yaild_type_is_into,
    yailde_type_get_into
);
create_is_and_get!(Map(_), MapWrapper, yaild_type_is_map, yailde_type_get_map);
create_is_and_get!(
    Named(_),
    NamedWrapper,
    yaild_type_is_named,
    yailde_type_get_named
);
create_is_and_get!(
    Undetermined(_),
    UndeterminedWrapper,
    yaild_type_is_undetermined,
    yailde_type_get_undetermined
);
