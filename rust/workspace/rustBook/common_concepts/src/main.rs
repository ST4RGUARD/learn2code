mod arrays;
mod conditional;
mod constants;
mod enums;
mod expression;
mod functions;
mod loops;
mod shadowing;
mod slice;
mod structs;
mod tuples;
mod vars;
mod vec;

fn main() {
    vars::mutability();
    constants::constants();
    shadowing::shadowing();
    tuples::tuples();
    tuples::tup_direct_access();
    arrays::arrays();
    arrays::array_element_access();
    functions::another_function(5);
    functions::plus_one(3);
    expression::expression();
    conditional::conditional();
    loops::loop_label();
    slice::slice_word();
    structs::my_struct();
    vec::vects();
    enums::stubby();
}
