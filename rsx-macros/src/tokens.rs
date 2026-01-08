use quote::{ToTokens, quote};

use crate::ir::{Node, Root, TagNode};

impl ToTokens for Root {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for child in &self.0 {
            ToTokens::to_tokens(child, tokens);
        }
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Node::Tag(tag_node) => ToTokens::to_tokens(tag_node, tokens),
            Node::Text(lit_str) => ToTokens::to_tokens(lit_str, tokens),
            Node::Dynamic(expr) => ToTokens::to_tokens(expr, tokens),
        }
    }
}

impl ToTokens for TagNode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.name.to_string();
        let mut attributes = Vec::new();
        let mut event_listeners = Vec::new();
        for prop in &self.props {
            let value = match &prop.prop_type {
                crate::ir::PropType::Plain { value } => quote! { #value },
                crate::ir::PropType::Expression { value } => quote! { #value },
                crate::ir::PropType::Boolean => quote! { "" },
            };
            let identifier = &prop.identifier;

            if let Some(identifier) = identifier.strip_prefix("on") {
                event_listeners.push(
                    quote! {(String::from(#identifier), Box::new(#value) as rsx::EventListener)},
                );
            } else {
                attributes.push(quote! {(String::from(#identifier),String::from(#value))});
            }
        }

        let children = self
            .children
            .iter()
            .map(|child| quote! {rsx::Element::from(#child)});

        tokens.extend(quote! { rsx::HTMLElement{
            name: String::from(#name),
            attributes: ::std::collections::HashMap::from_iter(::std::vec![#(#attributes),*]),
            event_listeners: ::std::collections::HashMap::from_iter(::std::vec![#(#event_listeners),*]),
            children: ::std::vec![#(#children),*]
        } });
    }
}
