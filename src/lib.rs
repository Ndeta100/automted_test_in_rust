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
#[test]
fn greeting_contains_name() {
    let results = greeting("Ndeta");
    assert!(
        results.contains("Ndea"),
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
