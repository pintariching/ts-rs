#![allow(dead_code)]
use ts_rs::TS;

#[derive(TS)]
#[ts(type = "const enum")]
enum SimpleConstEnum {
    #[ts(rename = "a")]
    A,
    B,
}

#[test]
fn const_enum() {
    assert_eq!(
        SimpleConstEnum::decl(),
        r#"const enum SimpleConstEnum {A="a", B="B"}"#
    );
}

#[derive(TS)]
#[ts(type = "enum")]
enum SimpleEnum {
    A,
    B,
}

#[test]
fn simple_enum() {
    assert_eq!(SimpleEnum::decl(), r#"enum SimpleEnum {A, B}"#);
}

#[derive(TS)]
#[ts(type = "enum")]
enum EnumWithBothNumberAndARename {
    A = 1,
    #[ts(rename = "XD")]
    B,
}

#[test]
fn enum_with_both_number_and_rename() {
    assert_eq!(
        EnumWithBothNumberAndARename::decl(),
        r#"enum EnumWithBothNumberAndARename {A=1, B="XD"}"#
    );
}

#[derive(TS)]
#[ts(type = "enum")]
enum SimpleEnumWithNumberAssigned {
    A = 1,
    B,
}

#[test]
fn simple_enum_discriminant() {
    assert_eq!(
        SimpleEnumWithNumberAssigned::decl(),
        r#"enum SimpleEnumWithNumberAssigned {A=1, B}"#
    )
}

#[derive(TS)]
#[ts(type = "enum")]
enum SimpleEnumWithRename {
    #[ts(rename = "a")]
    A,
    B,
}

#[test]
fn simple_enum_variant_rename() {
    assert_eq!(
        SimpleEnumWithRename::decl(),
        r#"enum SimpleEnumWithRename {A="a", B="B"}"#
    );
}

#[derive(TS)]
#[ts(type = "enum")]
#[ts(rename_all = "lowercase")]
enum SimpleEnumWithInflection {
    #[ts(rename = "a")]
    A,
    B,
}

#[test]
fn simple_enum_inflection() {
    assert_eq!(
        SimpleEnumWithInflection::decl(),
        r#"enum SimpleEnumWithInflection {A="a", B="b"}"#
    );
}

#[derive(TS)]
// #[ts(type="type")]
enum SimpleEnumNotChanged {
    #[ts(rename = "a")]
    A,
    B,
}

#[test]
fn simple_enum_not_changed() {
    assert_eq!(
        SimpleEnumNotChanged::decl(),
        r#"type SimpleEnumNotChanged = "a" | "B";"#
    );
}
