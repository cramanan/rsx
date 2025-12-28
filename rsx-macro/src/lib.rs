use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, LitStr, Token, parse::Parse, parse_macro_input};

struct Node {
    name: Ident,
    attributes: HashMap<Ident, LitStr>,
}

impl Parse for Node {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;
        // Parse tag name
        let name: Ident = input.parse()?;

        let mut attributes = HashMap::new();
        while !input.peek(Token![>]) && !input.peek(Token![/]) {
            let attr_name = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;
            let value = input.parse::<LitStr>()?;
            attributes.insert(attr_name, value);
        }

        if input.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            input.parse::<Token![>]>()?;
            // self-closing
        } else {
            input.parse::<Token![>]>()?;
            // // Parse children until `</tag>`
            // let mut children = Vec::new();
            // while !input.peek(Token![<]) || !input.peek2(Token![/]) {
            //     children.push(Node::parse(input)?);
            // }

            // Parse closing tag
            input.parse::<Token![<]>()?;
            input.parse::<Token![/]>()?;
            let end_name: Ident = input.parse()?;
            input.parse::<Token![>]>()?;

            if name != end_name {
                return Err(input.error("mismatched closing tag"));
            }
        }

        Ok(Node { name, attributes })
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.name.to_string();
        let attributes = self.attributes.iter().map(|(name, value)| {
            let name = LitStr::new(&name.to_string(), name.span());
            dbg!(
                quote! {( String::from(#name), String::from(#value))}
                    .into_token_stream()
                    .to_string()
            );
            quote! {( String::from(#name), String::from(#value))}
        });
        // let children = &self.children;

        tokens.extend(quote! {
            rsx::Element::Element(rsx::HTMLElement {
                name: #name.to_string(),
                attributes: std::collections::HashMap::from([#(#attributes),*]),
                // children: vec![#(#children),*],
            })
        });
    }
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let node = parse_macro_input!(input as Node);

    quote::quote! {{ #node }}.into()
}
