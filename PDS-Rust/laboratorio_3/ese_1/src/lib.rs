pub mod editor;
pub mod grep;
pub mod christmas_tree;


pub mod christmas_tree_test {
    use crate::christmas_tree::Albero;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            let tree = Albero::new();
            assert!(tree.children_map.contains_key("Root"));
            assert!(tree.switches.contains_key("Root"));
            assert_eq!(tree.children_map.get("Root").unwrap().len(), 0);
            assert_eq!(*tree.switches.get("Root").unwrap(), true);
            assert_eq!(tree.parent_map.len(), 0);
        }

        #[test]
        fn test_add() {
            let mut tree = Albero::new();

            // Add child to Root
            tree.add("Root", "Node1");
            assert!(tree.children_map.get("Root").unwrap().contains(&"Node1".to_string()));
            assert_eq!(tree.parent_map.get("Node1").unwrap(), "Root");
            assert_eq!(*tree.switches.get("Node1").unwrap(), false);

            // Add another child to Root
            tree.add("Root", "Node2");
            assert!(tree.children_map.get("Root").unwrap().contains(&"Node2".to_string()));
            assert_eq!(tree.parent_map.get("Node2").unwrap(), "Root");
            assert_eq!(*tree.switches.get("Node2").unwrap(), false);

            // Add child to Node1
            tree.add("Node1", "Node1_1");
            assert!(tree.children_map.get("Node1").unwrap().contains(&"Node1_1".to_string()));
            assert_eq!(tree.parent_map.get("Node1_1").unwrap(), "Node1");
            assert_eq!(*tree.switches.get("Node1_1").unwrap(), false);

            // Add to non-existent parent
            tree.add("NonExistent", "Node3");
            assert!(!tree.parent_map.contains_key("Node3"));
            assert!(!tree.switches.contains_key("Node3"));
        }

        #[test]
        fn test_remove() {
            let mut tree = Albero::new();

            // Setup tree
            tree.add("Root", "Node1");
            tree.add("Root", "Node2");
            tree.add("Node1", "Node1_1");
            tree.add("Node1", "Node1_2");
            tree.add("Node1_1", "Node1_1_1");

            // Remove a leaf node
            tree.remove("Node1_2");
            assert!(!tree.children_map.contains_key("Node1_2"));
            assert!(!tree.parent_map.contains_key("Node1_2"));
            assert!(!tree.switches.contains_key("Node1_2"));
            assert!(tree.children_map.get("Node1").unwrap().contains(&"Node1_1".to_string()));

            // Remove a branch (Node1 and all its descendants)
            tree.remove("Node1");
            assert!(!tree.children_map.contains_key("Node1"));
            assert!(!tree.parent_map.contains_key("Node1"));
            assert!(!tree.switches.contains_key("Node1"));
            assert!(!tree.children_map.contains_key("Node1_1"));
            assert!(!tree.parent_map.contains_key("Node1_1"));
            assert!(!tree.switches.contains_key("Node1_1"));
            assert!(!tree.children_map.contains_key("Node1_1_1"));
            assert!(!tree.parent_map.contains_key("Node1_1_1"));
            assert!(!tree.switches.contains_key("Node1_1_1"));

            // Node2 should still exist
            assert!(tree.parent_map.contains_key("Node2"));
        }

        #[test]
        fn test_toggle() {
            let mut tree = Albero::new();

            // Setup tree
            tree.add("Root", "Node1");

            // Toggle Node1 from false to true
            let result = tree.toggle("Node1");
            assert_eq!(result, Some(true));
            assert_eq!(*tree.switches.get("Node1").unwrap(), true);

            // Toggle Node1 from true to false
            let result = tree.toggle("Node1");
            assert_eq!(result, Some(false));
            assert_eq!(*tree.switches.get("Node1").unwrap(), false);

            // Try to toggle Root (should return None)
            let result = tree.toggle("Root");
            assert_eq!(result, None);
            assert_eq!(*tree.switches.get("Root").unwrap(), true);

            // Try to toggle non-existent node
            let result = tree.toggle("NonExistent");
            assert_eq!(result, None);
        }

        #[test]
        fn test_peek() {
            let mut tree = Albero::new();

            // Setup tree
            tree.add("Root", "Node1");
            tree.add("Node1", "Node1_1");
            tree.add("Node1_1", "Node1_1_1");

            // Root is always on
            assert_eq!(tree.peek("Root"), Some(true));

            // All other nodes should be off initially
            assert_eq!(tree.peek("Node1"), Some(false));
            assert_eq!(tree.peek("Node1_1"), Some(false));
            assert_eq!(tree.peek("Node1_1_1"), Some(false));

            // Turn on Node1
            tree.toggle("Node1");
            assert_eq!(tree.peek("Node1"), Some(true));
            assert_eq!(tree.peek("Node1_1"), Some(false));

            // Turn on Node1_1
            tree.toggle("Node1_1");
            assert_eq!(tree.peek("Node1_1"), Some(true));
            assert_eq!(tree.peek("Node1_1_1"), Some(false));

            // Turn on Node1_1_1
            tree.toggle("Node1_1_1");
            assert_eq!(tree.peek("Node1_1_1"), Some(true));

            // Turn off Node1, which should make all children appear off
            tree.toggle("Node1");
            assert_eq!(tree.peek("Node1"), Some(false));
            assert_eq!(tree.peek("Node1_1"), Some(false));
            assert_eq!(tree.peek("Node1_1_1"), Some(false));

            // Turn on Node1 again, and children should reflect their own state
            tree.toggle("Node1");
            assert_eq!(tree.peek("Node1"), Some(true));
            assert_eq!(tree.peek("Node1_1"), Some(true));
            assert_eq!(tree.peek("Node1_1_1"), Some(true));

            // Non-existent node should return None
            assert_eq!(tree.peek("NonExistent"), None);
        }

        #[test]
        fn test_complex_tree_operations() {
            let mut tree = Albero::new();

            // Build a more complex tree
            tree.add("Root", "A");
            tree.add("Root", "B");
            tree.add("A", "A1");
            tree.add("A", "A2");
            tree.add("B", "B1");
            tree.add("B", "B2");
            tree.add("A1", "A1a");
            tree.add("B2", "B2a");

            // Toggle some nodes on
            tree.toggle("A");
            tree.toggle("A1");
            tree.toggle("A1a");
            tree.toggle("B");
            tree.toggle("B2");

            // Check states
            assert_eq!(tree.peek("A"), Some(true));
            assert_eq!(tree.peek("A1"), Some(true));
            assert_eq!(tree.peek("A1a"), Some(true));
            assert_eq!(tree.peek("A2"), Some(false)); // A2 was not toggled on
            assert_eq!(tree.peek("B"), Some(true));
            assert_eq!(tree.peek("B1"), Some(false)); // B1 was not toggled on
            assert_eq!(tree.peek("B2"), Some(true));
            assert_eq!(tree.peek("B2a"), Some(false)); // B2a was not toggled on

            // Toggle B2a on
            tree.toggle("B2a");
            assert_eq!(tree.peek("B2a"), Some(true));

            // Remove B node and all its children
            tree.remove("B");

            // B and its descendants should no longer exist
            assert_eq!(tree.peek("B"), None);
            assert_eq!(tree.peek("B1"), None);
            assert_eq!(tree.peek("B2"), None);
            assert_eq!(tree.peek("B2a"), None);

            // A and its descendants should still exist with correct state
            assert_eq!(tree.peek("A"), Some(true));
            assert_eq!(tree.peek("A1"), Some(true));
            assert_eq!(tree.peek("A1a"), Some(true));
            assert_eq!(tree.peek("A2"), Some(false));
        }
    }
}