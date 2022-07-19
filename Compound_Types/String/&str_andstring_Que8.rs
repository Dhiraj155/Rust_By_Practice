//We can use String::from or to_string to convert a &str to String


// Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = s;

    println!("Success!");
}

//ans---

fn main() {
    let s = "hello, world".to_string();
    let s1: &str = &s;
}

//2

fn main() {
    let s = "hello, world";
    let s1: &str = s;
}

//3

fn main() {
    let s = "hello, world".to_string();
    let s1: String = s;
}