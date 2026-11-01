<div align="center">
  <h1><code>wit-bindgen</code></h1>

  <p>
    <strong>Guest Rust language bindings generator for
    <a href="https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md">WIT</a>
    and the
    <a href="https://github.com/WebAssembly/component-model">Component Model</a>
    </strong>
  </p>

<strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <img src="https://img.shields.io/badge/rustc-stable+-green.svg" alt="supported rustc stable" />
    <a href="https://docs.rs/wit-bindgen"><img src="https://docs.rs/wit-bindgen/badge.svg" alt="Documentation Status" /></a>
  </p>
</div>

# About

This crate provides a macro, [`generate!`], to automatically generate Rust
bindings for a [WIT] [world]. For more information about this crate see the
[online documentation] which includes some examples and longer form reference
documentation as well.

This crate is developed as a portion of the [`wit-bindgen` repository] which
also contains a CLI and generators for other languages.

## Experimental parameter-mode prototype

The `experiment/rust-parameter-modes` branch demonstrates how WIT parameter
direction can inform Rust binding generation. It uses the local experimental
`wasm-tools` parser and maps resolved parameter modes into generated Rust APIs:

| WIT parameter | Generated Rust parameter |
| --- | --- |
| `in value: T` (or unmarked) | `value: T` using the existing ownership rules |
| `out value: T` | `value: &mut T` |
| `inout value: T` | `value: &mut T` |

The [`generate!`] procedural macro reads the WIT at compile time and delegates
to `wit-bindgen-rust`, where these signatures are produced for both imported
functions and exported `Guest` traits.

This prototype is intentionally limited to binding generation. The canonical
component ABI still treats all parameters as ordinary inputs, so mutations
made through generated `&mut T` parameters are not transported back across a
component boundary. The generated adapter code only bridges the new signatures
well enough to compile and demonstrate the language-facing API. It must not be
treated as runtime output-parameter support.

Run the focused parser-and-macro test from the repository root with:

```console
cargo test -p wit-bindgen --test local_wasm_tools
```

The test verifies that `In`, `Out`, and `InOut` survive parsing, that an import
has the expected Rust function type, and that an exported `Guest` trait accepts
an implementation with the corresponding mutable-reference parameters.

[`generate!`]: https://docs.rs/wit-bindgen/latest/wit_bindgen/macro.generate.html
[WIT]: https://component-model.bytecodealliance.org/design/wit.html
[world]: https://component-model.bytecodealliance.org/design/worlds.html
[online documentation]: https://docs.rs/wit-bindgen
[`wit-bindgen` repository]: https://github.com/bytecodealliance/wit-bindgen

# License

This project is licensed under the Apache 2.0 license with the LLVM exception.
See [LICENSE](LICENSE) for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be licensed as above, without any additional terms or conditions.
