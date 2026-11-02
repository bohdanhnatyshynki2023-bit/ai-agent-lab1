fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(non_snake_case)]
    fn BasicAddition() {
        assert_eq!(2 + 2, 4);
    }
}
