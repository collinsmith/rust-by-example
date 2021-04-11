fn main() {
    let hello_world = hello_world();
    println!("{}", hello_world);
}

const fn hello_world() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_identity() {
        let hello_world = hello_world();
        assert_eq!(hello_world, hello_world);
    }

    #[test]
    fn hello_world_symmetric() {
        let hello_world = hello_world();
        assert_eq!(hello_world, "Hello, world!");
    }

    #[test]
    fn hello_world_asymmetric() {
        let hello_world = hello_world();
        assert_ne!(hello_world, "Hello, world");
    }
}
