//! Smoke test for the local `wasm-tools` parameter-mode experiment.
//!
//! The published parser version used by this `wit-bindgen` revision does not
//! understand these modes or expose `ParamMode`. Consequently, compiling and
//! passing this test demonstrates that both the Rust macro and this test are
//! using the local `wasm-tools` checkout configured in the workspace.
//!
//! This is a binding-generation test only. It does not assert that mutations
//! are transported across the canonical component ABI.

use wit_parser::{ParamMode, Resolve, WorldItem};

mod generated {
    wit_bindgen::generate!({
        inline: r#"
            package local:param-modes;

            world smoke {
                import exercise: func(
                    in input: u32,
                    out output: u32,
                    inout state: u32,
                );

                export exercise-export: func(
                    in input: u32,
                    out output: u32,
                    inout state: u32,
                );
            }
        "#,
        world: "smoke",
    });
}

struct Component;

impl generated::Guest for Component {
    fn exercise_export(input: u32, output: &mut u32, state: &mut u32) {
        *output = input;
        *state += input;
    }
}

#[test]
fn macro_generates_mode_aware_rust_signatures() {
    let _: fn(u32, &mut u32, &mut u32) = generated::exercise;
    let _ = Component;
}

#[test]
fn macro_uses_local_wasm_tools_parameter_modes() {
    const WIT: &str = r#"
        package local:param-modes;

        world smoke {
            import exercise: func(
                in input: u32,
                out output: u32,
                inout state: u32,
            );
        }
    "#;

    let mut resolve = Resolve::default();
    let package = resolve.push_str("parameter-modes.wit", WIT).unwrap();
    let world = resolve.select_world(&[package], Some("smoke")).unwrap();
    let function = resolve.worlds[world]
        .imports
        .values()
        .find_map(|item| match item {
            WorldItem::Function(function) => Some(function),
            _ => None,
        })
        .expect("the exercise import should resolve to a function");

    let modes = function
        .params
        .iter()
        .map(|param| param.mode)
        .collect::<Vec<_>>();

    assert_eq!(modes, vec![ParamMode::In, ParamMode::Out, ParamMode::InOut]);
}
