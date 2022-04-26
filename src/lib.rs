use proc_macro::TokenStream;
use quote::quote;

#[derive(Default)]
struct InnerAttrArgs {
    skip: bool,
}

fn parse_inner_attr_args(raw_args: Vec<syn::NestedMeta>) -> InnerAttrArgs {
    let mut args = InnerAttrArgs::default();

    for arg in raw_args {
        match arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::List(list) => {
                    for meta in list.nested {
                        parse_inner_attr_args(vec![meta]);
                    }
                }
                syn::Meta::Path(path) => {
                    for segment in path.segments.iter() {
                        if segment.ident == *"skip" {
                            args.skip = true;
                        }
                    }
                }
                _ => {
                    panic!("Unsupported argument");
                }
            },
            _ => {
                panic!("Unsupported argument");
            }
        }
    }

    args
}

/// The macro that processes structs.
/// # Examples
/// ```
/// #[recorder::record] // this struct will be #[derive(Debug, Serialize, Deserialize)]
/// pub struct MyRecord {
///     a: String, // this will be `pub`
///     b: u8, // this too!
///     c: Vec<String>, // and this!
///     #[record(skip)] d: String // this field will be preserved (private by default)
/// }
/// ```
#[proc_macro_attribute]
pub fn record(raw_args: TokenStream, raw_input: TokenStream) -> TokenStream {
    let _raw_args = syn::parse_macro_input!(raw_args as syn::AttributeArgs);
    let input: syn::ItemStruct = syn::parse_macro_input!(raw_input);

    let orig_attrs = &input.attrs;
    let vis = &input.vis;
    let struct_token = &input.struct_token;
    let ident = &input.ident;
    let generics = &input.generics;
    let semi_token = &input.semi_token;

    let attrs = if cfg!(feature = "serde") {
        quote! { #[derive(Debug, serde::Serialize, serde::Deserialize )]}
    } else {
        quote! { #[derive(Debug)] }
    }
    .into_iter();

    let mut fields_orig = if let syn::Fields::Named(fields) = input.fields {
        fields.named
    } else if let syn::Fields::Unnamed(fields) = input.fields {
        fields.unnamed
    } else {
        panic!("Unit structs are not supported by `recorder::record`")
    };

    let fields = fields_orig.iter_mut().map(|f: &mut syn::Field| {
        let attrs = &mut f.attrs;
        let vis = &f.vis;
        let ident = f.ident.as_ref();
        let colon_token = &f.colon_token;
        let ty = &f.ty;

        let mut skip = false;

        let mut new_attrs = vec![];
        for attr in attrs {
            let attr_args: syn::NestedMeta = attr.parse_args().unwrap();
            if attr.path.segments.first().unwrap().ident == *"record" {
                let parsed_args = parse_inner_attr_args(vec![attr_args]);
                if parsed_args.skip {
                    skip = true;
                }

                continue; // ignore our own invalid attributes so they are not emitted as code
            } else {
                new_attrs.push(attr); // keep non-record attributes
            }
        }

        if skip {
            // If skipped, return the field with only our own attributes removed
            quote! {
                #(#new_attrs)* #vis #ident #colon_token #ty
            }
        } else {
            // Otherwise, return the field with the new attributes and public visibility
            quote! {
                #(#new_attrs)* pub #ident #colon_token #ty
            }
        }
    });

    // Reconstruct the struct with our new attributes and fields
    if semi_token.is_some() {
        quote! {
            #(#orig_attrs)* #(#attrs)* #vis #struct_token #ident #generics (#(#fields),*) #semi_token
        }
    } else {
        quote! {
            #(#orig_attrs)* #(#attrs)* #vis #struct_token #ident #generics {
                #(#fields),*
            }
        }
    }
    .into()
}
