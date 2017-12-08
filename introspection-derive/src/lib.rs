#![recursion_limit="128"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate introspection;

use proc_macro::TokenStream;

#[proc_macro_derive(StaticIntrospection)]
pub fn static_introspection(input: TokenStream) -> TokenStream {
    implement_introspection(input, static_implementor)
}

#[proc_macro_derive(DynamicIntrospection)]
pub fn dynamic_introspection(input: TokenStream) -> TokenStream {
    implement_introspection(input, dynamic_implementor)
}
#[proc_macro_derive(IntoIntrospection)]
pub fn into_introspection(input: TokenStream) -> TokenStream {
    implement_introspection(input, into_implementor)
}

fn implement_introspection(input: TokenStream, implementor: fn(&syn::DeriveInput) -> quote::Tokens) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = implementor(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn get_entity_type(ast: &syn::DeriveInput) -> introspection::Type {
    if let syn::Body::Enum(_) = ast.body {
        return introspection::Type::Enum;
    }
    introspection::Type::Struct
}

fn get_fields_names(ast: &syn::DeriveInput) -> Vec<String> {
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

fn quote_field_names(fields: &Vec<String>) -> quote::Tokens {
    quote! {
        vec![
            #(#fields.to_owned(),)*
        ]
    }
}

fn quote_field_objects(name: &str, fields: &Vec<String>) -> quote::Tokens {
    quote! {
        vec![
            #name::#(#fields.to_owned(),)*
        ]
    }
}

// fn quote_field_objects_mut(name: &str, fields: &Vec<String>) -> quote::Tokens {
//     quote! {
//         vec![
//             #name::#(#fields.to_owned(),)*
//         ]
//     }
// }


fn static_implementor(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let vis = introspection::Visibility::from(ast.vis.clone());
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let entity_type = get_entity_type(&ast);
    let field_names = get_fields_names(&ast);
    let quoted_field_names = quote_field_names(&field_names);

    quote! {
        impl #impl_generics introspection::StaticIntrospection for #name #ty_generics #where_clause {
            fn static_introspection() -> introspection::StaticIntrospectionInfo {
                introspection::StaticIntrospectionInfo {
                    ident: stringify!(#name).to_owned(),
                    visibility: #vis,
                    entity_type: #entity_type,
                    fields: #quoted_field_names,
                }
            }
        }
    }
}

fn dynamic_implementor(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let vis = introspection::Visibility::from(ast.vis.clone());
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let entity_type = get_entity_type(&ast);
    let field_names = get_fields_names(&ast);
    let quoted_field_names = quote_field_names(&field_names);
    let quoted_field_objects = quote_field_objects(name.as_ref(), &field_names);

    quote! {
        impl #impl_generics introspection::DynamicIntrospection for #name #ty_generics #where_clause {
            fn dynamic_introspection<'a>(&'a self) -> introspection::DynamicIntrospectionInfo<'a, Self> {
                introspection::DynamicIntrospectionInfo {
                    ident: stringify!(#name).to_owned(),
                    visibility: #vis,
                    entity_type: #entity_type,
                    // fields: #quoted_field_objects,
                    fields: Vec::default(),
                    value: Some(self),
                }
            }

            fn dynamic_introspection_mut<'a>(&'a mut self) -> introspection::DynamicIntrospectionInfoMut<'a, Self> {
                introspection::DynamicIntrospectionInfoMut {
                    ident: stringify!(#name).to_owned(),
                    visibility: #vis,
                    entity_type: #entity_type,
                    fields: Vec::default(),
                    value: Some(self),
                }
            }
        }
    }
}


fn into_implementor(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let vis = introspection::Visibility::from(ast.vis.clone());
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let entity_type = get_entity_type(&ast);
    let field_names = get_fields_names(&ast);
    let quoted_field_names = quote_field_names(&field_names);
    let quoted_field_objects = quote_field_objects(name.as_ref(), &field_names);

    quote! {
        impl #impl_generics introspection::IntoIntrospection for #name #ty_generics #where_clause {
            fn into_introspection(mut self) -> introspection::IntoDynamicIntrospectionInfo<Self> {
                introspection::IntoDynamicIntrospectionInfo {
                    ident: stringify!(#name).to_owned(),
                    visibility: #vis,
                    entity_type: #entity_type,
                    fields: Vec::default(),
                    value: Some(self),
                }
            }
        }
    }
}
