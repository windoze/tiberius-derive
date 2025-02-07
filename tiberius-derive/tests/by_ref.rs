use tiberius_derive::FromRow;

#[derive(FromRow)]
pub struct Foobar<'a> {
    pub foo: Option<i32>,
    pub bar: Option<&'a str>,
}

#[derive(FromRow)]
#[tiberius_derive(auto)]
pub struct FoobarAuto<'a> {
    pub foo: Option<i32>,
    pub bar: Option<&'a str>,
    pub auto: String,
    pub auto_opt: Option<String>,
}

// impl<'a> Foobar<'a> {
//     fn from_row(__row: &'a tiberius::Row) -> Result<Foobar<'a>, tiberius::error::Error> {
//         Ok(Self {
//             foo: { __row.try_get("foo")? },
//             bar: { __row.try_get("bar")? },
//         })
//     }
// }

fn main() {}
