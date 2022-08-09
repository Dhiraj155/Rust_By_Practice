//The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a trait bound.

//When working with generics, the type parameters often must use traits as bounds to stipulate what functionality a type implements.

fn main() {
    assert_eq!(sum(1, 2), 3);
}

// Implement `fn sum` with trait bound in two ways.
fn sum<T>(x: T, y: T) -> T {
    x + y
}

//ans--

fn main() {
    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum(1.0, 2.0), 3.0);
}

fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

//2--

fn main() {
    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum(1.0, 2.0), 3.0);
}

fn sum<T>(x: T, y: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    x + y
}