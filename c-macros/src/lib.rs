use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Parse)]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    use syn::{Fields, FieldsNamed, FieldsUnnamed, Data};
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let extends_code = match &input.data {
        Data::Struct(data) => {
            let items = if let Fields::Named(FieldsNamed {named, ..}) = &data.fields {
                named.iter().map(|f| {
                    let ident = &f.ident;
                    let ty = &f.ty;
                    quote!(
                        #ident: <#ty as Parse>::parse(cursor)?,
                    )
                })
            } else {
                return syn::Error::new(name.span(), "Only support named fields now").to_compile_error().into();
            };
            quote!(
                impl Parse for #name {
                    fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
                        Ok(Self {
                            #(#items)*
                        })
                    }
                }
            )
        }
        Data::Enum(data) => {
            let items = data.variants.iter().map(|variant| {
                if let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = &variant.fields {
                    let tys = unnamed.iter().map(|f| {
                        let ty = &f.ty;
                        quote!(
                            #ty
                        )
                    }).collect::<Vec<_>>();
                    let placeholders = (0..tys.len()).map(|i| {
                        let id = format_ident!("_{}", i);
                        quote!(#id)
                    }).collect::<Vec<_>>();
                    let ident = &variant.ident;
                    quote!(
                        loop {
                            let mut c = cursor.clone();
                            #(
                                let #placeholders = <#tys as Parse>::parse(&mut c);
                                if let Err(s) = #placeholders {
                                    if span.cmp(&s) == std::cmp::Ordering::Less {
                                        span = s;
                                    }
                                    break;
                                }
                            )*
                            *cursor = c;
                            return Ok(#name::#ident(#(#placeholders?),*));
                        }
                    )
                } else {
                    panic!("Only support unnamed fields now");
                }
            });
            quote!(
                impl Parse for #name {
                    fn parse(cursor: &mut Cursor) -> Result<Self, Span> {
                        let mut span = cursor.span();
                        #(#items)*
                        Err(span)
                    }
                }
            )
        }
        _ => {
            return syn::Error::new(name.span(), "Only support struct or enum now").to_compile_error().into();
        }
    };
    extends_code.into()
}

// #[proc_macro_derive(SemanticAnalyze, attributes(scope, reverse))]
// pub fn derive_semantic(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     todo!()
// }

#[proc_macro_derive(Node)]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let extend_code = quote! (
        impl Node for #name {
            fn span(&self) -> Span {
                self.span
            }
        }
    );
    extend_code.into()
}