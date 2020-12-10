pub fn add_to_waitlist() {}

pub fn private() -> Vec<i32> {
    let v = vec![1,2,3,4,5];
    v.into_iter().map(|x| x + 1).collect()
}

#[cfg(test)]
mod tests {
    use super::private;
    
    #[test]
    fn is_vec() {
        assert_eq!(private(), vec![2,3,4,5,6])
    }
}
