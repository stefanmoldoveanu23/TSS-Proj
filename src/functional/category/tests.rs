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
                    x : 3.6,
                    y: 1.4
                }
            ]
        }
    }

    #[test]
    fn test_1() {
        env::set_var("n", "-7");
        env::set_var("paths", "./src/functional/category/test_1.json");
        test(get_pencil());
    }

    #[test]
    fn test_2() {
        env::set_var("n", "0");
        env::set_var("paths", "./src/functional/category/test_1.json");
        test(get_pencil());
    }

    #[test]
    fn test_3() {
        env::set_var("n", "1");
        env::set_var("paths", "");
        test(get_pencil());
    }

    #[test]
    fn test_4() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/category/test_10.json,./src/functional/category/test_correct.json");
        test(get_pencil());
    }

    #[test]
    fn test_5() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/category/test_1.json");
        test(get_pencil());
    }

    #[test]
    fn test_6() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/category/test_2.json");
        test(get_pencil());
    }

    #[test]
    fn test_7() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/category/test_3.json");
        test(get_pencil());
    }

    #[test]
    fn test_8() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/category/test_4.json");
        test(get_pencil());
    }

    #[test]
    fn test_9() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/category/test_5.json");
        test(get_pencil());
    }

    #[test]
    fn test_10() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/category/test_6.json");
        test(get_pencil());
    }

    #[test]
    fn test_11() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/category/test_7.json");
        test(get_pencil());
    }

    #[test]
    fn test_12() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/category/test_8.json");
        test(get_pencil());
    }

    #[test]
    fn test_13() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/category/test_9.json");
        test(get_pencil());
    }

    #[test]
    fn test_14() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/category/test_10.json");
        test(get_pencil());
    }

    #[test]
    fn test_15() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/category/test_10.json");
        test(get_pencil());
    }

    #[test]
    fn test_16() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/category/test_10.json,./src/functional/category/test_10.json,./src/functional/category/test_10.json");
        test(get_pencil());
    }

    #[test]
    fn test_17() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/category/test_10.json,./src/functional/category/test_1.json");
        test(get_pencil());
    }

    #[test]
    fn test_18() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/category/test_10.json,./src/functional/category/test_2.json");
        test(get_pencil());
    }

    #[test]
    fn test_19() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/category/test_10.json,./src/functional/category/test_3.json");
        test(get_pencil());
    }

    #[test]
    fn test_20() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/category/test_10.json,./src/functional/category/test_4.json");
        test(get_pencil());
    }

    #[test]
    fn test_21() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/category/test_10.json,./src/functional/category/test_5.json");
        test(get_pencil());
    }

    #[test]
    fn test_22() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/category/test_10.json,./src/functional/category/test_6.json");
        test(get_pencil());
    }
    #[test]
    fn test_23() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/category/test_10.json,./src/functional/category/test_7.json");
        test(get_pencil());
    }

    #[test]
    fn test_24() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/category/test_10.json,./src/functional/category/test_8.json");
        test(get_pencil());
    }

    #[test]
    fn test_25() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/category/test_10.json,./src/functional/category/test_9.json");
        test(get_pencil());
    }

    #[test]
    fn test_26() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/category/test_10.json,./src/functional/category/test_10.json");
        test(get_pencil());
    }
}