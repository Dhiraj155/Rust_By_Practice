
fn main() {
    let s = String::from("hello, ");
    
    // Modify this line only !
    let s1 = s;

    s1.push_str("world");

    println!("Success!");
}

//ans----

fn main() {
    let s = String::from("hello, ");
    
    // modify this line only !
    let mut s1 = s;

    s1.push_str("world")
}