use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Expr, Ident, LitStr, Token, braced, parse::Parse, parse_macro_input, token::Brace};

// Representation of RSX nodes
enum Node {
    Element(Element),
    Text(LitStr),
    Expression(Expr),
}

struct Element {
    name: Ident,
    attributes: HashMap<Ident, LitStr>,
    children: Vec<Node>,
}

impl Parse for Node {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.parse::<Token![<]>().is_ok() {
            // Parse tag name
            let name = input.parse::<Ident>()?;

            // Parse attributes
            let attributes = {
                let mut attributes = HashMap::new();
                // While closing tokens are not reached
                while !input.peek(Token![>]) && !input.peek(Token![/]) {
                    // Parse attribute identifier
                    let name = input.parse::<Ident>()?;

                    // Check for duplicates to avoid overwriting
                    if attributes.contains_key(&name) {
                        return Err(input.error(format!("duplicate attribute: {}", name)));
                    }

                    input.parse::<Token![=]>()?;
                    let value = input.parse::<LitStr>()?; // Parse attribute value
                    attributes.insert(name, value);
                }
                attributes
            };

            // Parse children depending on whether the tag is self-closing
            let children = if input.peek(Token![/]) {
                input.parse::<Token![/]>()?;
                input.parse::<Token![>]>()?;
                Vec::default()
            } else {
                input.parse::<Token![>]>()?;

                // Children handling
                let mut children = Vec::new();

                // While we don't reach the closing tag
                while !input.is_empty() && !(input.peek(Token![<]) && input.peek2(Token![/])) {
                    children.push(input.parse::<Node>()?); // Recursively parse child nodes
                }

                input.parse::<Token![<]>()?;
                input.parse::<Token![/]>()?;
                let closing_input = input.fork(); // highlight closing tag name
                let closing_name = input.parse::<Ident>()?;
                if name != closing_name {
                    return Err(closing_input.error(format!(
                        "mismatched closing tag. Expected: {name}, got {closing_name}"
                    )));
                }
                input.parse::<Token![>]>()?;
                children
            };

            Ok(Node::Element(Element {
                name,
                attributes,
                children,
            }))
        } else if input.peek(Brace) {
            let content;
            braced!(content in input);
            Ok(Node::Expression(content.parse()?))
        } else {
            let mut text = String::new();

            while !input.is_empty() && !input.peek(Brace) && !input.peek(Token![<]) {
                let tt = input.parse::<proc_macro2::TokenTree>()?;
                text.push_str(&tt.to_string());
            }

            Ok(Node::Text(LitStr::new(
                text.trim(),
                proc_macro2::Span::call_site(),
            )))
        }
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Node::Element(element) => {
                let name = element.name.to_string();
                let attributes = element.attributes.iter().map(|(name, value)| {
                    let name = LitStr::new(&name.to_string(), name.span());
                    quote! {( String::from(#name), String::from(#value))}
                });
                let children = element
                    .children
                    .iter()
                    .map(|c| quote! { Box::new(#c) as Box<dyn rsx::RSX> });

                tokens.extend(quote! {
                    rsx::HTMLElement {
                        name: String::from(#name),
                        attributes: std::collections::HashMap::from([#(#attributes),*]),
                        children: vec![#(#children),*],
                    }
                });
            }
            Node::Text(text) => tokens.extend(quote! {String::from(#text)}),
            Node::Expression(expression) => tokens.extend(quote! { {
                use rsx::RSX;
                #expression.to_string().render()
            } }),
        }
    }
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let node = parse_macro_input!(input as Node);
    quote! {{ #node }}.into()
}
