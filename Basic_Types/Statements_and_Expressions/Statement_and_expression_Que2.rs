
fn main() {
    let v = (let x = 3);
 
    assert!(v == 3);
 
    println!("Success!");
 }

 //Ans----

 fn main() {
    let v = {
        let x = 3;
        x
    };
 
    assert!(v == 3);
}