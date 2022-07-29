// We can also implement methods for enums


#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}

//ans---

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// implement TrafficLightColor with a method
impl TrafficLightColor {
    fn color(&self) -> String {
        match *self {
            TrafficLightColor::Red => "red".to_string(),
            TrafficLightColor::Yellow => "yellow".to_string(),
            TrafficLightColor::Green => "green".to_string(),
        }
    }
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}