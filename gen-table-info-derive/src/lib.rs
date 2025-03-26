use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(TableInfo)]
pub fn derive_table_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("TableInfo just use for struct with named fields"),
        },
        _ => panic!("TableInfo must be used on a struct"),
    };

    let field_entries = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            builder.add_entry(::gen_table_info::Entry::with_name(stringify!(#field_name))
                .with_type(stringify!(#field_type)));
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn table_info() -> String {
                let mut builder = ::gen_table_info::StructCommentBuilder::new();
                #(#field_entries)*
                builder.build()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
