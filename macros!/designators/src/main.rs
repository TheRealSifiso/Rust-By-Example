/*
Macros are created with the 'macro_rules!' macro

The following macro takes an argument type annotated with the 'ident'
designator.

This macro generates a new function with a name provided as an argument.

The 'stringify!' macro converts an 'ident' into a String.
*/

macro_rules! create_function {
    ($func_name: ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

/*
The following macro takes an expression of type 'expr' as an argument and
prints it as a String along with its result.

The 'expr' designator is used for expressions
*/

macro_rules! print_result {
    ($expression: expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}
fn main() {
    println!("Hello, world!");
}
