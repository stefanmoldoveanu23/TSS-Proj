#[cfg(test)]
mod tests {
    use std::env;
    use crate::{Pencil, Point, test};

    fn get_pencil() -> Pencil {
        Pencil {
            start: Point {
                x: 10.0,
                y: 4.6
            },
            offsets: vec![
                Point {
                    x: 2.4,
                    y: 3.0
                },
                Point {
                    x: 3.6,
                    y: 1.4
                }
            ]
        }
    }

    #[test]
    fn test_1() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/mutants/test_1.json");
        test(get_pencil());
    }
    
    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn test_2() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/mutants/test_2.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn test_3() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/mutants/test_3.json");
        test(get_pencil());
    }
}