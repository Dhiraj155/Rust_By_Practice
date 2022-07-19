
fn main() {
    // Fix error by modifying this line
    let  s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}

//ans---

fn main() {
    //fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object(&mut s)
}

fn borrow_object(s: &mut String) {}