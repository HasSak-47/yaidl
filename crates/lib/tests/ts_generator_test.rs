use anyhow::Result;
use genlib::{
    generators::ts::{self, ErrorHandling},
    parser::definitions::Definitons,
};
use std::path::PathBuf;

#[test]
fn result_mode_wraps_success_and_error_paths() -> Result<()> {
    let mut defs = Definitons::new();
    defs.load_from_file(PathBuf::from("./tests/unit.yaidl"))?;
    defs.build_definitons();
    let generator = ts::TS {
        error_handling: ErrorHandling::Result,
        ..Default::default()
    };

    let module = defs.build_unified_joint_module(&generator);
    let code = module.collapse_root("\t");

    assert!(
        code.contains("import Result from '@/utils/result'"),
        "missing Result import in result mode:\n{code}"
    );
    assert!(
        code.contains("return Result.Err<Baz[], Error>(new Error(response.statusText));"),
        "error branch did not return Result.Err in Baz endpoint:\n{code}"
    );
    assert!(
        code.contains("return Result.Ok<Baz[], Error>(payload);"),
        "success branch did not wrap payload in Result.Ok for Baz endpoint:\n{code}"
    );

    Ok(())
}
