#![feature(proc_macro_quote)]

use std::vec;

use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident};

// A macro that takes a struct and prints it as a subcommand with flags
// for instance Command { input: "input.json", output: "output.json" } becomes "command --input input.json --output output.json"
#[proc_macro_derive(ToSubcommand, attributes(rename))]
pub fn to_subcommand(input_enum: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input_enum as DeriveInput);

    // check for enum type and parse out fields from struct variants
    let fields: Vec<Vec<&Field>> = match ast.data {
        Data::Enum(ref data) => {
            // parse out all the fields if the enum is a struct
            data.variants
                .iter()
                .map(|variant| match &variant.fields {
                    Fields::Named(fields) => fields.named.iter().collect::<Vec<_>>(),
                    Fields::Unit => vec![],
                    _ => panic!("This macro only works named or unit structs"),
                })
                .collect::<Vec<_>>()
        }
        _ => panic!("This macro only works with enums"),
    };

    let variant_names = match ast.data {
        Data::Enum(ref data) => data
            .variants
            .iter()
            .map(|variant| variant.ident.clone())
            .collect::<Vec<_>>(),
        _ => panic!("This macro only works with enums"),
    };

    // parse out all the field names in the struct as `Ident`s
    let idents: Vec<Vec<&Ident>> = fields
        .iter()
        .map(|field| {
            field
                .iter()
                .filter_map(|f| f.ident.as_ref())
                .collect::<Vec<_>>()
        })
        .collect();

    // convert all the field names into strings
    let keys: Vec<Vec<String>> = idents
        .clone()
        .iter()
        .map(|ident| {
            ident
                .iter()
                .map(|name| name.to_string())
                .map(|name|
                    // replace _ with - in the field names
                    name.replace("_", "-"))
                .collect::<Vec<_>>()
        })
        .collect();

    // create a map of variant names to keys
    let variant_keys: Vec<(Ident, Vec<String>)> = variant_names
        .iter()
        .zip(keys.iter())
        .map(|(variant_name, keys)| (variant_name.clone(), keys.clone()))
        .collect();

    // get the name identifier of the struct input AST
    let name: &Ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    println!("{:?}", name);

    // create a match statement for each variant in the enum
    let match_statements = variant_keys
        .iter()
        .zip(fields.iter())
        .map(|(var_keys, fields)| {
            let variant_name = &var_keys.0;
            let keys = &var_keys.1;
            let fields = fields.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                quote! {
                    #field_name
                }
            });

            let fields = fields.clone().collect::<Vec<_>>();

            println!("{:?}", keys);

            let keys = keys.iter().map(|key| {
                quote! {
                    #key
                }
            });



            quote! {
                #name::#variant_name { #(#fields),* } => {
                    let mut map = vec![stringify!(#variant_name).to_lowercase()];
                    #(
                        map.push(format!("--{}", #keys));
                        map.push(format!("{}", #fields));
                    )*
                    map
                }
            }
        });

    // now impl the ToSubcommand trait for the input AST
    let expanded = quote! {
        impl #impl_generics tosubcommand::ToSubcommand for #name #ty_generics #where_clause {
            fn to_subcommand(&self) -> Vec<String> {
                match self {
                    #(#match_statements),*
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
    //
}
