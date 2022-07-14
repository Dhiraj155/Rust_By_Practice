//What's the size of the unit type?
// Modify `4` in assert to make it work
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 4);

    println!("Success!");
}

//Ans---

use std::mem::size_of_val;

fn main() {
    let unit: () = ();
    // unit type does't occupy any memeory space
    assert!(size_of_val(&unit) == 0);
}