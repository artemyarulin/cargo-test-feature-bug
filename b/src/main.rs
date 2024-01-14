fn main() {
    println!("Running");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ok() {
        assert_eq!(2, 2)
    }
}
