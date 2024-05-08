#[cfg(test)]
mod tests {
    use std::env;
    use crate::{Pencil, Point, test};

    fn get_pencil() -> Pencil {
        Pencil {
            start: Point {
                x: 5.0,
                y: 3.0
            },
            offsets: vec![
                Point {
                    x: 2.0,
                    y: 4.0
                },
                Point {
                    x: 0.0,
                    y: 10.0
                }
            ]
        }
    }

    #[test]
    fn test_1() {
        env::set_var("n", "-2");
        env::set_var("paths", "./src/structural/decision/test_1.json");
        test(get_pencil());
    }

    #[test]
    fn test_2() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/structural/decision/test_1.json");
        test(get_pencil());
    }

    #[test]
    fn test_3() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/decision/test_1.json");
        test(get_pencil());
    }

    #[test]
    fn test_4() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/decision/test_2.json");
        test(get_pencil());
    }

    #[test]
    fn test_5() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/decision/test_3.json");
        test(get_pencil());
    }

    #[test]
    fn test_6() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/decision/test_4.json");
        test(get_pencil());
    }

    #[test]
    fn test_7() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/decision/test_5.json");
        test(get_pencil());
    }

    #[test]
    fn test_8() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/decision/test_6.json");
        test(get_pencil());
    }

    #[test]
    fn test_9() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/decision/test_7.json");
        test(get_pencil());
    }

    #[test]
    fn test_10() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/structural/decision/test_8.json,./src/structural/decision/test_9.json");
        test(get_pencil());
    }
    #[test]
    fn test_11() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/decision/test_10.json");
        test(get_pencil());
    }

    #[test]
    fn test_12() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/decision/test_11.json");
        test(get_pencil());
    }

}