#![allow(dead_code)]
pub mod deref_example;
pub mod iter_partition_example;
pub mod default_example;
pub mod associated_type_example;
pub mod from_into_example;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
