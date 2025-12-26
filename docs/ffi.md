# FFI Shim

The FFI shim lets non-Rust code plug a generator into YAIDL by supplying
callbacks that build `Code` trees. The Rust side wraps those callbacks in a
`Generator` implementation so the rest of the pipeline can stay unchanged.

## Build + header generation

- Build the cdylib: `cargo build -p genlib --release`
- Generate a C header (optional): `cbindgen --config crates/lib/cbindgen.toml --crate genlib --output <out>.h`

The shared library name depends on platform (for example, `libgenlib.so`,
`libgenlib.dylib`, or `genlib.dll`).

## FFI surface

The FFI API lives in `crates/lib/src/generators/ffi/` and
`crates/lib/src/builder/ffi.rs`. The exported symbols are the `extern "C"` and
`#[no_mangle]` functions in those files; use `cbindgen` to see the exact symbol
names for your target.

### Wrapper types

These `repr(C)` structs carry raw pointers to internal Rust data. Treat them as
borrowed views that are only valid for the duration of the callback.

- `TypeWrapper` -> `*const Type`
- `TypeInfoWrapper` -> `*const TypeInformation`
- `EndpointWrapper` -> `*const EndPoint`
- `DefinitionsWrapper` -> `*const Definitons`
- `YaildStringView` -> `{ ptr, len }` view into UTF-8 data (not null-terminated)

### Generator callback signatures

The generator adapter calls into these callbacks to ask for code:

- `TypeHeader`: type header imports/aliases
- `EndpointHeader`: endpoint header imports/aliases
- `Type`: one type definition (name + model)
- `TypeTranslation`: domain <-> wire translations
- `Endpoint`: one endpoint definition

Parameters are:

- `*const c_void`: opaque `this` pointer you provide
- `*const c_char`: UTF-8 C string (type/endpoint name)
- `c_char`: `public` flag (`0` = false, non-zero = true)
- `DefinitionsWrapper`/`TypeWrapper`/`TypeInfoWrapper`/`EndpointWrapper`
- Return value: `CodeFFI`

### Generator adapter

`GeneratorFFI` stores your callbacks and is the bridge to the Rust `Generator`
trait. Its setters are exposed through the FFI surface:

- `new` -> create adapter
- `set_header_type`
- `set_header_endpoint`
- `set_type`
- `set_wire_translation`
- `set_domain_translation`
- `set_endpoint`
- `set_this`

All callbacks and `this` must be set before the adapter is used.

## Code builder helpers

`CodeFFI` is a C-friendly representation of the internal `Code` tree:

- `new_line`, `new_segment`, `new_block` build nodes
- `add_child` attaches an owned child node
- `create_child_segment`, `create_child_block` return references to newly created
  nodes inside the parent
- `add_line` appends a line node to a segment or block

When you return a `CodeFFI::Code`, ownership transfers to Rust. The `Ref` variant
is only returned by `create_child_*` and must not be freed or reused after the
parent is returned.

## Safety + ownership notes

- Wrapper pointers and C strings are borrowed for the duration of the callback.
  Do not store them.
- `public` is a `c_char`; treat any non-zero value as `true`.
- `CodeFFI` values returned to Rust are owned by Rust; do not free them on the
  foreign side.
- `YaildStringView` points at UTF-8 bytes with an explicit length; the data is
  not null-terminated.

## Minimal flow sketch (pseudocode)

```c
GeneratorFFI gen = GeneratorFFI_new();
gen = GeneratorFFI_set_this(gen, my_context);
gen = GeneratorFFI_set_type(gen, my_type_cb);
gen = GeneratorFFI_set_wire_translation(gen, my_wire_cb);
gen = GeneratorFFI_set_domain_translation(gen, my_domain_cb);
gen = GeneratorFFI_set_endpoint(gen, my_endpoint_cb);
gen = GeneratorFFI_set_header_type(gen, my_type_header_cb);
gen = GeneratorFFI_set_header_endpoint(gen, my_endpoint_header_cb);

// Pass `gen` to the Rust side that drives YAIDL generation.
```
