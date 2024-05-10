#[cfg(test)]
mod tests {
    use std::env;
    use crate::{test, Pencil, Point};
    
    fn get_pencil() -> Pencil {
        Pencil {
            start: Point { x: 5.0, y: 2.0 },
            offsets: vec! [
                Point { x: 1.0, y: 1.0 },
                Point { x: 2.0, y: 2.0 }
            ]
        }
    }

    #[test]
    #[should_panic(expected = "Field start needs to be in pencil object.")]
    fn test_1() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/equivalence/test_1.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field start needs to be an object in pencil object.")]
    fn test_2() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/equivalence/test_2.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field x needs to be in point object.")]
    fn test_3() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/equivalence/test_3.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field x needs to be a number in point object.")]
    fn test_4() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/equivalence/test_4.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field y needs to be in point object.")]
    fn test_5() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/equivalence/test_5.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field y needs to be a number in point object.")]
    fn test_6() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/equivalence/test_6.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field offsets needs to be an array in pencil object.")]
    fn test_7() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/equivalence/test_7.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "Field offset needs to only have objects in pencil object.")]
    fn test_8() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/equivalence/test_8.json,./src/functional/equivalence/test_10.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn test_9() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/equivalence/test_9.json");
        test(get_pencil());
    }

    #[test]
    fn test_10() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/equivalence/test_10.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "N needs to be greater than 0.")]
    fn test_11() {
        env::set_var("n", "-7");
        env::set_var("paths", "./src/functional/equivalence/test_10.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "There should be 2 paths.")]
    fn test_12() {
        env::set_var("n", "2");
        env::set_var("paths", "./src/functional/equivalence/test_10.json");
        test(get_pencil());
    }

    #[test]
    #[should_panic(expected = "There should only be 1 paths.")]
    fn test_13() {
        env::set_var("n", "1");
        env::set_var("paths", "./src/functional/equivalence/test_10.json,./src/functional/equivalence/test_9.json");
        test(get_pencil());
    }

}