pub const VERSION: &str = "0.1.0";

pub fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_greets() {
        assert_eq!(hello("world"), "Hello, world!");
    }
}
