pub mod contract;
pub mod error;
pub mod responses;
pub mod whitelist;
pub mod whitelist_impl;
#[cfg(test)]
pub mod multitest;

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
