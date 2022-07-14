//Make it work in two distinct ways
fn main() {
    assert!(0.1+0.2==0.3);

    println!("Success!");
}

//Ans--

fn main() {
    assert!(0.1_f32+0.2_f32==0.3_f32);
}

fn main() {
    assert!((0.1_f64+ 0.2 - 0.3).abs() < 0.001);
}