#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
  return a + b
}

#[no_mangle]
pub fn reverse_bytes(bytes: Vec<u8>) -> Vec<u8> {
    bytes.iter().copied().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_add() {
        assert_eq!(add(5, 4), 9);
    }
}
