# Data Tree

This a hierarchical data tree which uses human-readable string keys / paths for
navigating / searching for nodes.

## Features

- Is an actual tree (I've seen a few solutions simulating a 'tree' through a vec)
- No 'unsafe' code
- Single-struct library (all nodes are the 'root' of their own trees)
- Elaborate pathing support
- Support for tree analysis such as getting all possible paths within the tree
- Doesn't rely on any form of reference counting
- Nodes derived with serde
- Explicit ownership management
- Reverse lookups
- Search functions (with callbacks)
- Documentation includes 'beginner tips' for things readers might not know about Rust
- Non-halting (at no point will a library error trigger a panic)
- Some doubling up on functions, with one set supporting higher-performance, while their 
counterparts collecting error data to help with debugging

## Library limitations

- Mono-directional: Nodes cannot store their parents (Parents can be fetched from the root node)
- Flexibility and utility took precedence over efficiency where it made sense; so don't
  expect this library to out-perform more extensively engineered competitors
- No auto-balancing (it doesn't make sense for this type of tree)

## General notes

This readme does *not* provide an exhaustive list of all the available functions. See the
docs page for more info.

## How to use this

### Basic example
    //
    // Crate
    //
    use data_tree::Node;
    //
    // Test area
    //
    fn main() {
    //
    // Definitions for the sake of this documentation:
    //
    // 'key' - This refers to any single-string value, usually used to access a node's
    // adjacent child.
    //
    // 'path' - This is effectively a 'slice of keys.'
    //
    let path_original = [ "a", "b", "c" ];
    //
    // All new, immediately configurable, nodes *must* be instantiated from outside
    // the tree.
    //
    let mut node_root = Node::new();
    let mut node_child_new = Node::new();
    //
    // 'Data' stored in the node at 'field_string_data'. This is a dedicated String.
    // This allows for easy interoperability with serde, while providing a guaranteed
    // data-type for development.
    //
    let example_data = "test";
    node_child_new.field_string_data = example_data.to_string();
    //
    // Creates any necessary intermediary nodes
    // WARNING: Upon passing a node to an 'insert' function, the tree *will*
    // take ownership (see 'Design' section for details).
    //
    // This is necessary keep the tree's flexibility high.
    //
    match node_root.insert_node_at_path( &path_original, node_child_new ) {
      Ok( () ) => {},
      Err( err ) => panic!( "{}", err )
    };
    //
    // Don't worry. You can get the child back.
    //
    // The node can now be fetched at the path; with the option of having it mutable.
    //
    let node_child_fetched_as_mut =
      match node_root.get_node_mut_at_path_or_error( &path_original ) {
        Ok( result ) => result,
        Err( err ) => panic!( "{}", err )
      };
    //
    // The data extracted from the node should match what was originally stored
    //
    assert_eq!( &node_child_fetched_as_mut.field_string_data, example_data );
    //
    // You can mutate the data without errors
    //
    let example_data_new = "test_new";

    node_child_fetched_as_mut.field_string_data = example_data_new.to_string();

    assert_eq!( node_child_fetched_as_mut.field_string_data, example_data_new );
    //////////////////////////////////////////////////////////////////////////////////////////////////
    // WARNING: node_child_fetched_as_mut will limit what else we can do if we try to keep it in use
    // with other function calls. At this point, we should ignore it from here on to avoid compiler
    // gripes.
    //////////////////////////////////////////////////////////////////////////////////////////////////
    //
    // However, if we want to get it as immutable, this improves our flexibility. One issue, I forgot
    // the path already. Let's do a search.
    //
    let vec_of_nodes_found_in_search = node_root.get_vec_of_nodes_satisfying_callback(
        |item_node| {
            item_node.field_string_data == example_data_new
        }
    );
    //
    // One result. Looks promising.
    //
    assert_eq!( vec_of_nodes_found_in_search.len(), 1 );

    let node_from_search =
        match vec_of_nodes_found_in_search.get( 0 ) {
            Some( result ) => result,
            None => panic!( "Failed to get node from vec." )
        };

    println!( "node_from_search.field_string_data = {}", node_from_search.field_string_data );

    // Check to see if this node's data is what we assigned it earlier
    assert_eq!( node_from_search.field_string_data, example_data_new );
    //
    // Is this the same node we originally created?
    //
    let path_to_node =
        match node_root.get_path_to_node( &node_from_search ) {
            Some( result ) => result,
            None => panic!( "Failed to find path to node." )
        };
    //
    // Looks like we got the right one
    //
    println!( "path_to_node = {:?}", path_to_node );
    assert_eq!( path_to_node, path_original );
    //
    // This example is non-exhaustive and the crate docs should be consulted for other options
    //
}

## Design choices

### Every node is the 'root' of its own subtree

This library is single-struct is because all the functionality to manage a node's children
and grand children can be managed within it.

### Node storage

Each node has a committed String field: field_string_data

Rather than defining accessor and 'set' functions, this field is simply public.

Tip: serde is recommended for use with this crate.

### No persisting internal references

This solves two big issues:
- Managing lifetimes from the outside
- Dealing with overlapping (im)mutable reference errors

In this case, the tree serves as the 'single source of truth' regarding its contents
and all associated lifetimes.

### Only nodes are returned as references

Much like with 'no internal references', this choice is meant to maximize the tree's
utility from outside the library.

Returned paths and keys are treated like 'snapshots' taken at the time the accessor function
ran.

Since only nodes are returned as references, its fairly straightforward to track what type 
'borrow' state a given root node is in.

### Ownership

#### Tree takes ownership of nodes 'inserted' into it.

If adding a node to the tree, the node is 'moved' into the tree. Paths and keys
passed via these functions are copied internally.

Some notes of optimizations:
- It made more sense to clone necessary string components internally since the dev
doesn't have a lot of options to avoid redundant clones from the outside.
- All queries are 'pass by reference' and avoid internal clones with the exception
of returned values. These need to be cloned to avoid borrow checker issues.
- Where possible, zero-cost options were used over unnecessary copies.

## Updates

### 0.1.2

- Reduced some verbosity 
