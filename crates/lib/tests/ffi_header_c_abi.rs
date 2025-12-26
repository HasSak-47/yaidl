use anyhow::{Result, anyhow, bail};

use clang::{Clang, Entity, EntityKind, Index, Type, TypeKind};

/// Valid if complete OR a pointer
fn is_invalid_ffi_type(ty: &Type, is_param: bool) -> bool {
    let canon = ty.get_canonical_type();

    // Pointers are always allowed
    if canon.get_kind() == TypeKind::Pointer {
        return false;
    }

    match canon.get_kind() {
        // Incomplete arrays allowed ONLY for parameters
        TypeKind::IncompleteArray if is_param => false,

        TypeKind::IncompleteArray => true,

        TypeKind::Record => canon
            .get_declaration()
            .map(|decl| !decl.is_definition())
            .unwrap_or(true),

        TypeKind::Elaborated => canon
            .get_declaration()
            .and_then(|decl| decl.get_type())
            .map(|t| is_invalid_ffi_type(&t, is_param))
            .unwrap_or(true),
        _ => false,
    }
}

fn entity_location(e: &Entity) -> String {
    if let Some(loc) = e.get_location() {
        let floc = loc.get_file_location();
        return format!(
            "{}:{}",
            floc.file
                .and_then(|p| Some(p.get_path().display().to_string()))
                .unwrap_or(String::from("<unknown file>")),
            floc.line
        );
    }
    "<unknown location>".into()
}

#[test]
fn ffi_header_type_is_complete_or_pointer() -> Result<()> {
    let clang = Clang::new().map_err(|e| anyhow!(e))?;
    let index = Index::new(&clang, true, true);
    let tu = index.parser("include/yaidl_ffi.h").parse()?;

    for f in tu
        .get_entity()
        .get_children()
        .into_iter()
        .filter(|e| e.get_kind() == EntityKind::FunctionDecl)
    {
        let loc = entity_location(&f);
        let fn_ty = f.get_type().unwrap();

        // Return type
        let ret = fn_ty.get_result_type().unwrap();
        if is_invalid_ffi_type(&ret, false) {
            bail!(
                "Invalid return type in {}: {:?} @ {loc}",
                f.get_name().unwrap_or("<anon>".into()),
                ret.get_canonical_type().get_display_name()
            );
        }

        // Argument types
        if let Some(args) = fn_ty.get_argument_types() {
            for (i, arg) in args.iter().enumerate() {
                if is_invalid_ffi_type(arg, true) {
                    bail!(
                        "Invalid arg {i} in {}: {:?} @ {loc}",
                        f.get_name().unwrap_or("<anon>".into()),
                        arg.get_canonical_type().get_display_name()
                    );
                }
            }
        }
    }

    Ok(())
}
