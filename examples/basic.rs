use gen_table_info::prelude::*;

fn main() {
    let comment = StructCommentBuilder::new()
        .add_entry(Entry::with_name("key").with_type("String"))
        .add_entry(Entry::with_name("value").with_type("(String, String, String)"))
        .add_entry(Entry::with_name("size").with_type("u64"))
        .build();

    println!("{}", comment);
}
