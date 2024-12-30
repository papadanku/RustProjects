pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result: i32 = add_one(2);
        assert_eq!(result, 3);
    }
}
