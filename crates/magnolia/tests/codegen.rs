use wit_bindgen_core::{Files, wit_parser::Resolve};

#[test]
fn generates_abstract_scalar_signatures() {
    const WIT: &str = r#"
        package learning:magnolia;

        world calculator {
            import add: func(a: s32, b: s32) -> s32;

            export operations: interface {
                double: func(value: s32) -> s32;
                notify: func(message: string);
            }
        }
    "#;

    let mut resolve = Resolve::default();
    let package = resolve.push_str("calculator.wit", WIT).unwrap();
    let world = resolve
        .select_world(&[package], Some("calculator"))
        .unwrap();
    let mut files = Files::default();
    let mut generator = wit_bindgen_magnolia::Opts::default().build();

    generator.generate(&mut resolve, world, &mut files).unwrap();

    let generated = String::from_utf8(files.remove("calculator.mg").unwrap()).unwrap();
    assert!(generated.starts_with("package calculator;\n"));
    assert!(generated.contains("signature CalculatorTypes = {"));
    assert!(generated.contains("type WitS32;"));
    assert!(generated.contains("type WitString;"));
    assert!(generated.contains("signature CalculatorImports = {"));
    assert!(generated.contains("procedure add(obs a: WitS32, obs b: WitS32, out result: WitS32);"));
    assert!(generated.contains("signature CalculatorOperationsExports = {"));
    assert!(generated.contains("procedure double(obs value: WitS32, out result: WitS32);"));
    assert!(generated.contains("procedure notify(obs message: WitString);"));
}

#[test]
fn reports_unsupported_compound_types() {
    const WIT: &str = r#"
        package learning:magnolia;

        world records {
            record point {
                x: s32,
                y: s32,
            }
            export inspect: func(value: point);
        }
    "#;

    let mut resolve = Resolve::default();
    let package = resolve.push_str("records.wit", WIT).unwrap();
    let world = resolve.select_world(&[package], Some("records")).unwrap();
    let mut files = Files::default();
    let mut generator = wit_bindgen_magnolia::Opts::default().build();

    let error = generator
        .generate(&mut resolve, world, &mut files)
        .unwrap_err();
    assert!(
        error.to_string().contains("is not supported"),
        "unexpected error: {error:#}"
    );
}

#[test]
fn preserves_wit_out_parameter_mode() {
    const WIT: &str = r#"
        package learning:magnolia;

        world calculator {
            export divide: func(
                dividend: u32,
                divisor: u32,
                out remainder: u32,
            );
        }
    "#;

    let mut resolve = Resolve::default();
    let package = resolve.push_str("calculator.wit", WIT).unwrap();
    let world = resolve
        .select_world(&[package], Some("calculator"))
        .unwrap();
    let mut files = Files::default();
    let mut generator = wit_bindgen_magnolia::Opts::default().build();

    generator.generate(&mut resolve, world, &mut files).unwrap();

    let generated = String::from_utf8(files.remove("calculator.mg").unwrap()).unwrap();
    assert!(generated.contains(
        "procedure divide(obs dividend: WitU32, obs divisor: WitU32, out remainder: WitU32);"
    ));
}
