pub mod prelude {
    pub use crate::Entry;
    pub use crate::StructCommentBuilder;
}

/// usage:
///
/// ```rust
/// use gen_table_info::prelude::*;
/// 
/// Entry::with_name("key").with_type("String");
/// ```
pub struct Entry {
    field_name: String,
    file_type: String,
}

/// usage:
///
/// ```rust
/// use gen_table_info::prelude::*;
/// 
/// StructCommentBuilder::new()
///     .add_entry(Entry::with_name("key").with_type("String"))
///     .add_entry(Entry::with_name("value").with_type("(String, String, String)"))
///     .add_entry(Entry::with_name("size").with_type("u64"))
///     .build();
/// ```
#[derive(Default)]
pub struct StructCommentBuilder {
    entries: Vec<Entry>,
}

impl Entry {
    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            field_name: name.into(),
            file_type: String::new(),
        }
    }

    pub fn with_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = file_type.into();
        self
    }
}

impl StructCommentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(&mut self, entry: Entry) -> &mut Self {
        self.entries.push(entry);
        self
    }

    pub fn build(&self) -> String {
        if self.entries.is_empty() {
            return String::new();
        }

        let mut max_content_length = 0;
        for entry in &self.entries {
            let content_length = format!("{}: {}", entry.field_name, entry.file_type).len();
            if content_length > max_content_length {
                max_content_length = content_length;
            }
        }

        let border = format!("+{}+", "-".repeat(max_content_length + 2));

        let mut result = String::new();
        result.push('\n');
        result.push_str(&border);
        result.push('\n');

        for entry in &self.entries {
            let content = format!("{}: {}", entry.field_name, entry.file_type);
            let padding = " ".repeat(max_content_length - content.len());

            result.push_str(&format!("| {}{} |\n", content, padding));
            result.push_str(&border);
            result.push('\n');
        }

        result
    }
}
