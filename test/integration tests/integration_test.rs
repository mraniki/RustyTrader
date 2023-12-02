// src/main.rs or src/lib.rs

// Function to be tested
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        // Add more test cases as needed
    }
}