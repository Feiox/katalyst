#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

struct BindingTokens {
    pub ident: syn::Ident,
    pub brace: syn::token::Brace,
    pub methods: Punctuated<syn::ItemFn, Token![;]>,
}

impl Parse for BindingTokens {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let ident = input.parse()?;
        let brace = braced!(content in input);
        let methods = content.parse_terminated(syn::ItemFn::parse)?;

        Ok(BindingTokens {
            ident,
            brace,
            methods,
        })
    }
}

struct BindingAttrValues {
    pub ident: syn::Ident,
    pub equal: Token![=],
    pub val: syn::Lit,
}

impl Parse for BindingAttrValues {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(BindingAttrValues {
            ident: input.parse()?,
            equal: input.parse()?,
            val: input.parse()?,
        })
    }
}

struct BindingAttrParens {
    pub parens: syn::token::Paren,
    pub contents: Punctuated<BindingAttrValues, Token![,]>,
}

impl Parse for BindingAttrParens {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let parens = parenthesized!(content in input);
        let contents = content.parse_terminated(BindingAttrValues::parse)?;
        Ok(BindingAttrParens { parens, contents })
    }
}

#[proc_macro]
pub fn binding(input: TokenStream) -> TokenStream {
    let bindings: BindingTokens = syn::parse(input).expect("First parameter should be an identity");
    let mut ident = bindings.ident;
    let mut binding_name = ident.to_string();
    binding_name.push_str("Binding");
    let id_string = ident.to_string().to_ascii_lowercase();
    ident = syn::Ident::new(&binding_name, proc_macro2::Span::call_site());
    let mut fns = bindings.methods;
    let mut match_options = vec![];
    for fn_item in fns.iter_mut() {
        let fn_ident = &fn_item.ident;
        let fn_name = fn_ident.to_string();
        let mut checks = vec![];
        for attr in fn_item
            .attrs
            .iter()
            .filter(|a| !a.path.segments.is_empty() && a.path.segments[0].ident == "args")
        {
            let container: BindingAttrParens =
                syn::parse2(attr.tts.clone()).expect("First parameter should be an identity");
            for vals in container.contents.iter() {
                match &vals.val {
                    syn::Lit::Str(s) => println!("{:?} : {:?}", vals.ident, s.value()),
                    syn::Lit::Int(i) => {
                        let res = i.value() as usize;
                        if vals.ident == "count" {
                            checks.push(quote! {
                                if args.len() != #res {
                                    Err(KatalystError::ConfigParseError)?;
                                }
                            });
                        }
                    }
                    syn::Lit::Bool(b) => println!("{:?} : {:?}", vals.ident, b.value),
                    _ => (),
                };

                checks.push(quote! {
                    println!("Test");
                });
            }
        }
        fn_item.attrs.clear();
        match_options.push(quote! {
            #fn_name => {
                #(#checks)*
                Ok(std::sync::Arc::new(#ident::#fn_ident))
            },
        });
    }

    // Ok(std::sync::Arc::new(|_, _| "".to_string()))

    let result = quote! {
        pub struct #ident {}

        impl ExpressionBuilder for #ident {
            fn identifier(&self) -> &'static str {
                #id_string
            }

            fn make_fn(&self, name: &str, args: &[ExpressionArg]) -> Result<ExpressionRenderMethod, KatalystError> {
                match name {
                    #(#match_options)*
                    _ => Err(KatalystError::ConfigParseError)
                }
            }
        }

        impl #ident {
            #(#fns)*
        }
    };
    result.into()
}
