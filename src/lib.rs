#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
