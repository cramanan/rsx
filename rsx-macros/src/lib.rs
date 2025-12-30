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
    event_listeners: HashMap<Ident, Expr>,
    children: Vec<Node>,
}

impl Parse for Node {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.parse::<Token![<]>().is_ok() {
            // Parse tag name
            let name = input.parse::<Ident>()?;

            // Parse attributes
            let (attributes, event_listeners) = {
                let mut attributes = HashMap::new();
                let mut event_listeners = HashMap::new();
                // While closing tokens are not reached
                while !input.peek(Token![>]) && !input.peek(Token![/]) {
                    // Parse attribute identifier
                    let name = input.parse::<Ident>()?;

                    match name.to_string().strip_prefix("on") {
                        Some(name) => {
                            input.parse::<Token![=]>()?;
                            let content;
                            braced!(content in input);
                            event_listeners
                                .insert(Ident::new(name, input.span()), content.parse::<Expr>()?);
                        }
                        None => {
                            // Check for duplicates to avoid overwriting
                            if attributes.contains_key(&name) {
                                return Err(input.error(format!("duplicate attribute: {}", name)));
                            }

                            // Parse attribute value
                            let value = if input.parse::<Token![=]>().is_ok() {
                                input.parse::<LitStr>()?
                            } else {
                                LitStr::new(&true.to_string(), proc_macro2::Span::call_site())
                            };
                            attributes.insert(name, value);
                        }
                    }
                }
                (attributes, event_listeners)
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
                event_listeners,
                children,
            }))
        } else if input.peek(Brace) {
            let content;
            braced!(content in input);
            Ok(Node::Expression(content.parse()?))
        } else {
            let mut text = Vec::new();

            while !input.is_empty() && !input.peek(Brace) && !input.peek(Token![<]) {
                let tt = input.parse::<proc_macro2::TokenTree>()?;
                text.push(tt.to_string());
            }

            Ok(Node::Text(LitStr::new(&text.join(" "), input.span())))
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

                let event_listeners = element.event_listeners.iter().map(|(name, expr)| {
                    let name = LitStr::new(&name.to_string(), name.span());
                    quote! {( String::from(#name), Box::new(#expr) as rsx::EventListener)}
                });

                let children = element
                    .children
                    .iter()
                    .map(|node| quote! { rsx::Element::from(#node) });

                tokens.extend(quote! {
                    rsx::HTMLElement {
                        name: String::from(#name),
                        attributes: std::collections::HashMap::from([#(#attributes),*]),
                        event_listeners: std::collections::HashMap::from([#(#event_listeners),*]),
                        children: vec![#(#children),*],
                    }
                });
            }
            Node::Text(text) => tokens.extend(quote! {String::from(#text)}),
            Node::Expression(expression) => tokens.extend(quote! {#expression}),
        }
    }
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let node = parse_macro_input!(input as Node);
    quote! {{ rsx::Element::from(#node) }}.into()
}
