mod arrays;
mod conditional;
mod constants;
mod expression;
mod functions;
mod loops;
mod shadowing;
mod tuples;
mod vars;

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
}
