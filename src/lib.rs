//
// Libraries
//
use std::collections::HashMap;
use std::fmt::Debug;
use std::ptr::eq;

use serde::{Deserialize, Serialize};
//
// Tests
//
#[cfg(test)]
mod test {
    use super::*;
    use std::ptr::eq;

    struct ExampleTree {}
    impl ExampleTree {
        pub fn get_node_root_of_tree_new() -> Node {
            let mut node_root = Node::new();

            let slice_of_paths: [Box<[&str]>; 7] = [
                Box::new(["a0"]),
                Box::new(["b0", "b1", "b2", "b3"]),
                Box::new(["c0", "ca1", "ca2", "ca3", "ca4", "ca5"]),
                Box::new(["c0", "cb1", "cb2", "cb3", "cc4", "cd5"]),
                Box::new([
                    "d0", "da1", "da2", "da3", "da4", "da5", "da6", "da7", "da8", "da9",
                ]),
                Box::new([
                    "d0", "da1", "da2", "db3", "db4", "db5", "db6", "db7", "db8", "db9",
                ]),
                Box::new([
                    "d0", "da1", "da2", "db3", "db4", "db5", "dc6", "dc7", "dd8", "dd9",
                ]),
            ];

            for item_path in slice_of_paths {
                let mut item_node_child = Node::new();

                let item_string_data = ExampleTree::get_string_test_data_via_path(&item_path);

                item_node_child.field_string_data = item_string_data;

                match node_root.insert_node_at_path(&item_path, item_node_child) {
                    Ok(()) => {}
                    Err(err) => panic!("{}", err),
                };
            }
            node_root
        }

        /*
        // Returns a vec of known paths within the generic complex tree
        pub fn get_vec_of_paths_expected() -> Vec<Vec<String>> {
            let slice_of_paths: [Box<[&str]>; 37] = [
                Box::new( [ "a0" ] ),
                Box::new( [ "b0" ] ),
                Box::new( [ "b0", "b1" ] ),
                Box::new( [ "b0", "b1", "b2" ] ),
                Box::new( [ "b0", "b1", "b2", "b3" ] ),
                Box::new( [ "c0" ] ),
                Box::new( [ "c0", "ca1" ] ),
                Box::new( [ "c0", "ca1", "ca2" ] ),
                Box::new( [ "c0", "ca1", "ca2", "ca3" ] ),
                Box::new( [ "c0", "ca1", "ca2", "ca3", "ca4" ] ),
                Box::new( [ "c0", "ca1", "ca2", "ca3", "ca4", "ca5" ] ),
                Box::new( [ "c0", "cb1" ] ),
                Box::new( [ "c0", "cb1", "cb2" ] ),
                Box::new( [ "c0", "cb1", "cb2", "cb3" ] ),
                Box::new( [ "c0", "cb1", "cb2", "cb3", "cc4" ] ),
                Box::new( [ "c0", "cb1", "cb2", "cb3", "cc4", "cd5" ] ),
                Box::new( [ "d0" ] ),
                Box::new( [ "d0", "da1" ] ),
                Box::new( [ "d0", "da1", "da2" ] ),
                Box::new( [ "d0", "da1", "da2", "da3" ] ),
                Box::new( [ "d0", "da1", "da2", "da3", "da4" ] ),
                Box::new( [ "d0", "da1", "da2", "da3", "da4", "da5" ] ),
                Box::new( [ "d0", "da1", "da2", "da3", "da4", "da5", "da6" ] ),
                Box::new( [ "d0", "da1", "da2", "da3", "da4", "da5", "da6", "da7" ] ),
                Box::new( [ "d0", "da1", "da2", "da3", "da4", "da5", "da6", "da7", "da8" ] ),
                Box::new( [ "d0", "da1", "da2", "da3", "da4", "da5", "da6", "da7", "da8", "da9" ] ),
                Box::new( [ "d0", "da1", "da2", "db3" ] ),
                Box::new( [ "d0", "da1", "da2", "db3", "db4" ] ),
                Box::new( [ "d0", "da1", "da2", "db3", "db4", "db5" ] ),
                Box::new( [ "d0", "da1", "da2", "db3", "db4", "db5", "db6" ] ),
                Box::new( [ "d0", "da1", "da2", "db3", "db4", "db5", "db6", "db7" ] ),
                Box::new( [ "d0", "da1", "da2", "db3", "db4", "db5", "db6", "db7", "db8" ] ),
                Box::new( [ "d0", "da1", "da2", "db3", "db4", "db5", "db6", "db7", "db8", "db9" ] ),
                Box::new( [ "d0", "da1", "da2", "db3", "db4", "db5", "dc6" ] ),
                Box::new( [ "d0", "da1", "da2", "db3", "db4", "db5", "dc6", "dc7" ] ),
                Box::new( [ "d0", "da1", "da2", "db3", "db4", "db5", "dc6", "dc7", "dd8" ] ),
                Box::new( [ "d0", "da1", "da2", "db3", "db4", "db5", "dc6", "dc7", "dd8", "dd9" ] ),
            ];
            slice_of_paths.iter().map(|item| {item.iter().map(|item|{item.to_string()}).collect::<Vec<String>>()}).collect()
        }
        */
        //
        // Public - get - path
        //
        // Returns a path generated from a given node's expected test data
        pub fn get_path_from_test_data(arg_string_test_data: &str) -> Vec<String> {
            arg_string_test_data[ExampleTree::get_string_prefix_test_data().len()..]
                .split("_")
                .map(|item| item.to_string())
                .collect()
        }
        //
        // Public - get - string
        //
        // This provides a hard-coded prefix for consistency across all functions that depend on it
        fn get_string_prefix_test_data() -> String {
            "test_data_".to_string()
        }

        // Returns test data without the prefix
        //pub fn get_string_test_data_without_prefix( arg_string: &str ) -> String { arg_string[ ExampleTree::get_string_prefix_test_data().len().. ].to_string() }

        // Returns test data compiled from the path argument
        pub fn get_string_test_data_via_path(arg_path: &[&str]) -> String {
            [
                ExampleTree::get_string_prefix_test_data(),
                arg_path.join("_"),
            ]
            .join("")
        }
        //
        // Public - get - vec
        //
        //
        // This is a convenience function for writing out a 'quick' slice of boxed paths that
        // won't trigger an error due to each path being a different size.
        // I'm leaving this here for the sanity of anyone who wants to setup custom tests.
        /*
            // Example output:
            let slice_of_paths: [Box<[&str]>; 5] = [
                Box::new( [ "a0" ] ),
                Box::new( [ "b0" ] ),
                Box::new( [ "b0", "b1" ] ),
                Box::new( [ "b0", "b1", "b2" ] ),
                Box::new( [ "b0", "b1", "b2", "b3" ] ),
            ];
        */
        /*
        pub fn get_string_of_rust_code_to_support_rapid_creation_of_test_cases( arg_vec_of_paths: &Vec<Vec<String>> ) -> String {
            let mut vec_to_return = vec![ format!( "let slice_of_paths: [Box<[&str]>; {}] = [", arg_vec_of_paths.len(), ) ];
            for item_path in arg_vec_of_paths {
                let item_path_string = format!( "    Box::new( [ \"{}\" ] ),", item_path.join( "\", \"" ) );
                vec_to_return.push( item_path_string )
            }
            vec_to_return.push( "];".to_string() );
            vec_to_return.join( "\n" )
        }
        */
    }
    //
    // Test - get
    //
    #[test]
    fn test_get_count_of_node_children() {
        let mut node_root = Node::new();
        for item_key in ["a", "b", "c"] {
            match node_root.insert_node_at_key(item_key, Node::new()) {
                Ok(()) => {}
                Err(err) => panic!("{}", err),
            };
        }
        assert_eq!(node_root.get_count_of_node_children(), 3 as usize);
    }

    #[test]
    fn test_get_count_of_leaves() {
        let node_root = ExampleTree::get_node_root_of_tree_new();
        assert_eq!(node_root.get_count_of_leaves(), 7)
    }

    #[test]
    fn test_get_key_to_node() {
        let mut node_root = Node::new();

        let slice_of_keys = ["a", "b", "c"];

        for item_key in slice_of_keys {
            let mut item_node_child = Node::new();
            let item_test_data = ["test", item_key].join("_");
            item_node_child.field_string_data = item_test_data;
            match node_root.insert_node_at_key(item_key, item_node_child) {
                Ok(()) => {}
                Err(err) => panic!("{}", err),
            }
        }

        let node_target = match node_root.get_node_at_key("b") {
            Some(result) => result,
            None => panic!("Failed to get node."),
        };

        let key_result = match node_root.get_key_to_node(&node_target) {
            Some(result) => result,
            None => panic!("Failed to get key."),
        };

        assert_eq!(key_result, "b".to_string())
    }

    #[test]
    fn test_get_node_mute_at_key() {
        let mut node_root = Node::new();
        let node_child = Node::new();
        match node_root.insert_node_at_key("a", node_child) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        };

        let node_mut = match node_root.get_node_mut_at_key("a") {
            Some(result) => result,
            None => panic!("Failed to get node at key"),
        };

        assert_eq!(node_mut.field_string_data, "".to_string());

        let data_expected_after_update = "TEST";
        node_mut.field_string_data = data_expected_after_update.to_string();
        assert_eq!(node_mut.field_string_data, data_expected_after_update);
    }

    #[test]
    fn test_get_node_parent() {
        let node_root = ExampleTree::get_node_root_of_tree_new();

        let path = ["b0", "b1", "b2", "b3"];

        let node_child_of_parent = match node_root.get_node_at_path_or_error(&path) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        let node_parent_result = match node_root.get_node_parent(&node_child_of_parent) {
            Some(result) => result,
            None => panic!("Failed to get node parent."),
        };

        let path_to_parent = &path[0..path.len() - 1];

        let node_parent_expected = match node_root.get_node_at_path_or_error(&path_to_parent) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        assert!(eq(node_parent_result, node_parent_expected))
    }

    #[test]
    fn test_get_path_to_node() {
        let node_root = ExampleTree::get_node_root_of_tree_new();

        let path_expected = ["c0", "ca1", "ca2", "ca3", "ca4", "ca5"];

        let node_child = match node_root.get_node_at_path_or_error(&path_expected) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        let path_to_node = match node_root.get_path_to_node(&node_child) {
            Some(result) => result,
            None => panic!("Failed to get node"),
        };

        println!("path = {:?}", path_to_node);

        assert_eq!(
            path_to_node,
            path_expected
                .iter()
                .map(|item| { item.to_string() })
                .collect::<Vec<String>>()
        )
    }

    #[test]
    fn test_get_vec_of_data_along_path() {
        let path = ["a", "b", "c"];
        let data = "test";

        let mut node_root = Node::new();

        let mut node_child = Node::new();
        node_child.field_string_data = data.to_string();

        match node_root.insert_node_at_path(&path, node_child) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        };

        let vec_result = match node_root.get_vec_of_nodes_along_path(&path) {
            Some(result) => result,
            None => panic!("Failed to return vec of data with correct path."),
        }
        .iter()
        .map(|item_node| item_node.field_string_data.clone())
        .collect::<Vec<String>>();

        assert_eq!(
            vec_result,
            ["", "", data]
                .iter()
                .map(|item| { item.to_string() })
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_get_vec_of_node_leaves() {
        let mut node_root = Node::new();

        let mut node_child = Node::new();
        let string_data = "node_leaf";

        node_child.field_string_data = string_data.to_string();

        match node_root.insert_node_at_path(&["1", "2", "3"], node_child) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        };

        let vec_of_node_leaves = node_root.get_vec_of_node_leaves();

        let node_referenced = match vec_of_node_leaves.get(0) {
            Some(result) => result,
            None => panic!("No node leaves detected."),
        };
        assert_eq!(node_referenced.field_string_data, string_data)
    }

    #[test]
    fn test_get_vec_of_nodes_satisfying_callback() {
        let node_root = ExampleTree::get_node_root_of_tree_new();

        let string_test_data_expected = "test_data_b0_b1_b2_b3";

        let vec_of_nodes = node_root.get_vec_of_nodes_satisfying_callback(|arg_node| {
            arg_node.field_string_data == string_test_data_expected.to_string()
        });

        assert_eq!(vec_of_nodes.len(), 1);

        let node_result = match vec_of_nodes.get(0) {
            Some(result) => result,
            None => panic!("Error: Failed to get path."),
        };

        assert_eq!(node_result.field_string_data, string_test_data_expected)
    }

    #[test]
    fn test_get_vec_of_pairs_paths_and_nodes_with_data_satisfying_callback_sorted() {
        let node_root = ExampleTree::get_node_root_of_tree_new();

        let string_test_data_expected = "test_data_b0_b1_b2_b3";

        let vec_of_pairs_paths_and_nodes = node_root
            .get_vec_of_pairs_paths_and_nodes_with_data_satisfying_callback_sorted(|arg_node| {
                arg_node.field_string_data == string_test_data_expected.to_string()
            });

        assert_eq!(vec_of_pairs_paths_and_nodes.len(), 1);

        let (path_result, _node_result) = match vec_of_pairs_paths_and_nodes.get(0) {
            Some(result) => result,
            None => panic!("Error: Failed to get path."),
        };

        let path_expected = ExampleTree::get_path_from_test_data(&string_test_data_expected);

        assert_eq!(path_result.to_vec(), path_expected)
    }

    #[test]
    fn test_get_vec_of_paths_to_nodes_satisfying_callback_sorted() {
        let node_root = ExampleTree::get_node_root_of_tree_new();

        let string_test_data_expected = "test_data_b0_b1_b2_b3";

        let vec_of_paths =
            node_root.get_vec_of_paths_to_nodes_satisfying_callback_sorted(|arg_node| {
                arg_node.field_string_data == string_test_data_expected.to_string()
            });

        assert_eq!(vec_of_paths.len(), 1);

        let path_result = match vec_of_paths.get(0) {
            Some(result) => result,
            None => panic!("Error: Failed to get path."),
        };

        let path_expected = ExampleTree::get_path_from_test_data(&string_test_data_expected);

        assert_eq!(path_result.to_vec(), path_expected)
    }

    #[test]
    fn test_get_vec_of_nodes_along_path() {
        let path = ["a", "b", "c"];
        let data = "test";

        let mut node_root = Node::new();

        let mut node_child = Node::new();
        node_child.field_string_data = data.to_string();

        match node_root.insert_node_at_path(&path, node_child) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        };

        let vec_of_nodes_result = match node_root.get_vec_of_nodes_along_path(&path) {
            Some(result) => result,
            None => panic!("Failed to get vec of nodes."),
        };

        let vec_result = vec_of_nodes_result
            .iter()
            .map(|item_node| item_node.field_string_data.as_str())
            .collect::<Vec<&str>>();
        let vec_expected = ["", "", data]
            .iter()
            .map(|item| *item)
            .collect::<Vec<&str>>();
        assert_eq!(vec_result, vec_expected)
    }

    #[test]
    fn test_get_vec_of_nodes_in_tree() {
        let path = ["a", "b", "c"];
        let data = "test";

        let mut node_root = Node::new();

        let mut node_child = Node::new();
        node_child.field_string_data = data.to_string();

        match node_root.insert_node_at_path(&path, node_child) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        };

        let vec_of_strings = node_root
            .get_vec_of_nodes_in_tree()
            .iter()
            .map(|item| item.field_string_data.as_str())
            .collect::<Vec<&str>>();

        assert_eq!(vec_of_strings, ["", "", data].to_vec())
    }

    #[test]
    fn test_get_vec_of_paths_in_tree() {
        let mut node_root = Node::new();

        let slice_of_string_data = [
            "test_a",
            "test_a_b",
            "test_a_b_c_d_e",
            "test_aa_ab_ac",
            "test_ba_bb_bc_bd",
        ];

        let slice_of_paths: [Box<[&str]>; 5] = [
            Box::new(["a"]),
            Box::new(["a", "b"]),
            Box::new(["a", "b", "c", "d", "e"]),
            Box::new(["aa", "ab", "ac"]),
            Box::new(["ba", "bb", "bc", "bd"]),
        ];

        for item_index in 0..slice_of_string_data.len() {
            let mut item_node_child = Node::new();
            item_node_child.field_string_data = slice_of_string_data[item_index].to_string();

            match node_root.insert_node_at_path(&*slice_of_paths[item_index], item_node_child) {
                Ok(()) => {}
                Err(err) => panic!("{}", err),
            };
        }
        //
        // Reminder: Trying to turn 'expected' right off the bat throws a compile error
        //
        let expected: [Box<[&str]>; 12] = [
            Box::new(["a"]),
            Box::new(["a", "b"]),
            Box::new(["a", "b", "c"]),
            Box::new(["a", "b", "c", "d"]),
            Box::new(["a", "b", "c", "d", "e"]),
            Box::new(["aa"]),
            Box::new(["aa", "ab"]),
            Box::new(["aa", "ab", "ac"]),
            Box::new(["ba"]),
            Box::new(["ba", "bb"]),
            Box::new(["ba", "bb", "bc"]),
            Box::new(["ba", "bb", "bc", "bd"]),
        ];

        let vec_expected = expected
            .iter()
            .map(|item| (*item).iter().map(|item| *item).collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        assert_eq!(node_root.get_vec_of_paths_in_tree_sorted(), vec_expected);
    }
    //
    // Test - insert
    //
    #[test]
    fn test_insert_node_at_key() {
        // Functions covered...
        // - add_node_at_key
        // - set_string_data

        let key = "a";
        let data = "test";

        let mut node_root = Node::new();

        let mut node_child = Node::new();
        node_child.field_string_data = data.to_string();

        match node_root.insert_node_at_key(key, node_child) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        };

        let node_at_key = match node_root.get_node_at_key(key) {
            Some(result) => result,
            None => panic!("Error: Failed to get node."),
        };

        assert_eq!(node_at_key.field_string_data, data.to_string());

        let result_node = match node_root.get_node_at_key(key) {
            Some(result) => result,
            None => panic!("Failed to get node at key."),
        };
        assert_eq!(result_node.field_string_data, data.to_string())
    }

    #[test]
    fn test_insert_node_at_path() {
        let path = ["a", "b", "c"];

        let data = "test";

        let mut node_root = Node::new();
        let mut node_child = Node::new();

        node_child.field_string_data = data.to_string();

        match node_root.insert_node_at_path(&path, node_child) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        };

        let node_at_path = match node_root.get_node_at_path_or_error(&path) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(node_at_path.field_string_data, data.to_string())
    }
    //
    // Test - pop
    //
    #[test]
    fn test_pop_all_children_and_return_vec_of_pairs_keys_and_child_nodes() {
        let slice_of_keys = ["a", "b", "c"];

        let str_prefix_data = "test_data_";

        let mut vec_of_node_data_expected = Vec::new();
        let mut node_root = Node::new();
        for item_key in slice_of_keys {
            let item_string_test_data = [str_prefix_data, item_key].join("");

            let mut node_child = Node::new();

            // Capture the string here for testing later
            // Clone the string here so the compiler won't complain when it drops from memory
            vec_of_node_data_expected.push(item_string_test_data.clone());

            node_child.field_string_data = item_string_test_data;

            // node_root won't leave its 'borrowed as mut' state until the loop ends completely
            match node_root.insert_node_at_key(item_key, node_child) {
                Ok(()) => {}
                Err(err) => panic!("{}", err),
            };
        }

        vec_of_node_data_expected.sort();

        let vec_of_pairs_result =
            match node_root.pop_all_children_and_return_vec_of_pairs_keys_and_child_nodes() {
                Some(result) => result,
                None => panic!(
                    "Failed: pop_all_children_and_return_vec_of_pairs_keys_and_child_nodes()"
                ),
            };
        //
        // Validate keys
        //
        let mut vec_of_keys_result = vec_of_pairs_result
            .iter()
            .map(|(item_key, _item_node)| item_key.as_str())
            .collect::<Vec<&str>>();
        vec_of_keys_result.sort_by(|item_left, item_right| item_left.cmp(item_right));

        let vec_of_keys_expected = slice_of_keys
            .iter()
            .map(|item| *item)
            .collect::<Vec<&str>>();

        assert_eq!(vec_of_keys_result, vec_of_keys_expected,);

        let mut vec_of_test_data_result = vec_of_pairs_result
            .iter()
            .map(|(_item_key, item_node)| item_node.field_string_data.clone())
            .collect::<Vec<String>>();

        vec_of_test_data_result.sort();

        // This has to be a String comparison because the initially generated values drop from memory in the loop
        assert_eq!(vec_of_test_data_result, vec_of_node_data_expected)
    }

    #[test]
    fn test_pop_node_at_key() {
        let key = "test";
        let data = "test_data";

        let mut node_root = Node::new();

        let mut node_child = Node::new();
        node_child.field_string_data = data.to_string();

        match node_root.insert_node_at_key(key, node_child) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        };

        node_child = match node_root.pop_node_at_key(key) {
            Some(result) => result,
            None => panic!("Failed to pop node at key: {}", key),
        };

        if node_root.has_key(key) {
            panic!("Error: node_root still has key: {}", key)
        }

        assert_eq!(node_child.field_string_data, data.to_string())
    }

    #[test]
    fn test_pop_node_at_path() {
        let data = "test_data";

        let path = ["a", "b", "c"];

        let mut node_root = Node::new();

        let mut node_child = Node::new();
        node_child.field_string_data = data.to_string();
        match node_root.insert_node_at_path(&path, node_child) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        };

        node_child = match node_root.pop_node_at_path(&path) {
            Some(result) => result,
            None => panic!("Failed to pop node at path: {:?}", &path),
        };

        if node_root.has_path(&path) {
            panic!("Error: node_root still has path: {:?}", &path)
        }

        assert_eq!(node_child.field_string_data, data.to_string())
    }

    #[test]
    fn test_pop_node_at_path_or_error() {
        let mut node_root = ExampleTree::get_node_root_of_tree_new();

        let path = ["b0", "b1", "b2", "b3"];

        assert_eq!(node_root.has_path(&path), true);

        let node_popped = match node_root.pop_node_at_path_or_error(&path) {
            Ok(result) => result,
            Err(err) => panic!("{}", err,),
        };

        assert_eq!(node_root.has_path(&path), false);

        let path_to_parent = &path[0..path.len() - 1];

        assert_eq!(node_root.has_path(&path_to_parent), true);

        let node_parent_expected = match node_root.get_node_at_path_or_error(&path_to_parent) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        assert_eq!(node_parent_expected.has_key(path[path.len() - 1]), false);

        assert_eq!(
            node_popped.field_string_data,
            ExampleTree::get_string_test_data_via_path(&path)
        );
    }

    #[test]
    fn test_pop_node_at_key_and_promote_its_children() {
        let mut node_root = Node::new();

        let key_to_pop = "a";

        let slice_of_keys = [key_to_pop, "b", "c"];
        let slice_of_keys_tier_two = ["aa", "ab", "ac"];

        let str_prefix_data = "test_";

        for item_key in slice_of_keys {
            let mut item_child = Node::new();
            item_child.field_string_data = [str_prefix_data, item_key].join("");

            match node_root.insert_node_at_key(item_key, item_child) {
                Ok(()) => {}
                Err(err) => panic!("{}", err),
            };
        }

        let node_root_child = match node_root.get_node_mut_at_key("a") {
            Some(result) => result,
            None => panic!("Failed to get child node."),
        };
        for item_key in slice_of_keys_tier_two {
            let mut item_child = Node::new();
            item_child.field_string_data = [str_prefix_data, item_key].join("");

            match node_root_child.insert_node_at_key(item_key, item_child) {
                Ok(()) => {}
                Err(err) => panic!("{}", err),
            };
        }

        let _node_popped =
            match node_root.pop_node_at_key_and_promote_its_children("a", "_promoted") {
                Ok(result) => result,
                Err(err) => panic!("{}", err,),
            };

        let expected = ["aa_promoted", "ab_promoted", "ac_promoted", "b", "c"]
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        assert_eq!(node_root.get_vec_of_keys_sorted(), expected);

        let expected_filtered = ["aa_promoted", "ab_promoted", "ac_promoted"];

        assert_eq!(
            node_root.get_vec_of_keys_all_ending_with_suffix_sorted("_promoted"),
            expected_filtered
        )
    }

    #[test]
    fn test_pop_node_at_path_and_promote_its_children() {
        let mut node_root = Node::new();

        let path = ["a", "b", "c"];

        let path_parent_of_popped_node = ["a", "b"];

        let keys_for_child_at_path = ["ca", "cb", "cc"];

        let str_prefix_data = "test_";

        let item_child = Node::new();
        match node_root.insert_node_at_path(&path, item_child) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        };

        let node_root_child = match node_root.get_node_mut_at_path(&path) {
            Some(result) => result,
            None => panic!("Failed to get child node."),
        };

        for item_key in keys_for_child_at_path {
            let mut item_child = Node::new();
            item_child.field_string_data = [str_prefix_data, item_key].join("");

            match node_root_child.insert_node_at_key(item_key, item_child) {
                Ok(()) => {}
                Err(err) => panic!("{}", err),
            };
        }

        let _node_popped =
            match node_root.pop_node_at_path_and_promote_its_children(&path, "_promoted") {
                Ok(result) => result,
                Err(err) => panic!("{}", err,),
            };

        let node_parent = match node_root.get_node_at_path_or_error(&path_parent_of_popped_node) {
            Ok(result) => result,
            Err(err) => panic!("{}", err),
        };

        let expected = ["ca_promoted", "cb_promoted", "cc_promoted"]
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        assert_eq!(node_parent.get_vec_of_keys_sorted(), expected)
    }
    //
    // Test - raise
    //
    #[test]
    fn test_raise_error_if_path_is_invalid() {
        let node_root = Node::new();

        let path_valid = ["a", "b", "c"];
        match node_root.raise_error_if_path_is_invalid(&path_valid) {
            Ok(()) => {}
            Err(err) => panic!("{}", err),
        };

        let path_invalid = ["a", "b", ""];
        match node_root.raise_error_if_path_is_invalid(&path_invalid) {
            Ok(()) => {
                panic!("raise_error_if_path_is_invalid() failed to raise error on invalid path.")
            }
            Err(_err) => {}
        }
    }
}
//
// Public
//
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    //
    // Fields
    //
    /// Stores any within node
    pub field_string_data: String,
    // Reminders:
    // - References won't work here because Rust limits mutable borrows to *one*
    // - &str seems to get Rust to demand Node also becomes a reference (moves are *absolutely* necessary to avoid 'borrow hell')
    pub field_hash_map_children: HashMap<String, Node>,
}

impl Node {
    //
    // Public - clear
    //
    /// This removes all children without returning anything
    // No test needed, simple wrapper
    pub fn clear_children(&mut self) {
        self.field_hash_map_children.clear()
    }
    //
    // Public - get - count
    //
    /// Returns usize count for leaves
    // Test: test_get_count_of_leaves()
    pub fn get_count_of_leaves(&self) -> usize {
        let mut int_count_to_return = 0;
        let mut stack = self
            .field_hash_map_children
            .values()
            .collect::<Vec<&Node>>();
        loop {
            let item_node = match stack.pop() {
                Some(result) => result,
                None => break,
            };
            if item_node.field_hash_map_children.is_empty() {
                int_count_to_return += 1;
            } else {
                stack.extend(item_node.field_hash_map_children.values())
            }
        }
        int_count_to_return
    }

    /// Returns a usize value representing the number of children this node possesses
    // No test needed, simple wrapper
    pub fn get_count_of_node_children(&self) -> usize {
        self.field_hash_map_children.values().len()
    }
    //
    // Public - get - node
    //
    /// Returns the node reference; or None
    /// # Arguments
    /// * arg_key: Key referring to the child node
    // Test: test_add_node_at_key()
    pub fn get_node_at_key(&self, arg_key: &str) -> Option<&Node> {
        Some(self.field_hash_map_children.get(&arg_key.to_string())?)
    }

    /// Returns the node referenced; or None
    /// # Arguments
    /// * arg_path: Path referring to node within sub-tree
    // Test: test_insert_node_at_path()
    pub fn get_node_at_path(&self, arg_path: &[&str]) -> Option<&Node> {
        let mut node_to_return = self;
        for item_key in arg_path {
            node_to_return = node_to_return
                .field_hash_map_children
                .get(&item_key.to_string())?
        }
        Some(node_to_return)
    }

    /// Returns the node referenced; or an error explaining where the path failed
    /// # Arguments
    /// * arg_path: Path referring to node within sub-tree
    // Test: test_insert_node_at_path()
    pub fn get_node_at_path_or_error(&self, arg_path: &[&str]) -> Result<&Node, String> {
        let mut vec_path_that_exists = Vec::new();
        let mut node_to_return = self;
        for item_key in arg_path {
            match node_to_return
                .field_hash_map_children
                .get(&item_key.to_string())
            {
                Some(node_result) => {
                    node_to_return = node_result;
                    vec_path_that_exists.push(*item_key);
                }
                None => {
                    return Err(Node::raise_error_because_path_failed(
                        item_key,
                        &arg_path,
                        &vec_path_that_exists,
                    ));
                }
            }
        }
        Ok(node_to_return)
    }
    //
    // Public - get - node mut
    //
    /// Returns the node referenced as a mutable; or None
    /// # Arguments
    /// * arg_key: Key referring to child node
    // Test: test_get_node_mute_at_key()
    pub fn get_node_mut_at_key(&mut self, arg_key: &str) -> Option<&mut Node> {
        Some(self.field_hash_map_children.get_mut(&arg_key.to_string())?)
    }

    /// Returns the node referenced as a mutable; or None
    /// # Arguments
    /// * arg_path: Path referring to node within sub-tree
    // Test: test_pop_node_at_path_and_promote_its_children()
    pub fn get_node_mut_at_path(&mut self, arg_path: &[&str]) -> Option<&mut Node> {
        let mut node_to_return = self;
        for item_key in arg_path {
            node_to_return = node_to_return
                .field_hash_map_children
                .get_mut(&item_key.to_string())?
        }
        Some(node_to_return)
    }

    /// Returns the node referenced as a mutable; or an error message explaining where the path failed
    /// # Arguments
    /// * arg_path: Path referring to node within sub-tree
    // Test: test_pop_node_at_path_and_promote_its_children()
    pub fn get_node_mut_at_path_or_error(
        &mut self,
        arg_path: &[&str],
    ) -> Result<&mut Node, String> {
        match arg_path.len() {
            0 => return Err("Error: arg_path is empty.".to_string()),
            1 => {
                match self.get_node_mut_at_key(arg_path[0]) {
                    Some(result) => return Ok(result),
                    None => {
                        return Err([
                            "Error: arg_path does not exist.".to_string(),
                            format!("arg_path = {:?}", arg_path),
                        ]
                        .join("\n"))
                    }
                };
            }
            _ => {
                let mut vec_path_that_exists = Vec::new();
                let mut node_to_return = self;
                for item_key in arg_path {
                    match node_to_return
                        .field_hash_map_children
                        .get_mut(&item_key.to_string())
                    {
                        Some(node_result) => {
                            vec_path_that_exists.push(*item_key);
                            node_to_return = node_result
                        }
                        None => {
                            return Err(Node::raise_error_because_path_failed(
                                item_key,
                                arg_path,
                                &vec_path_that_exists,
                            ));
                        }
                    }
                }
                Ok(node_to_return)
            }
        }
    }
    //
    // Public - get - node parent
    //
    /// Returns the parent of a given node; or None
    /// # Arguments
    /// * arg_node: The node with the parent
    // Test: test_get_node_parent()
    pub fn get_node_parent(&self, arg_node: &Node) -> Option<&Node> {
        if eq(arg_node, self) {
            return None;
        }
        let mut stack = self
            .field_hash_map_children
            .values()
            .map(|item| (self, item))
            .collect::<Vec<(&Node, &Node)>>();
        loop {
            let (item_node_parent, item_node) = stack.pop()?;
            if eq(item_node, arg_node) {
                return Some(item_node_parent);
            }
            stack.extend(
                item_node
                    .field_hash_map_children
                    .values()
                    .map(|item_node_sub| (item_node, item_node_sub))
                    .collect::<Vec<(&Node, &Node)>>(),
            );
        }
    }
    //
    // Public - get - path
    //
    /// Returns the key pointing to arg_node
    /// # Arguments
    /// * arg_node: This is the node searched for
    // Test: test_get_key_to_node()
    pub fn get_key_to_node(&self, arg_node: &Node) -> Option<String> {
        for (item_key, item_node) in &self.field_hash_map_children {
            if eq(item_node, arg_node) {
                return Some(item_key.clone());
            }
        }
        None
    }

    /// Returns the path to the node, relative to the root node; or None
    /// # Arguments
    /// * arg_node: This is the node searched for
    // Test: test_get_path_to_node()
    pub fn get_path_to_node(&self, arg_node: &Node) -> Option<Vec<String>> {
        let mut stack = self
            .field_hash_map_children
            .iter()
            .map(|(item_key, item_node)| (vec![item_key.as_str()], item_node))
            .collect::<Vec<(Vec<&str>, &Node)>>();
        loop {
            let (item_path, item_node) = stack.pop()?;
            if eq(item_node, arg_node) {
                return Some(item_path.iter().map(|item| item.to_string()).collect());
            }
            //
            // Prep next iteration
            //
            for (item_key_sub, item_node_sub) in &item_node.field_hash_map_children {
                stack.push((
                    {
                        let mut vec_path_sub = item_path.to_vec();
                        vec_path_sub.push(item_key_sub.as_str());
                        vec_path_sub
                    },
                    item_node_sub,
                ))
            }
        }
    }
    //
    // Public - get - vec - keys
    //
    /// Returns a vec of keys for each immediate child of the root node
    // No test needed since this func is a straight wrapper
    pub fn get_vec_of_keys(&self) -> Vec<String> {
        self.field_hash_map_children
            .keys()
            .map(|item| item.clone())
            .collect()
    }

    /// Returns a vec of keys sorted
    // Tested in: test_pop_node_and_promote_its_children()
    pub fn get_vec_of_keys_sorted(&self) -> Vec<String> {
        let mut vec_to_return = self.get_vec_of_keys();
        vec_to_return.sort_by(|item_left, item_right| item_left.cmp(item_right));
        vec_to_return
    }

    /// Returns a vec of child keys, filtered by suffix
    /// Tip: Useful if running pop_node_and_promote_children_at_key()
    /// # Arguments
    /// * arg_suffix: All keys possessing this suffix get returned
    // Test: get_vec_of_keys_all_ending_with_suffix_sorted()
    pub fn get_vec_of_keys_all_ending_with_suffix(&self, arg_suffix: &str) -> Vec<String> {
        self.field_hash_map_children
            .keys()
            .filter(|item| item.ends_with(arg_suffix))
            .map(|item| item.clone())
            .collect::<Vec<String>>()
    }

    /// Returns a vec of child keys, filtered by suffix and sorted
    /// Tip: Useful if running pop_node_and_promote_children_at_key()
    /// # Arguments
    /// * arg_suffix: All keys possessing this suffix get returned
    // Tested: test_pop_node_at_key_and_promote_its_children()
    pub fn get_vec_of_keys_all_ending_with_suffix_sorted(&self, arg_suffix: &str) -> Vec<String> {
        let mut vec_to_return = self
            .field_hash_map_children
            .keys()
            .filter(|item| item.ends_with(arg_suffix))
            .map(|item| item.clone())
            .collect::<Vec<String>>();
        vec_to_return.sort_by(|item_left, item_right| item_left.cmp(item_right));
        vec_to_return
    }
    //
    // Public - get - vec - node
    //
    /// Returns vec consisting of the root node's immediate children
    // Testing not needed, this is a wrapper
    pub fn get_vec_of_node_children(&self) -> Vec<&Node> {
        self.field_hash_map_children
            .values()
            .collect::<Vec<&Node>>()
    }

    /// Returns vec of nodes within subtree that have no children
    // Test: get_vec_of_node_leaves()
    pub fn get_vec_of_node_leaves(&self) -> Vec<&Node> {
        let mut vec_to_return = Vec::new();
        let mut stack = self
            .field_hash_map_children
            .values()
            .collect::<Vec<&Node>>();
        loop {
            let item_node = match stack.pop() {
                Some(result) => result,
                None => break,
            };
            if item_node.field_hash_map_children.is_empty() {
                vec_to_return.push(item_node)
            } else {
                stack.extend(item_node.field_hash_map_children.values())
            }
        }
        vec_to_return
    }

    /// Returns vec of nodes along a given path; this does *not* include the root node
    /// # Arguments
    /// * arg_path: This is the path along which the nodes will be collected
    // Test: test_pathing()
    pub fn get_vec_of_nodes_along_path(&self, arg_path: &[&str]) -> Option<Vec<&Node>> {
        let mut vec_to_return = Vec::new();
        let mut item_node_current = self;
        for item_key in arg_path {
            // Update item_node_current for next iteration
            item_node_current = item_node_current
                .field_hash_map_children
                .get(&item_key.to_string())?;
            vec_to_return.push(item_node_current);
        }
        Some(vec_to_return)
    }

    /// Returns a vec of all nodes within the tree; this does *not* include the root node
    // Test: test_get_vec_of_nodes_in_tree()
    pub fn get_vec_of_nodes_in_tree(&self) -> Vec<&Node> {
        let mut vec_to_return = Vec::new();
        let mut stack = self
            .field_hash_map_children
            .values()
            .collect::<Vec<&Node>>();
        loop {
            let item_node = match stack.pop() {
                Some(result) => result,
                None => break,
            };
            vec_to_return.push(item_node);
            stack.extend(item_node.field_hash_map_children.values())
        }
        vec_to_return
    }

    /// Returns a vec of all nodes that cause callback to return true
    /// # Arguments
    /// * arg_callback: Receives a node and returns bool
    // Test: test_get_vec_of_nodes_satisfying_callback()
    pub fn get_vec_of_nodes_satisfying_callback(
        &self,
        arg_callback: impl Fn(&Node) -> bool,
    ) -> Vec<&Node> {
        let mut vec_to_return = Vec::new();
        let mut stack = self
            .field_hash_map_children
            .values()
            .collect::<Vec<&Node>>();
        loop {
            let item_node = match stack.pop() {
                Some(result) => result,
                None => break,
            };
            if arg_callback(item_node) {
                vec_to_return.push(item_node)
            }
            stack.extend(item_node.field_hash_map_children.values());
        }
        vec_to_return
    }
    //
    // Public - get - vec - pairs
    //
    /// Returns a vec of tuple-pairs referring to the keys and node children
    // No test needed; simple collection
    pub fn get_vec_of_pairs_keys_and_node_children(&self) -> Vec<(String, &Node)> {
        self.field_hash_map_children
            .iter()
            .map(|(item_string, item_node)| (item_string.clone(), item_node))
            .collect()
    }

    /// Returns a vec of tuple-pairs referring to the keys and node children
    // No test needed; simple collection
    pub fn get_vec_of_pairs_keys_and_node_mut_children(&mut self) -> Vec<(String, &mut Node)> {
        self.field_hash_map_children
            .iter_mut()
            .map(|(item_string, item_node)| (item_string.clone(), item_node))
            .collect()
    }

    /// Returns a vec of tuple-pairs consisting of paths and nodes; filtered by callback returning true
    /// Arguments
    /// * arg_callback: Receives a node and returns bool
    // Test: test_get_vec_of_pairs_paths_and_nodes_with_data_satisfying_callback_sorted()
    pub fn get_vec_of_pairs_paths_and_nodes_with_data_satisfying_callback(
        &self,
        arg_callback: impl Fn(&Node) -> bool,
    ) -> Vec<(Vec<String>, &Node)> {
        let mut vec_to_return = Vec::new();
        let mut stack = self
            .field_hash_map_children
            .iter()
            .map(|(item_key, item_node)| (vec![item_key.as_str()], item_node))
            .collect::<Vec<(Vec<&str>, &Node)>>();
        loop {
            //
            // Pop from stack
            //
            let (item_path, item_node) = match stack.pop() {
                Some(result) => result,
                None => break,
            };
            //
            // Do comparison and add to return value
            //
            if arg_callback(item_node) {
                vec_to_return.push((item_path.to_vec(), item_node))
            }
            //
            // Prep next iteration
            //
            for (item_key_sub, item_node_sub) in &item_node.field_hash_map_children {
                stack.push((
                    {
                        let mut item_path_sub = item_path.to_vec();
                        item_path_sub.push(item_key_sub.as_str());
                        item_path_sub
                    },
                    &item_node_sub,
                ));
            }
        }
        vec_to_return
            .iter()
            .map(|(item_path, item_node)| {
                (
                    item_path
                        .iter()
                        .map(|item| item.to_string())
                        .collect::<Vec<String>>(),
                    *item_node,
                )
            })
            .collect()
    }

    /// Returns a sorted vec of tuple-pairs consisting of paths and nodes with data matching the argument; filtered by callback
    /// # Arguments
    /// * arg_callback: Receives a node and returns bool
    // Test: test_get_vec_of_pairs_paths_and_nodes_with_data_satisfying_callback_sorted()
    pub fn get_vec_of_pairs_paths_and_nodes_with_data_satisfying_callback_sorted(
        &self,
        arg_callback: impl Fn(&Node) -> bool,
    ) -> Vec<(Vec<String>, &Node)> {
        Node::get_vec_of_pairs_paths_and_generics_sorted(
            &self.get_vec_of_pairs_paths_and_nodes_with_data_satisfying_callback(arg_callback),
        )
    }
    //
    // Public - get - vec - paths
    //
    /// Returns a vec of all paths within the tree
    // Test: test_get_vec_of_paths_in_tree()
    pub fn get_vec_of_paths_in_tree(&self) -> Vec<Vec<String>> {
        let mut vec_to_return = Vec::new();
        let mut stack = self
            .field_hash_map_children
            .iter()
            .map(|(item_key, item_node)| (vec![item_key.as_str()], item_node))
            .collect::<Vec<(Vec<&str>, &Node)>>();
        loop {
            let (item_path, item_node) = match stack.pop() {
                Some(result) => result,
                None => break,
            };
            vec_to_return.push(item_path.to_vec());
            for (item_key_sub, item_node_sub) in &item_node.field_hash_map_children {
                stack.push((
                    {
                        let mut item_path_sub = item_path.to_vec();
                        item_path_sub.push(item_key_sub.as_str());
                        item_path_sub
                    },
                    &item_node_sub,
                ));
            }
        }
        vec_to_return
            .iter()
            .map(|item| {
                item.iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
            })
            .collect()
    }

    /// Returns a vec of all paths within the tree, sorted
    // Test: test_get_vec_of_paths_in_tree()
    pub fn get_vec_of_paths_in_tree_sorted(&self) -> Vec<Vec<String>> {
        Node::get_vec_of_paths_sorted(&self.get_vec_of_paths_in_tree())
    }

    /// Returns a vec of all paths pointing to nodes with no children
    // Test: test_get_vec_of_node_leaves()
    pub fn get_vec_of_paths_to_node_leaves(&self) -> Vec<Vec<String>> {
        let mut vec_to_return = Vec::new();
        let mut stack = self
            .field_hash_map_children
            .iter()
            .map(|(item_key, item_node)| (vec![item_key.as_str()], item_node))
            .collect::<Vec<(Vec<&str>, &Node)>>();
        loop {
            let (item_path, item_node) = match stack.pop() {
                Some(result) => result,
                None => break,
            };
            if item_node.field_hash_map_children.is_empty() {
                vec_to_return.push(item_path.to_vec())
            } else {
                for (item_key_sub, item_node_sub) in &item_node.field_hash_map_children {
                    stack.push((
                        {
                            let mut item_path_sub = item_path.to_vec();
                            item_path_sub.push(item_key_sub.as_str());
                            item_path_sub
                        },
                        &item_node_sub,
                    ));
                }
            }
        }
        vec_to_return
            .iter()
            .map(|item| {
                item.iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
            })
            .collect()
    }

    /// Returns a vec of all paths pointing to nodes whose data satisfies the callback
    /// # Arguments
    /// * arg_callback: Receives a node and returns bool.
    // Test: test_get_vec_of_paths_to_nodes_satisfying_callback_sorted()
    pub fn get_vec_of_paths_to_nodes_satisfying_callback(
        &self,
        arg_callback: impl Fn(&Node) -> bool,
    ) -> Vec<Vec<String>> {
        let mut vec_to_return = Vec::new();
        let mut stack = self
            .field_hash_map_children
            .iter()
            .map(|(item_key, item_node)| (vec![item_key.as_str()], item_node))
            .collect::<Vec<(Vec<&str>, &Node)>>();
        loop {
            let (item_path, item_node) = match stack.pop() {
                Some(result) => result,
                None => break,
            };
            if arg_callback(item_node) {
                vec_to_return.push(item_path.to_vec())
            }
            for (item_key_sub, item_node_sub) in &item_node.field_hash_map_children {
                stack.push((
                    {
                        let mut item_path_sub = item_path.to_vec();
                        item_path_sub.push(item_key_sub.as_str());
                        item_path_sub
                    },
                    &item_node_sub,
                ));
            }
        }
        vec_to_return
            .iter()
            .map(|item| {
                item.iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>()
    }

    /// Returns a vec of paths to all nodes whose satisfies the callback
    /// * arg_callback: Receives a node and returns bool
    // Test: test_get_vec_of_paths_to_nodes_satisfying_callback_sorted()
    pub fn get_vec_of_paths_to_nodes_satisfying_callback_sorted(
        &self,
        arg_callback: impl Fn(&Node) -> bool,
    ) -> Vec<Vec<String>> {
        let mut vec_to_return = self.get_vec_of_paths_to_nodes_satisfying_callback(arg_callback);
        vec_to_return.sort_by(|item_path_left, item_path_right| {
            item_path_left.join(".").cmp(&item_path_right.join("."))
        });
        vec_to_return
    }
    //
    // Public - insert
    //
    /// Adds node at the designated key.
    /// WARNING: The tree needs to take ownership of this node upon passing
    /// # Arguments
    /// * arg_key: A key of type &str used to identify a child node
    /// * arg_node: This is node to add at the end of the path
    // Test: test_insert_node_at_key()
    pub fn insert_node_at_key(&mut self, arg_key: &str, arg_node: Node) -> Result<(), String> {
        Node::raise_error_if_key_is_invalid(arg_key)?;
        self.field_hash_map_children
            .insert(arg_key.to_string(), arg_node);
        Ok(())
    }

    /// Adds node at path. Returns the parent of the new node. Despite the return value being 'optional'
    /// this will always return a Node.
    /// WARNING: The tree needs to take ownership of this node upon passing
    /// The function will create any necessary intermediate nodes along its approach
    /// # Arguments
    /// * arg_path: Slice of &str keys which is use to navigate from the node of origin to the descendant node
    /// * arg_node: This is node to add at the end of the path
    // Test: test_pop_node_at_path() ...also this is a 'work horse' function in the test file
    pub fn insert_node_at_path(&mut self, arg_path: &[&str], arg_node: Node) -> Result<(), String> {
        match arg_path.len() {
            1 => return self.insert_node_at_key(arg_path[0], arg_node),
            _ => {
                // Block any attempt to use a path with empty elements
                self.raise_error_if_path_is_invalid(&arg_path)?;

                let mut node_current = self;
                for item_key in &arg_path[0..arg_path.len() - 1] {
                    // Reminder: This approach is necessary because Rust won't tolerate more than *one* mutable borrow
                    if node_current
                        .field_hash_map_children
                        .contains_key(&item_key.to_string())
                    {
                        node_current = match node_current
                            .field_hash_map_children
                            .get_mut(&item_key.to_string())
                        {
                            Some(result) => result,
                            None => {
                                return Err(
                                    "Error: Failed to get node child that's definitely there."
                                        .to_string(),
                                )
                            }
                        }
                    } else {
                        node_current
                            .field_hash_map_children
                            .insert(item_key.to_string(), Node::new());
                        node_current = match node_current
                            .field_hash_map_children
                            .get_mut(&item_key.to_string())
                        {
                            Some(result) => result,
                            None => {
                                return Err(
                                    "Error: Failed to get node child that's definitely there."
                                        .to_string(),
                                )
                            }
                        }
                    }
                }
                node_current
                    .field_hash_map_children
                    .insert(arg_path[arg_path.len() - 1].to_string(), arg_node);
            }
        }
        Ok(())
    }
    //
    // Public - logic
    //
    /// Returns true if the root node has children
    // No test needed since this is a wrapper
    pub fn has_children(&self) -> bool {
        self.field_hash_map_children.len() > 0
    }

    /// Returns true if the string within the node contains any data
    // No test needed since this is a wrapper
    pub fn has_data(&self) -> bool {
        self.field_string_data.len() > 0
    }

    /// Returns true if the key exists for an immediate child
    /// # Arguments
    /// * arg_key: A key of type &str used to identify a child node
    // No test needed since this is a wrapper
    pub fn has_key(&self, arg_key: &str) -> bool {
        self.field_hash_map_children
            .contains_key(&arg_key.to_string())
    }

    /// Returns true if function can travers entire path without encountering a missing node
    /// # Arguments
    /// * arg_path: Slice of &str keys which is use to navigate from the node of origin to the descendant node
    // Test: test_pop_node_at_path()
    pub fn has_path(&self, arg_path: &[&str]) -> bool {
        let mut node_current = self;
        for item_key in arg_path {
            node_current = match node_current
                .field_hash_map_children
                .get(&item_key.to_string())
            {
                Some(result) => result,
                None => return false,
            }
        }
        true
    }

    /// Returns true if all path elements are valid (is *not* empty, and has no empty components)
    // Test isn't really needed here
    pub fn is_path_valid(&self, arg_path: &[&str]) -> bool {
        if arg_path.is_empty() {
            return false;
        }
        for item_key in arg_path {
            if item_key.is_empty() {
                return false;
            }
        }
        true
    }
    //
    // Public - pop
    //
    /// Returns a vec of tuple pairs: key and node, and removes them all from this node
    // Test: test_pop_all_children_and_return_vec_of_pairs_keys_and_child_nodes()
    pub fn pop_all_children_and_return_vec_of_pairs_keys_and_child_nodes(
        &mut self,
    ) -> Option<Vec<(String, Node)>> {
        let mut vec_to_return = Vec::new();
        // Reminder: The clone is needed here to avoid an (im)mutable complaint
        for item_key in self
            .field_hash_map_children
            .keys()
            .map(|item| item.clone())
            .collect::<Vec<String>>()
        {
            vec_to_return.push((
                item_key.clone(),
                self.field_hash_map_children.remove(&item_key)?,
            ));
        }
        Some(vec_to_return)
    }

    /// Pops a child, and promotes all of its children to become parents of this node
    /// # Arguments
    /// * arg_key: The str key to pop
    /// * arg_collision_suffix: Custom suffix to append to all promoted nodes
    // Test: test_pop_node_and_promote_its_children()
    pub fn pop_node_at_key_and_promote_its_children(
        &mut self,
        arg_key: &str,
        arg_collision_suffix: &str,
    ) -> Result<Node, String> {
        let node_to_pop = match self.field_hash_map_children.get(arg_key) {
            Some(result) => result,
            None => {
                return Err([
                    "Error: arg_key is not a child.".to_string(),
                    format!("arg_key = {}", arg_key),
                ]
                .join("\n"))
            }
        };
        //
        // Generate the keys with the suffixes and check for collisions before making any changes to the tree
        //
        let mut vec_of_collisions_detected: Vec<String> = Vec::new();
        let mut vec_of_pairs_keys_in_node_to_pop_and_keys_with_suffixes = Vec::new();
        for item_key in node_to_pop.field_hash_map_children.keys() {
            let item_key_with_suffix = [item_key, arg_collision_suffix].join("");
            vec_of_pairs_keys_in_node_to_pop_and_keys_with_suffixes
                .push((item_key.clone(), item_key_with_suffix.clone()));
            if self
                .field_hash_map_children
                .contains_key(&item_key_with_suffix)
            {
                vec_of_collisions_detected.push(item_key_with_suffix.clone());
            }
        }
        if vec_of_collisions_detected.len() > 0 {
            return Err([
                "Error: Collisions detected with suffix.".to_string(),
                format!(
                    "vec_of_collisions_detected = {:?}",
                    vec_of_collisions_detected
                ),
            ]
            .join("\n"));
        }
        //
        // If we get this far, then there are no collisions
        //
        let mut node_popped = match self.field_hash_map_children.remove(arg_key) {
            Some(result) => result,
            None => {
                return Err([
                    "Error: Failed key doesn't exist.".to_string(),
                    format!("arg_key = {}", arg_key),
                ]
                .join("\n"))
            }
        };
        for (item_key_to_pop, item_key_with_suffix) in
            vec_of_pairs_keys_in_node_to_pop_and_keys_with_suffixes
        {
            self.field_hash_map_children.insert(
                item_key_with_suffix.to_string(),
                match node_popped
                    .field_hash_map_children
                    .remove(item_key_to_pop.as_str())
                {
                    Some(result) => result,
                    None => {
                        return Err([
                            "Error: Failed to pop child from node_popped.".to_string(),
                            format!("item_key_to_pop = {}", item_key_to_pop,),
                            format!(
                                "slice of valid keys = {:?}",
                                self.field_hash_map_children.keys()
                            ),
                        ]
                        .join("\n"))
                    }
                },
            );
        }
        Ok(node_popped)
    }

    /// Pops a child, and promotes all of its children to become parents of this node
    ///
    pub fn pop_node_at_path_and_promote_its_children(
        &mut self,
        arg_path: &[&str],
        arg_collision_suffix: &str,
    ) -> Result<Node, String> {
        match arg_path.len() {
            1 => {
                return self
                    .pop_node_at_key_and_promote_its_children(arg_path[0], arg_collision_suffix)
            }
            _ => {
                self.raise_error_if_path_is_invalid(&arg_path)?;

                let index_last = arg_path.len() - 1;
                let node_parent = self.get_node_mut_at_path_or_error(&arg_path[0..index_last])?;
                Ok(node_parent.pop_node_at_key_and_promote_its_children(
                    arg_path[index_last],
                    arg_collision_suffix,
                )?)
            }
        }
    }

    /// Returns the node stored at the key; the tree will relinquish ownership of this node
    /// # Arguments
    /// * arg_key: A key of type &str used to identify a child node
    // Test: test_pop_node_at_key()
    pub fn pop_node_at_key(&mut self, arg_key: &str) -> Option<Node> {
        Some(self.field_hash_map_children.remove(&arg_key.to_string())?)
    }

    /// Returns the node stored at the path; the tree will relinquish ownership of this node
    /// # Arguments
    /// * arg_path: Slice of &str keys which is use to navigate from the node of origin to the descendant node
    // Test: test_pop_node_at_key()
    pub fn pop_node_at_path(&mut self, arg_path: &[&str]) -> Option<Node> {
        match arg_path.len() {
            0 => return None,
            1 => return self.pop_node_at_key(arg_path[0]),
            _ => {
                let mut node_current = self;
                for item_key in &arg_path[0..arg_path.len() - 1] {
                    node_current = match node_current
                        .field_hash_map_children
                        .get_mut(&item_key.to_string())
                    {
                        Some(result) => result,
                        None => return None,
                    };
                }
                Some(
                    node_current
                        .field_hash_map_children
                        .remove(&arg_path[arg_path.len() - 1].to_string())?,
                )
            }
        }
    }

    /// Returns the node stored at the path; the tree will relinquish ownership of this node
    /// If the path fails, then the function will return an error message explaining where the path failed
    /// # Arguments
    /// * arg_path: Slice of &str keys which is use to navigate from the node of origin to the descendant node
    pub fn pop_node_at_path_or_error(&mut self, arg_path: &[&str]) -> Result<Node, String> {
        match arg_path.len() {
            0 => return Err("Error: arg_path is empty.".to_string()),
            1 => match self.pop_node_at_key(arg_path[0]) {
                Some(result) => return Ok(result),
                None => {
                    return Err([
                        "Error: arg_path not found.".to_string(),
                        format!("arg_path = {:?}", arg_path),
                    ]
                    .join("\n"))
                }
            },
            _ => {
                let mut vec_path_existing = Vec::new();
                let mut node_current = self;
                for item_key in &arg_path[0..arg_path.len() - 1] {
                    node_current = match node_current
                        .field_hash_map_children
                        .get_mut(&item_key.to_string())
                    {
                        Some(result) => {
                            vec_path_existing.push(*item_key);
                            result
                        }
                        None => {
                            return Err(Node::raise_error_because_path_failed(
                                item_key,
                                arg_path,
                                &vec_path_existing,
                            ))
                        }
                    };
                }
                match node_current
                    .field_hash_map_children
                    .remove(arg_path[arg_path.len() - 1])
                {
                    Some(result) => Ok(result),
                    None => {
                        return Err([
                            "Error: Failed to remove node at path.".to_string(),
                            format!("arg_path = {:?}", arg_path,),
                            format!("part of path that exists = {:?}", vec_path_existing,),
                        ]
                        .join("\n"))
                    }
                }
            }
        }
    }
    //
    // Public - raise
    //
    /// Returns error in all cases when called
    // No test needed, this is just a basic message
    fn raise_error_because_path_failed(
        arg_key_at_failure: &str,
        arg_path: &[&str],
        arg_vec_path_that_exists: &Vec<&str>,
    ) -> String {
        return [
            "Error: Path doesn't exist.".to_string(),
            format!("key at failure = {}", arg_key_at_failure),
            format!("path = {:?}", arg_path,),
            format!("existing path component = {:?}", arg_vec_path_that_exists),
        ]
        .join("\n");
    }

    /// Returns an error if string is invalid
    // No test needed, this is a basic test
    pub fn raise_error_if_key_is_invalid(arg_key: &str) -> Result<(), String> {
        if arg_key.is_empty() {
            return Err("Error: arg_key is empty.".to_string());
        }
        Ok(())
    }

    /// Returns an error if the path is invalid; otherwise it returns ()
    /// See is_path_valid() for criteria
    // Test: test_raise_error_if_path_is_invalid()
    pub fn raise_error_if_path_is_invalid(&self, arg_path: &[&str]) -> Result<(), String> {
        if arg_path.is_empty() {
            return Err(["Error: arg_path is empty.".to_string()].join("\n"));
        }
        for item_key in arg_path {
            if item_key.is_empty() {
                return Err([
                    "Error: Empty items detected in arg_path.".to_string(),
                    format!("arg_path = {:?}", arg_path),
                ]
                .join("\n"));
            }
        }
        Ok(())
    }
    //
    // Private - sorting
    //
    // Reminders:
    // - Each of these do a single pass of converting paths into strings, and then sorts them based on that
    // - When the sort finishes, the original layout is rebuilt and returned
    // - To keep the memory use low, this method *takes ownership* of the argument and does the sort on the data directly
    // - These are private since they 'move' data, rather than copies it
    //
    fn get_vec_of_pairs_paths_and_generics_sorted<'l, T>(
        arg_vec_of_pairs: &Vec<(Vec<String>, &'l T)>,
    ) -> Vec<(Vec<String>, &'l T)> {
        let mut vec_to_sort = Vec::new();
        for (item_path, item_t) in arg_vec_of_pairs {
            vec_to_sort.push((item_path.join("\n"), item_path, item_t));
        }
        vec_to_sort.sort_by(
            |(item_string_left, _item_vec_left, _item_t_left),
             (item_string_right, _item_vec_right, _item_t_right)| {
                item_string_left.cmp(item_string_right)
            },
        );
        let mut vec_to_return = Vec::new();
        for (_item_string, item_path, item_t) in vec_to_sort {
            vec_to_return.push((item_path.to_vec(), *item_t))
        }
        vec_to_return
    }

    fn get_vec_of_paths_sorted(arg_vec_of_paths: &Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut vec_to_sort = Vec::new();
        for item_path in arg_vec_of_paths {
            vec_to_sort.push((item_path.join("."), item_path));
        }
        vec_to_sort.sort_by(
            |(item_string_left, _item_vec_left), (item_string_right, _item_vec_right)| {
                item_string_left.cmp(item_string_right)
            },
        );
        let mut vec_to_return = Vec::new();
        for (_item_string, item_path) in vec_to_sort {
            vec_to_return.push(item_path.to_vec());
        }
        vec_to_return
    }
    //
    // Setup
    //
    /// Get new node instance
    pub fn new() -> Self {
        return Self {
            field_string_data: "".to_string(),
            field_hash_map_children: HashMap::new(),
        };
    }
}
