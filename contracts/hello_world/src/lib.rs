pub fn hello_world() {
    println!("Hola, mundo!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hello_world() {
        hello_world();
    }
}
