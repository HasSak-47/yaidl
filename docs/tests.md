# Tests Guide

This document explains what the current tests exercise and why they exist.

## Running tests

- Run everything: `cargo test --workspace`
- Focus the library crate: `cargo test -p lib`
- Parser-centric loop: `cargo test -p lib parse_test`

The `ffi_header_c_abi` test requires `libclang` to be installed and discoverable.
If libclang is not on your system path, set `LIBCLANG_PATH` before running the suite.

## Shared fixture

`crates/lib/tests/unit.yaidl` is the main DSL fixture. It mixes:

- Scalar fields, optional fields, and arrays.
- A union of named types (`FooBar = outer { Foo | Bar }`).
- Struct nesting with `datetime` conversions.
- Endpoints that cover GET, POST, PUT, and DELETE with path, query, and body bindings.
- A tuple response (`<string, Entry[]>`) to exercise generator edge cases.

## Test files and intent

- `crates/lib/tests/parse_test.rs`
  - Loads `unit.yaidl`, builds definitions, and generates a unified TypeScript module.
  - Writes the output to `temp/generated.ts` to ensure the builder emits usable code.
- `crates/lib/tests/ts_generator_test.rs`
  - Verifies TypeScript `result` error-handling mode wraps success and error branches.
  - Asserts the `Result` import and the `Result.Ok`/`Result.Err` calls are present.
- `crates/lib/tests/conversion_helpers.rs`
  - Ensures nested named types emit both `into_domain_*` and `into_wire_*` helpers.
- `crates/lib/tests/ffi_header_c_abi.rs`
  - Parses `crates/lib/include/yaidl_ffi.h` with clang and validates that function
    parameters and returns are either pointers or complete types.
  - Guards against ABI regressions that would break consumers of the FFI shim.

## Notes

- The `parse_test` output path is relative to the crate root. Ensure `temp/` exists
  at the repo root if you run tests from the workspace.
- If you add new grammar features or generator branches, extend `unit.yaidl` and
  add a focused test in `crates/lib/tests/` to lock in the behavior.
