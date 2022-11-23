use List::*;
enum List{
    //Box<T> for heap memory allocation of (a recursively-defined) type
    //whose size cannot be known at compile-time; thus, cannot be stored
    //on the stack
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {

        /*
        We prefer matching on type 'T' as opposed to matching on a
        reference
        */
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }

    }

    fn stringify(&self) -> String {
        match *self {
            Cons(ref head,ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => format!("Nil"),
        }
    }
}

fn main(){
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list had length: {}", list.len());
    println!("{}", list.stringify());
}