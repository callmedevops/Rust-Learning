mod addition;
mod module_user;
mod submodules;

use addition::add as adder_function;
use submodules::subtraction::sub as my_sub;

fn main() {
    println!("{:?}", addition::add(5, 10));
    println!("{:?}", submodules::subtraction::sub(10, 5));

    println!("{:?}", adder_function(5, 10));
    println!("{:?}", my_sub(10, 5));

    println!("{:?}", module_user::print_addition())
}
