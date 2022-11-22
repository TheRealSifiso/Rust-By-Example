use std::fmt;

struct List(Vec<i32>);

// The 'Display' trait is the format trait for an empty format, {}
// It is similar to 'Debug' ({:?}), except that it is for user-facing
// output, thus it cannot be derived!

// The use of the 'Display' trait can be thought of similarly to the
// use of the toString() method in Java!
/*
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
*/

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //Writes formatted data into a buffer; 'f' is our buffer!
        write!(f, "[")?;

        let v = &self.0;

        for (index, item) in v.iter().enumerate(){
            write!(f, "{index}: {item}")?;

            if index != v.len()-1{
                write!(f, ", ")?;
            }
        }

        write!(f, "]")

    }
}

fn main(){

    let list = List(vec![1, 2, 3, 4, 5]);

    println!("{}", list);

}