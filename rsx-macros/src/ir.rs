// use proc_macro2::Span;
use syn::{Expr, Ident, LitStr};

pub(crate) struct Root(pub(crate) Vec<Node>);

pub(crate) enum Node {
    Tag(TagNode),
    Text(String),
    Dynamic(Expr),
}

pub(crate) struct TagNode {
    pub(crate) name: Ident,
    pub(crate) props: Vec<Prop>,
    pub(crate) children: Vec<Node>,
}

pub(crate) struct Prop {
    pub(crate) identifier: String,
    pub(crate) prop_type: PropType,
}

pub enum PropType {
    /// Syntax: `<name>="<expr>"`.
    Plain { value: LitStr },

    /// Syntax: `<name>={<expr>}`
    Expression { value: Expr },
    /// Syntax: `<name>`
    Boolean,
}
