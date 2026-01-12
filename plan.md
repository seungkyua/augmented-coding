# B+ Tree Test Plan

This document tracks the TDD implementation of the B+ Tree data structure. Tests are marked as complete when they pass and the implementation is verified.

## Test List

### Basic Structure Tests
- [x] `empty_tree_has_len_zero` - An empty tree should have length 0
- [x] `create_tree_with_branching_factor` - Add branching_factor parameter to BPlusTree constructor

### Insertion Tests
- [x] `insert_increases_length` - Insert one key-value pair and verify it's stored

### Retrieval Tests
- [ ] `get_returns_inserted_value`  - Implement get method for BPlusTree
                                    - Add test for retrieving inserted values
                                    - Implement get method to return values from the tree
                                    - Verify correct handling of both existing and missing keys

### Insertion Tests
- [ ] `insert_multiple_items` - Insert multiple items and verify all are stored
- [ ] `insert_duplicate_key` - Inserting the same key twice should update the value

### Retrieval Tests
- [ ] `get_existing_key` - Retrieve a value using an existing key
- [ ] `get_nonexistent_key` - Retrieve should return None for non-existent key
- [ ] `get_after_insert` - Verify can retrieve after insertion

### Removal Tests
- [ ] `remove_existing_key` - Remove an existing key-value pair
- [ ] `remove_nonexistent_key` - Removing a non-existent key should return None
- [ ] `remove_decreases_len` - Removing an item should decrease the length

### Edge Cases
- [ ] `tree_with_one_item_has_len_one` - Tree with one item should have length 1
- [ ] `remove_all_items_makes_tree_empty` - Removing all items should make tree empty

## Current Status

Completed: `empty_tree_has_len_zero`
Next: `empty_tree_is_empty`
