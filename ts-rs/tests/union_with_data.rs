#![allow(dead_code)]
use serde::Serialize;
use ts_rs::{Dependency, TS};

#[derive(TS, Serialize)]
struct Bar {
    field: i32,
}

#[derive(TS, Serialize)]
struct Foo {
    bar: Bar,
}

#[derive(TS, Serialize)]
enum SimpleEnum {
    A(String),
    B(i32),
    C,
    D(String, i32),
    E(Foo),
    F { a: i32, b: String },
}

#[test]
fn test_stateful_enum() {
    assert_eq!(Bar::decl(), r#"interface Bar { field: number, }"#);
    assert_eq!(Bar::dependencies(), vec![]);

    assert_eq!(Foo::decl(), r#"interface Foo { bar: Bar, }"#);
    assert_eq!(
        Foo::dependencies(),
        vec![Dependency::from_ty::<Bar>().unwrap()]
    );

    assert_eq!(
        SimpleEnum::decl(),
        r#"type SimpleEnum = SimpleEnumA | SimpleEnumB | SimpleEnumC | SimpleEnumD | SimpleEnumE | SimpleEnumF;
export type SimpleEnumA = { "A": string };
export type SimpleEnumB = { "B": number };
export type SimpleEnumC = "C";
export type SimpleEnumD = { "D": [string, number] };
export type SimpleEnumE = { "E": Foo };
export type SimpleEnumF = { "F": { a: number, b: string, } };"#
    );
    assert!(SimpleEnum::dependencies()
        .into_iter()
        .all(|dep| dep == Dependency::from_ty::<Foo>().unwrap()),);
}
