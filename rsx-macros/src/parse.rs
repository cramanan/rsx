use proc_macro2::TokenTree;
use syn::{
    Ident, LitStr, Token, braced,
    parse::{Parse, ParseStream},
    token::Brace,
};

use crate::ir::{Node, Prop, PropType, Root, TagNode};

impl Parse for Root {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut children = Vec::new();
        while !input.is_empty() {
            children.push(input.parse()?);
        }
        Ok(Self(children))
    }
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![<]) {
            return Ok(Self::Tag(input.parse()?));
        }

        if input.peek(Brace) {
            let content;
            braced!(content in input);
            return Ok(Self::Dynamic(content.parse()?));
        }

        return Ok(Self::Text(input.parse::<TokenTree>()?.to_string()));
    }
}

fn is_closing_tag(input: ParseStream, name: &Ident) -> bool {
    let fork = input.fork();

    fork.parse::<Token![<]>().is_ok()
        && fork.parse::<Token![/]>().is_ok()
        && fork.parse::<Ident>().map(|id| id == *name).unwrap_or(false)
}
impl Parse for TagNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;
        let name = input.parse::<Ident>()?;
        let mut props = Vec::new();

        while !input.is_empty() && !input.peek(Token![>]) {
            props.push(input.parse()?);
        }

        input.parse::<Token![>]>()?;

        let mut children = Vec::new();
        // While we don't reach the closing tag
        while !is_closing_tag(input, &name) {
            children.push(input.parse()?);
        }

        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let span = input.fork();
        let closing_name = input.parse::<Ident>()?;
        input.parse::<Token![>]>()?;
        if name != closing_name {
            return Err(span.error(format!(
                "mismatched closing tag. Expected: {name}, got {closing_name}"
            )));
        }

        Ok(Self {
            name,
            props,
            children,
        })
    }
}

impl Parse for Prop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let identifier = input.parse::<Ident>()?.to_string();
        let prop_type = input.parse::<PropType>()?;

        Ok(Self {
            identifier,
            prop_type,
        })
    }
}

impl Parse for PropType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.parse::<Token![=]>().is_ok() {
            return Ok(Self::Boolean);
        };

        if let Ok(value) = input.parse::<LitStr>() {
            return Ok(Self::Plain { value });
        };

        if input.peek(Brace) {
            let content;
            braced!(content in input);
            return Ok(Self::Expression {
                value: content.parse()?,
            });
        };

        Err(input.error("unknown token"))
    }
}
