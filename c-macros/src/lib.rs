use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Ident, Type};

struct StructItem {
    ident: Ident,
    ty: Type,
}

struct EnumItem {
    ident: Ident,
    tys: Vec<Type>,
}

fn get_struct_items(data: &DataStruct) -> Vec<StructItem> {
    let mut items = Vec::new();
    for field in &data.fields {
        items.push(StructItem {
            ident: field.ident.clone().unwrap(),
            ty: field.ty.clone(),
        });
    }
    items.pop();
    items
}

fn get_enum_items(data: &DataEnum) -> Vec<EnumItem> {
    let mut items = Vec::new();
    for variant in &data.variants {
        items.push(EnumItem {
            ident: variant.ident.clone(),
            tys: variant
                .fields
                .iter()
                .map(|field| field.ty.clone())
                .collect(),
        });
    }
    items
}

fn spawn_placeholders(len: usize) -> Vec<TokenStream2> {
    (0..len)
        .map(|i| {
            let id = format_ident!("_{}", i);
            quote!(#id)
        })
        .collect()
}

#[proc_macro_derive(Parse)]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let extends = match &input.data {
        Data::Struct(data) => {
            let items = get_struct_items(data);
            let assigns = items.iter().map(|item| {
                let ident = &item.ident;
                let ty = &item.ty;
                quote! {
                    let #ident = <#ty as Parse>::parse(cursor)?;
                    span = span.merge(&#ident.span());
                }
            });
            let idents = items.iter().map(|item| {
                let ident = &item.ident;
                quote! {
                    #ident
                }
            });
            quote! (
                let mut span = Span::empty();
                #(#assigns)*
                Ok(Self {
                    #(#idents,)*
                    span,
                })
            )
        }
        Data::Enum(data) => {
            let items = get_enum_items(data);
            let check_enums = items.iter().map(|item| {
                let ident = &item.ident;
                let tys = &item.tys;
                let placeholders = spawn_placeholders(tys.len());
                quote!(
                    {
                        let mut c = cursor.clone();
                        match <(#(#tys),*) as Parse>::parse(&mut c) {
                            Ok((#(#placeholders),*)) => {
                                return Ok(#name::#ident(#(#placeholders),*));
                            }
                            Err(s) => {
                                if span.cmp(&s) == std::cmp::Ordering::Less {
                                    span = s;
                                }
                            }
                        }
                    }
                )
            });
            quote!(
                let mut span = cursor.span();
                #(#check_enums)*
                Err(span)
            )
        }
        _ => {
            return syn::Error::new(name.span(), "Only support struct or enum now")
                .to_compile_error()
                .into();
        }
    };
    quote! (
        impl Parse for #name {
            fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
                #extends
            }
        }
    )
    .into()
}

#[proc_macro_derive(SemanticAnalyze)]
pub fn derive_semantic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let extends = match &input.data {
        Data::Struct(data) => {
            let items = get_struct_items(data);
            let idents = items.iter().map(|item| {
                let ident = &item.ident;
                quote! {
                    #ident
                }
            });
            quote!(
                #(self.#idents.semantic_analyze();)*;
            )
        }
        Data::Enum(data) => {
            let items = get_enum_items(data);
            let match_arms = items.iter().map(|item| {
                let ident = &item.ident;
                let tys = &item.tys;
                let placeholders = spawn_placeholders(tys.len());
                quote!(
                    #name::#ident(#(#placeholders),*) => {
                        #(#placeholders.semantic_analyze();)*
                    }
                )
            });
            quote!(
                let mut span = Span::empty();
                match self {
                    #(#match_arms)*
                }
            )
        }
        _ => {
            return syn::Error::new(name.span(), "Only support struct or enum now")
                .to_compile_error()
                .into();
        }
    };
    quote! (
        impl SemanticAnalyze for #name {
            fn semantic_analyze(&self) -> Result<(), Span> {
                #extends
                Ok(())
            }
        }
    )
    .into()
}

#[proc_macro_derive(Node)]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let extends = match &input.data {
        Data::Struct(_) => {
            quote!(
                self.span
            )
        }
        Data::Enum(data) => {
            let items = get_enum_items(data);
            let match_arms = items.iter().map(|item| {
                let ident = &item.ident;
                let tys = &item.tys;
                let placeholders = spawn_placeholders(tys.len());
                quote!(
                    #name::#ident(#(#placeholders),*) => {
                        #(span = span.merge(&#placeholders.span());)*
                        span
                    }
                )
            });
            quote!(
                let mut span = Span::empty();
                match self {
                    #(#match_arms)*
                }
            )
        }
        _ => {
            return syn::Error::new(name.span(), "Only support struct or enum now")
                .to_compile_error()
                .into();
        }
    };
    quote! (
        impl Node for #name {
            fn span(&self) -> Span {
                #extends
            }
        }
    )
    .into()
}
