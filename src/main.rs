fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(test_private_rs::fun(), 100);
    }
}
