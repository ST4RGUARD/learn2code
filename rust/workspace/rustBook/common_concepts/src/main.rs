mod arrays;
mod constants;
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
}
