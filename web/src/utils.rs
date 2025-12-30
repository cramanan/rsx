/// Get all nodes between `start` and `end`.
///
/// If `end` is before `start`, all nodes after `start` will be returned.
///
/// The range is exclusive so `start` and `end` will not be included.
#[must_use]
pub fn get_nodes_between(start: &web_sys::Node, end: &web_sys::Node) -> Vec<web_sys::Node> {
    let parent = start.parent_node().unwrap();
    debug_assert_eq!(
        parent,
        end.parent_node().unwrap(),
        "parents of `start` and `end` do not match"
    );

    let mut nodes = Vec::new();

    let mut next = start.next_sibling();
    while let Some(current) = next {
        let tmp = current.next_sibling();
        if &current == end {
            break;
        } else {
            nodes.push(current);
        }
        next = tmp;
    }

    nodes
}
