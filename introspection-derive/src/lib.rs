extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate introspection;

use proc_macro::TokenStream;

#[proc_macro_derive(Introspection)]
pub fn introspection(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_introspection(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn get_entity_type(ast: &syn::DeriveInput) -> introspection::Type {
    if let syn::Body::Enum(_) = ast.body {
        return introspection::Type::Enum;
    }
    introspection::Type::Struct
}

fn get_fields(ast: &syn::DeriveInput) -> Vec<String> {
    match ast.body {
        syn::Body::Enum(ref variants) => {
            variants.iter().map(|v| v.ident.clone().to_string()).collect::<Vec<String>>()
        },
        syn::Body::Struct(ref variant_data) => {
            variant_data.fields().iter().map(|f| {
                if let Some(ref id) = f.ident {
                    id.clone().to_string()
                } else {
                    String::default()
                }
            }).collect::<Vec<String>>()
        },
    }
}

fn quote_fields(fields: Vec<String>) -> quote::Tokens {
    quote! {
        vec![
            #(#fields.to_owned(),)*
        ]
    }
}

fn impl_introspection(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let vis = introspection::Visibility::from(ast.vis.clone());
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let entity_type = get_entity_type(&ast);
    let fields = quote_fields(get_fields(&ast));
    quote! {
        impl #impl_generics introspection::Introspection for #name #ty_generics #where_clause {
            fn introspection() -> introspection::IntrospectionInfo {
                introspection::IntrospectionInfo {
                    ident: stringify!(#name).to_owned(),
                    visibility: #vis,
                    entity_type: #entity_type,
                    fields: #fields,
                }
            }

            fn fields() -> introspection::FieldsInfo {
                introspection::FieldsI
            }
        }
    }
}
