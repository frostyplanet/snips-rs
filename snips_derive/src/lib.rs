#![recursion_limit = "512"]

#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use syn::{MetaNameValue, MetaList};
use syn::Meta::{List, NameValue};
use syn::NestedMeta::Meta;

extern crate snips_core;

use std::prelude::v1::*;
use std::collections::HashMap;


#[proc_macro_derive(SnipsInput, attributes(snips))]
pub fn snips_input(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive_snips_input(&input)
}

#[allow(non_shorthand_field_patterns)]
fn get_snips_field_items(m: &mut HashMap<String, String>, attr: &syn::Attribute) {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "snips" {
        let item = attr.interpret_meta();
        match item {
            Some(List(MetaList{ident: _, paren_token: _ , nested: ref nest}))=>{
                for p in nest.iter() {
                    match p {
                        Meta(NameValue(MetaNameValue{ident: ref ident, eq_token:_, lit: ref lit})) =>
                            if let syn::Lit::Str(s)=lit {
                                m.insert(ident.to_string(), s.value());
                            } else {
                                println!("other lit={:#?}", lit);
                            }
                        _ =>println!("not supported pun value ={:#?}", p),
                    }
                }
            },
            _=>println!("not supported item={:#?}", item),
        }
    }
}

fn derive_snips_input(ast: &syn::DeriveInput) -> TokenStream {

    let name = &ast.ident;
    let n = "ahhaha";
    if let syn::Data::Struct(ref s) = ast.data {

        let mut headers_fields: Vec<String> = Vec::new();
        let mut param_fields: Vec<String> = Vec::new();
        let mut elm_fields: Vec<String> = Vec::new();

        if let syn::Fields::Named(ref n) = s.fields {
            for ref p in n.named.iter() {
                let mut attr_items = HashMap::new();
                for ref attr in p.attrs.iter() {
                    get_snips_field_items(&mut attr_items, attr);
                }
                if attr_items.len() > 0 {
                    if let Some(ref ident) = p.ident {
                        if let Some(ref field_name) = attr_items.get("loc") {
                            match field_name.as_ref() {
                                "param" => param_fields.push(ident.to_string()),
                                "header" => headers_fields.push(ident.to_string()),
                                "elm" => elm_fields.push(ident.to_string()),
                                _ => println!("not supported attr {:?} on {}", field_name, ident.to_string()),
                            }
                        }
                    } else {
                        println!("other type of ident {:#?}", p.ident)
                    }
                }
            }
        } else {
            println!("other type of s.Fields {:#?}", s.fields);
        }
        println!("param_fields: {:?}, headers_fields: {:?}, elm_fields: {:?}", param_fields, headers_fields, elm_fields);

        let gen = quote! {

            use std::collections::HashMap;
            use snips_core::interface::*;
            impl SnipsInput for #name {

                fn get_headers(&self) -> Option<HashMap<&str, &str>> {
                    println!("name={}", #n);
                    let mut headers: HashMap<&str, &str> = HashMap::new();
                    //for field_name in #headers_fields {
                    //    headers.insert()
                    //}
                    None
                }

                fn get_params(&self) -> Option<HashMap<&str, &str>> {
                    None
                }
            }
        };
        gen.into()
    } else {
        // Nope. This is an Enum. We cannot handle these!
        panic!("#[derive(SnipsInput)] is only defined for structs, not for enums!");
    }
}
