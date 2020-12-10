#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn larger() -> Rectangle {
        Rectangle {
            width: 8,
            height: 7,
        }
    }

    fn smaller() -> Rectangle {
        Rectangle {
            width: 5,
            height: 1,
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = larger();
        let smaller = smaller();

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = larger();
        let smaller = smaller();

        assert!(!smaller.can_hold(&larger));
    }
}
