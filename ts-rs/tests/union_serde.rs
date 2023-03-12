#![allow(dead_code)]

use serde::Deserialize;
use ts_rs::TS;

#[derive(TS, Deserialize)]
#[serde(tag = "kind", content = "d")]
enum SimpleEnum {
    A,
    B,
}

#[derive(TS, Deserialize)]
#[serde(tag = "kind", content = "data")]
enum ComplexEnum {
    A,
    B { foo: String, bar: f64 },
    W(SimpleEnum),
    F { nested: SimpleEnum },
    T(i32, SimpleEnum),
}

#[derive(TS, Deserialize)]
#[serde(untagged)]
enum Untagged {
    Foo(String),
    Bar(i32),
    None,
}

#[cfg(feature = "serde-compat")]
#[test]
fn test_serde_enum() {
    assert_eq!(
        SimpleEnum::decl(),
        r#"type SimpleEnum = SimpleEnumA | SimpleEnumB;
export type SimpleEnumA = { kind: "A" };
export type SimpleEnumB = { kind: "B" };"#
    );
    assert_eq!(
        ComplexEnum::decl(),
        r#"type ComplexEnum = ComplexEnumA | ComplexEnumB | ComplexEnumW | ComplexEnumF | ComplexEnumT;
export type ComplexEnumA = { kind: "A" };
export type ComplexEnumB = { kind: "B", data: { foo: string, bar: number, } };
export type ComplexEnumW = { kind: "W", data: SimpleEnum };
export type ComplexEnumF = { kind: "F", data: { nested: SimpleEnum, } };
export type ComplexEnumT = { kind: "T", data: [number, SimpleEnum] };"#
    );

    assert_eq!(
        Untagged::decl(),
        r#"type Untagged = UntaggedFoo | UntaggedBar | UntaggedNone;
export type UntaggedFoo = string;
export type UntaggedBar = number;
export type UntaggedNone = null;"#
    )
}
