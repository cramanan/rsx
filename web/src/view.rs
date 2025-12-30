use rsx::component::Children;
use smallvec::{SmallVec, smallvec};

use crate::node::ViewHtmlNode;

/// Represents a view tree.
///
/// Internally, this stores a list of nodes. This is the main type that is returned from
/// components.
pub struct View<T> {
    /// The nodes in the view tree.
    pub(crate) nodes: SmallVec<[T; 1]>,
}

impl<T> View<T> {
    /// Create a new blank view.
    pub fn new() -> Self {
        Self {
            nodes: SmallVec::new(),
        }
    }

    /// Create a new view with a single node.
    pub fn from_node(node: T) -> Self {
        Self {
            nodes: smallvec![node],
        }
    }

    /// Create a new view with multiple nodes.
    pub fn from_nodes(nodes: Vec<T>) -> Self {
        Self {
            nodes: nodes.into(),
        }
    }

    /// Create a new view from a function that returns a view. An alias to
    /// [`ViewNode::create_dynamic_view`].
    pub fn from_dynamic<U: Into<Self> + 'static>(f: impl FnMut() -> U + 'static) -> Self
    where
        T: ViewNode,
    {
        T::create_dynamic_view(f)
    }

    /// Create a flat list of all the web-sys nodes in the view.
    pub fn as_web_sys(&self) -> Vec<web_sys::Node>
    where
        T: ViewHtmlNode,
    {
        self.nodes
            .iter()
            .map(|node| node.as_web_sys().clone())
            .collect()
    }
}

impl<T> Default for View<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Children<Self>> for View<T> {
    fn from(children: Children<Self>) -> Self {
        children.call()
    }
}

impl<T> From<Vec<View<T>>> for View<T> {
    fn from(nodes: Vec<View<T>>) -> Self {
        View {
            nodes: nodes.into_iter().flat_map(|v| v.nodes).collect(),
        }
    }
}

impl<T> From<Option<View<T>>> for View<T> {
    fn from(node: Option<View<T>>) -> Self {
        node.unwrap_or_default()
    }
}

// impl<T: ViewNode, U: Clone + Into<Self>> From<ReadSignal<U>> for View<T> {
//     fn from(signal: ReadSignal<U>) -> Self {
//         (move || signal.get_clone()).into()
//     }
// }

// impl<T: ViewNode, U: Clone + Into<Self>> From<Signal<U>> for View<T> {
//     fn from(signal: Signal<U>) -> Self {
//         (*signal).into()
//     }
// }

// impl<T: ViewNode, U: Clone + Into<Self> + Into<MaybeDyn<U>>> From<MaybeDyn<U>> for View<T> {
//     fn from(value: MaybeDyn<U>) -> Self {
//         (move || value.get_clone()).into()
//     }
// }

/// A trait that should be implemented for anything that represents a node in the view tree (UI
/// tree).
///
/// Examples include `DomNode` and `SsrNode` which are used to render views to the browser DOM and
/// to a string respectively. This trait can be implemented for other types to create custom render
/// backends.
pub trait ViewNode: Into<View<Self>> + Sized + 'static {
    /// Appends a child to the node. Panics if the node is not an element or other node that can
    /// have children (e.g. text node).
    fn append_child(&mut self, child: Self);

    /// Append a view to this node. Since a view is just a list of nodes, this essentially appends
    /// every node in the view to this node.
    fn append_view(&mut self, view: View<Self>) {
        for node in view.nodes {
            self.append_child(node);
        }
    }

    /// Create a dynamic view from a function that returns a view.
    ///
    /// The returned view will no longer be a function and can be treated as a normal view and,
    /// e.g., appended as a child to another node.
    ///
    /// Some render backends may not support dynamic views (e.g. `SsrNode`). In that case, the
    /// default behavior is to simply evaluate the function as a static view.
    fn create_dynamic_view<U: Into<View<Self>> + 'static>(
        mut f: impl FnMut() -> U + 'static,
    ) -> View<Self> {
        f().into()
    }
}

// Implement `From` for all tuples of types that implement `Into<View<U>>`.
macro_rules! impl_from_tuple {
    ($($name:ident),*) => {
        paste::paste! {
            impl<U, $($name),*> From<($($name,)*)> for View<U>
            where
                $($name: Into<View<U>>),*
            {
                fn from(t: ($($name,)*)) -> Self {
                    let ($([<$name:lower>]),*) = t;
                    #[allow(unused_mut)]
                    let mut nodes = SmallVec::new();
                    $(
                        nodes.extend([<$name:lower>].into().nodes);
                    )*
                    View { nodes }
                }
            }
        }
    };
}

impl_from_tuple!(A, B, C);
