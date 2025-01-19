use indextree::{Arena, NodeId};
use itertools::{self, Itertools};

pub fn tree_string(root: &NodeId, arena: &Arena<crate::suree::CommandNode>) -> String {
    let mut tree = String::new();

    draw_tree_format(root, arena, &mut tree, "");

    tree
}

pub fn draw_tree_format(
    node: &NodeId,
    arena: &Arena<crate::suree::CommandNode>,
    tree: &mut String,
    indent: &str,
) {
    let mut sorted_children = node
        .children(arena)
        .sorted_by(|a, b| {
            let a_index = arena[*a].get().index;
            let b_index = arena[*b].get().index;
            a_index.cmp(&b_index)
        })
        .peekable();

    let s = format!("{}\n", arena[*node].get().command);
    tree.push_str(s.as_str());

    if sorted_children.peek().is_some() {
        while let Some(c) = sorted_children.next() {
            let is_last = sorted_children.peek().is_none();

            tree.push_str(format!("{}{}", indent, if is_last { "└── " } else { "├── " }).as_str());

            draw_tree_format(
                &c,
                arena,
                tree,
                format!("{}{}   ", indent, if is_last { " " } else { "│" }).as_str(),
            );
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::suree::CommandNode;
    use indextree::Arena;

    #[test]
    fn test_draw_tree_format() {
        let mut arena = Arena::new();
        let root = arena.new_node(CommandNode {
            index: 0,
            command: "root".to_string(),
            description: Some("".to_string()),
        });
        let child1 = arena.new_node(CommandNode {
            index: 0,
            command: "child1".to_string(),
            description: Some("".to_string()),
        });
        let child2 = arena.new_node(CommandNode {
            index: 1,
            command: "child2".to_string(),
            description: Some("".to_string()),
        });
        let grandchild1 = arena.new_node(CommandNode {
            index: 0,
            command: "grandchild1".to_string(),
            description: Some("".to_string()),
        });
        let grandchild2 = arena.new_node(CommandNode {
            index: 1,
            command: "grandchild2".to_string(),
            description: Some("".to_string()),
        });

        // reverse append order to check sorting by index
        root.append(child2, &mut arena);
        root.append(child1, &mut arena);
        child1.append(grandchild1, &mut arena);
        child1.append(grandchild2, &mut arena);

        let mut result = String::new();
        draw_tree_format(&root, &arena, &mut result, "");

        let expected_output = "\
root
├── child1
│   ├── grandchild1
│   └── grandchild2
└── child2
";

        assert_eq!(result, expected_output);
    }
}
