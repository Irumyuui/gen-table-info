#![allow(unused)]

use gen_table_info::TableInfo;

#[derive(TableInfo)]
struct Foo {
    a: i32,
    b: u64,
    name: String,
    vec: Vec<i32>,
}

fn main() {
    println!("Foo: \n{}", Foo::table_info());
}
