#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Returns an empty list, which is just Nil
fn create_empty_list() -> List {
    List::Nil
}

// Returns a list like: 1 -> 2 -> 3 -> Nil
fn create_non_empty_list() -> List {
    List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(
                3,
                Box::new(List::Nil),
            )),
        )),
    )
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}
