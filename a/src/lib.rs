#[cfg(test)]
mod tests {

    #[cfg(feature = "foo")]
    #[test]
    fn foo() {
        panic!("I should never run")
    }
}
