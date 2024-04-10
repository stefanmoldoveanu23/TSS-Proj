#[cfg(test)]
mod tests {
    use std::env;
    use crate::{test, Pencil, Point};
    
    fn get_pencil() -> Pencil {
        Pencil {
            start: Point { x: 5.0, y: 2.0 },
            offsets: vec![]
        }
    }
    
    #[test]
    fn test_1() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/boundary/test_1.json");
        test(get_pencil());
    }

    #[test]
    fn test_2() {
        env::set_var("n", "0");
        env::set_var("paths", "./src/functional/boundary/test_1.json,./src/functional/boundary/test_2.json,./src/functional/boundary/test_3.json");
        test(get_pencil());
    }
}