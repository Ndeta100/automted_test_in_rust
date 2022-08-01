#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.width
    }
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}
pub fn greeting(name: &str) -> String {
    format!("hello {:?}", name)
}
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("gues value must be between 1 and 100, got {}", value)
        }
        Guess { value }
    }
}
#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}
#[test]
fn greeting_contains_name() {
    let results = greeting("Ndeta");
    assert!(
        results.contains("Ndeta"),
        "greeting did not contain name, value was `{}`",
        results
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // Testing that a larger rectangle can hold a smaller rectangle, test will pass
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn it_adds_two() {
        assert_eq!(6, add_two(4));
    }
    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 9,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn another() {
        panic!("Make this test fail!");
    }
}
