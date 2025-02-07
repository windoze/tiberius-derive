use tiberius_derive::FromRow;

#[derive(FromRow)]
pub struct Row<'b> {
    pub foo: Option<i32>,
    pub bar: Option<&'b str>,
}

#[derive(FromRow)]
pub struct RowNoLifetime {
    pub foo: Option<i32>,
}

#[derive(FromRow)]
#[tiberius_derive(owned)]
pub struct RowOwned {
    pub foo: Option<String>,
    pub bar: String,
}

#[derive(FromRow)]
#[tiberius_derive(by_position)]
pub struct RowByPosition {
    pub foo: Option<i32>,
    pub bar: i32,
}

#[derive(FromRow)]
#[tiberius_derive(rename_all = "camelCase")]
pub struct RowRenamed {
    pub foo_bar: Option<i32>,
    pub bar_foo: i32,
}

#[derive(FromRow)]
#[tiberius_derive(auto)]
pub struct AutoOwnedStringNoLifetime {
    pub foo_bar: String,
    pub bar_foo: i32,
}

#[derive(FromRow)]
#[tiberius_derive(auto)]
pub struct AutoOwnedString<'a> {
    pub foo_bar: String,
    pub foo_bar_optional: Option<String>,
    pub bar_foo: i32,
    pub not_auto: &'a str,
}

#[derive(FromRow)]
#[tiberius_derive(auto, by_position)]
pub struct AutoOwnedStringByPosition<'a> {
    pub foo_bar: String,
    pub foo_bar_optional: Option<String>,
    pub bar_foo: i32,
    pub not_auto: &'a str,
}

impl RowRenamed {
    pub fn multiple_impls_are_fine() {}
}

fn main() {
    println!("Hello, world!");
}
