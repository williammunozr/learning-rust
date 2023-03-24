pub mod string_type;
pub mod string_clone_heap;
pub mod functions;
pub mod return_values_and_scope;
pub mod return_multiple_values;

pub fn ownership() {
    // String Type
    string_type::master();

    // String Clone Heap
    string_clone_heap::master();

    // Moving a Value
    functions::master();

    // Return values and scope
    return_values_and_scope::master();

    // Return multiple values
    return_multiple_values::master();
}
