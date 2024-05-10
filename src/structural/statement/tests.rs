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
                    x : 0.0,
                    y: 10.0
                }
            ]
        }
    }

    #[test]
    fn test_1() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/statement/test_1.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "The pencil needs to be a json object.")]
    fn test_2() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/statement/test_2.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field start needs to be in pencil object.")]
    fn test_3() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/statement/test_3.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field start needs to be an object in pencil object.")]
    fn test_4() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/statement/test_4.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field x needs to be in point object.")]
    fn test_5() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/statement/test_5.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field offsets needs to be an array in pencil object.")]
    fn test_6() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/statement/test_6.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field x needs to be a number in point object.")]
    fn test_7() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/statement/test_7.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field offset needs to only have objects in pencil object.")]
    fn test_8() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/statement/test_8.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field y needs to be in point object.")]
    fn test_9() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/statement/test_9.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field y needs to be a number in point object.")]
    fn test_10() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/structural/statement/test_10.json");
        test(get_pencil());
    }
}