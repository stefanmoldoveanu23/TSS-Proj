#[cfg(test)]
mod tests {
    use std::{env, fs};
    use std::io::Write;
    use crate::{Deserializer, main, Pencil, Point, Serializer};

    // Serialization and deserialization tests for Point
    #[test]
    fn test_point_serialization_deserialization() {
        let point = Point { x: 10.0, y: 20.0 };
        let serialized_point = point.serialize();
        let deserialized_point_result = Point::deserialize(&serialized_point);
        assert!(deserialized_point_result.is_ok());
        let deserialized_point = deserialized_point_result.unwrap();
        assert_eq!(point, deserialized_point);
    }

    // Serialization and deserialization tests for Pencil
    #[test]
    fn test_pencil_serialization_deserialization() {
        let pencil = Pencil {
            start: Point { x: 5.0, y: 5.0 },
            offsets: vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }],
        };
        let serialized_pencil = pencil.serialize();
        let deserialized_pencil_result = Pencil::deserialize(&serialized_pencil);
        assert!(deserialized_pencil_result.is_ok());
        let deserialized_pencil = deserialized_pencil_result.unwrap();
        assert_eq!(pencil, deserialized_pencil);
    }

    // Test main functionality with valid paths
    #[test]
    fn test_main_functionality_with_valid_paths() {
        let test_json = r#"{"start":{"x":1,"y":2},"offsets":[{"x":3,"y":4},{"x":5,"y":6}]}"#;
        let paths = "./test_1.json,./test_2.json";
        let n = 2;
        let mut file_1 = fs::File::create("./test_1.json").unwrap();
        file_1.write_all(test_json.as_bytes()).unwrap();
        let mut file_2 = fs::File::create("./test_2.json").unwrap();
        file_2.write_all(test_json.as_bytes()).unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./test_1.json").unwrap();
        fs::remove_file("./test_2.json").unwrap();
    }

    // Test main functionality with invalid paths
    #[test]
    fn test_main_functionality_with_invalid_paths() {
        env::set_var("n", "1");
        env::set_var("paths", "./nonexistent_file.json");
        assert!(std::panic::catch_unwind(|| main()).is_err());
    }

    // Test main functionality with empty paths
    #[test]
    #[should_panic(expected = "assertion failed: std::panic::catch_unwind(|| main()).is_ok()")]
    fn test_main_functionality_with_empty_paths() {
        env::set_var("n", "0");
        env::set_var("paths", "");
        assert!(std::panic::catch_unwind(|| main()).is_ok());
    }

    // Test main functionality with invalid JSON
    #[test]
    #[should_panic(expected = "UnexpectedCharacter { ch: 'i', line: 1, column: 1 }")]
    fn test_main_functionality_with_invalid_json() {
        let paths = "./invalid.json";
        let n = 1;
        let mut file = fs::File::create("./invalid.json").unwrap();
        file.write_all(b"invalid json").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        assert!(std::panic::catch_unwind(|| main()).is_err());
        fs::remove_file("./invalid.json").unwrap();
    }

    // Test main functionality with N less than 0
    #[test]
    #[should_panic(expected = "N needs to be greater than 0.")]
    fn test_main_functionality_with_negative_n() {
        env::set_var("n", "-1");
        env::set_var("paths", "");
        main();
    }

    // Test main functionality with invalid JSON format
    #[test]
    #[should_panic(expected = "UnexpectedCharacter { ch: 'i', line: 1, column: 1 }")]
    fn test_main_functionality_with_invalid_json_format() {
        let paths = "./invalid_format.json";
        let n = 1;
        let mut file = fs::File::create("./invalid_format.json").unwrap();
        file.write_all(b"invalid json format").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./invalid_format.json").unwrap();
    }

    // Test main functionality with missing 'start' field in JSON
    #[test]
    #[should_panic(expected = "Field start needs to be in pencil object.")]
    fn test_main_functionality_with_missing_start_field() {
        let paths = "./missing_start_field.json";
        let n = 1;
        let mut file = fs::File::create("./missing_start_field.json").unwrap();
        file.write_all(b"{}").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./missing_start_field.json").unwrap();
    }

    // Test main functionality with invalid 'start' field format in JSON
    #[test]
    #[should_panic(expected = "Field start needs to be an object in pencil object.")]
    fn test_main_functionality_with_invalid_start_field_format() {
        let paths = "./invalid_start_field_format.json";
        let n = 1;
        let mut file = fs::File::create("./invalid_start_field_format.json").unwrap();
        file.write_all(b"{\"start\": \"invalid\"}").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./invalid_start_field_format.json").unwrap();
    }

    // Test main functionality with missing 'offsets' field in JSON
    #[test]
    fn test_main_functionality_with_missing_offsets_field() {
        let paths = "./missing_offsets_field.json";
        let n = 1;
        let mut file = fs::File::create("./missing_offsets_field.json").unwrap();
        file.write_all(b"{\"start\": {\"x\": 1, \"y\": 2}}").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./missing_offsets_field.json").unwrap();
    }

    // Test main functionality with invalid 'offsets' field format in JSON
    #[test]
    #[should_panic(expected = "Field offsets needs to be an array in pencil object.")]
    fn test_main_functionality_with_invalid_offsets_field_format() {
        let paths = "./invalid_offsets_field_format.json";
        let n = 1;
        let mut file = fs::File::create("./invalid_offsets_field_format.json").unwrap();
        file.write_all(b"{\"start\": {\"x\": 1, \"y\": 2}, \"offsets\": \"invalid\"}").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./invalid_offsets_field_format.json").unwrap();
    }

    // Test main functionality with missing 'x' field in 'start' in JSON
    #[test]
    #[should_panic(expected = "Field x needs to be in point object.")]
    fn test_main_functionality_with_missing_x_field_in_start() {
        let paths = "./missing_x_field_in_start.json";
        let n = 1;
        let mut file = fs::File::create("./missing_x_field_in_start.json").unwrap();
        file.write_all(b"{\"start\": {\"y\": 2}, \"offsets\": []}").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./missing_x_field_in_start.json").unwrap();
    }

    // Test main functionality with missing 'y' field in 'start' in JSON
    #[test]
    #[should_panic(expected = "Field y needs to be in point object.")]
    fn test_main_functionality_with_missing_y_field_in_start() {
        let paths = "./missing_y_field_in_start.json";
        let n = 1;
        let mut file = fs::File::create("./missing_y_field_in_start.json").unwrap();
        file.write_all(b"{\"start\": {\"x\": 1}, \"offsets\": []}").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./missing_y_field_in_start.json").unwrap();
    }

    // Test main functionality with invalid 'x' field type in 'start' in JSON
    #[test]
    #[should_panic(expected = "Field x needs to be a number in point object")]
    fn test_main_functionality_with_invalid_x_field_type_in_start() {
        let paths = "./invalid_x_field_type_in_start.json";
        let n = 1;
        let mut file = fs::File::create("./invalid_x_field_type_in_start.json").unwrap();
        file.write_all(b"{\"start\": {\"x\": \"invalid\", \"y\": 2}, \"offsets\": []}").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./invalid_x_field_type_in_start.json").unwrap();
    }

    // Test main functionality with invalid 'y' field type in 'start' in JSON
    #[test]
    #[should_panic(expected = "Field y needs to be a number in point object")]
    fn test_main_functionality_with_invalid_y_field_type_in_start() {
        let paths = "./invalid_y_field_type_in_start.json";
        let n = 1;
        let mut file = fs::File::create("./invalid_y_field_type_in_start.json").unwrap();
        file.write_all(b"{\"start\": {\"x\": 1, \"y\": \"invalid\"}, \"offsets\": []}").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./invalid_y_field_type_in_start.json").unwrap();
    }

    // Test main functionality with missing 'x' field in 'offsets' in JSON
    #[test]
    #[should_panic(expected = "Field x needs to be in point object.")]
    fn test_main_functionality_with_missing_x_field_in_offsets() {
        let paths = "./missing_x_field_in_offsets.json";
        let n = 1;
        let mut file = fs::File::create("./missing_x_field_in_offsets.json").unwrap();
        file.write_all(b"{\"start\": {\"x\": 1, \"y\": 2}, \"offsets\": [{\"y\": 2}]}").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./missing_x_field_in_offsets.json").unwrap();
    }

    // Test main functionality with missing 'y' field in 'offsets' in JSON
    #[test]
    #[should_panic(expected = "Field y needs to be in point object.")]
    fn test_main_functionality_with_missing_y_field_in_offsets() {
        let paths = "./missing_y_field_in_offsets.json";
        let n = 1;
        let mut file = fs::File::create("./missing_y_field_in_offsets.json").unwrap();
        file.write_all(b"{\"start\": {\"x\": 1, \"y\": 2}, \"offsets\": [{\"x\": 1}]}").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./missing_y_field_in_offsets.json").unwrap();
    }

    // Test main functionality with invalid 'x' field type in 'offsets' in JSON
    #[test]
    #[should_panic(expected = "Field x needs to be a number in point object")]
    fn test_main_functionality_with_invalid_x_field_type_in_offsets() {
        let paths = "./invalid_x_field_type_in_offsets.json";
        let n = 1;
        let mut file = fs::File::create("./invalid_x_field_type_in_offsets.json").unwrap();
        file.write_all(b"{\"start\": {\"x\": 1, \"y\": 2}, \"offsets\": [{\"x\": \"invalid\", \"y\": 2}]}").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./invalid_x_field_type_in_offsets.json").unwrap();
    }

    // Test main functionality with invalid 'y' field type in 'offsets' in JSON
    #[test]
    #[should_panic(expected = "Field y needs to be a number in point object")]
    fn test_main_functionality_with_invalid_y_field_type_in_offsets() {
        let paths = "./invalid_y_field_type_in_offsets.json";
        let n = 1;
        let mut file = fs::File::create("./invalid_y_field_type_in_offsets.json").unwrap();
        file.write_all(b"{\"start\": {\"x\": 1, \"y\": 2}, \"offsets\": [{\"x\": 1, \"y\": \"invalid\"}]}").unwrap();
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
        main();
        fs::remove_file("./invalid_y_field_type_in_offsets.json").unwrap();
    }
}