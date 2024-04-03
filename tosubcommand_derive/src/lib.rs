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

            let keys = keys.iter().map(|key| {
                quote! {
                    #key
                }
            });

            quote! {
                #name::#variant_name { #(#fields),* } => {
                    // split Pascal variant or Camel case into kebab case. GenWitness becomes gen-witness
                    // then push it to the map
                    // this is camel or pascal case
                    let variant = format!("{}", stringify!(#variant_name));
                    // now convert to kebab case
                    let variant = variant.chars().fold(String::new(), |mut acc, c| {
                        if c.is_uppercase() {
                            if !acc.is_empty() {
                                acc.push('-');
                            }
                            acc.push(c.to_lowercase().next().unwrap());
                        } else {
                            acc.push(c);
                        }
                        acc
                    });

                    let mut map = vec![variant];
                    #(
                        // if field is not struct then to_string else try and call to_flags on the struct
                        if #fields.is_optional() {
                            if let Some(val) = #fields.to_flags().first() {
                                map.push(format!("--{}", #keys));
                                map.extend(#fields.to_flags());
                            }
                        } else {
                            if #fields.is_flag() {
                                map.extend(#fields.to_flags());
                            } else if #fields.is_value() {
                                map.push(format!("--{}", #keys));
                                map.extend(#fields.to_flags());
                            }
                        }
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

/// Converts a given input struct into a BTreeMap where the keys are the attribute names assigned to
/// the values of the entries.
#[proc_macro_derive(ToFlags, attributes(rename))]
pub fn to_flags(input_struct: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input_struct as DeriveInput);

    // check for struct type and parse out fields
    let fields = match ast.data {
        Data::Struct(st) => st.fields,
        _ => panic!("Implementation must be a struct"),
    };

    // parse out all the field names in the struct as `Ident`s
    let idents: Vec<&Ident> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<&Ident>>();

    // convert all the field names into strings
    let keys: Vec<String> = idents
        .clone()
        .iter()
        .map(|ident| ident.to_string())
        .map(|name| {
            // replace _ with - in the field names
            name.replace("_", "-")
        })
        .collect::<Vec<String>>();

    // get the name identifier of the struct input AST
    let name: &Ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // start codegen for to_flags functionality that converts a struct into a set of flags
    let tokens = quote! {

        impl #impl_generics ToFlags for #name #ty_generics #where_clause {

            fn to_flags(&self) -> Vec<String> {
                let mut map = vec![];
                // check if optional
                #(
                    if self.#idents.is_optional() {
                    if let Some(val) = self.#idents.to_flags().first() {
                        map.push(format!("--{}", #keys));
                        map.extend(self.#idents.to_flags());
                    }
                } else {
                    if self.#idents.is_flag() {
                        map.extend(self.#idents.to_flags());
                    } else if self.#idents.is_value() {
                        map.push(format!("--{}", #keys));
                        map.extend(self.#idents.to_flags());
                    }
                }
                )*
                map
            }

            fn is_flag(&self) -> bool {
                true
            }

            fn is_value(&self) -> bool {
                false
            }

        }
    };
    proc_macro::TokenStream::from(tokens)
}
