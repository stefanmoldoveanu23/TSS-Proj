#[cfg(test)]
mod batch_1 {
    use std::io::Write;
    use std::{env, fs};
    use crate::{main, Pencil, Point};

    fn setup_env(n: i32, paths: &str) {
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
    }

    fn create_temp_json_file(content: &str, file_name: &str) -> String {
        let path = format!("./{}", file_name);
        fs::write(&path, content.as_bytes()).expect("Failed to write to file");
        path
    }

    #[test]
    fn test_valid_single_pencil() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": 0.5, "y": 0.5 },
                { "x": -1.0, "y": -1.0 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "valid_single_pencil.json");
        setup_env(1, &path);

        main();

        // The expected output should be validated through the console output or further assertions.
    }

    #[test]
    fn test_valid_multiple_pencils() {
        let content1 = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": 0.5, "y": 0.5 }
            ]
        }
        "#;
        let content2 = r#"
        {
            "start": { "x": 3.0, "y": 4.0 },
            "offsets": [
                { "x": 1.5, "y": 1.5 },
                { "x": -2.0, "y": -2.0 }
            ]
        }
        "#;
        let path1 = create_temp_json_file(content1, "valid_multiple_pencil1.json");
        let path2 = create_temp_json_file(content2, "valid_multiple_pencil2.json");
        let paths = format!("{},{}", path1, path2);
        setup_env(2, &paths);

        main();

        // Validate the output for multiple pencil objects.
    }

    #[test]
    #[should_panic(expected = "The pencil needs to be a json object.")]
    fn test_invalid_json_format() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": "invalid_format"
        }
        "#;
        let path = create_temp_json_file(content, "invalid_json_format.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    #[should_panic(expected = "Field start needs to be in pencil object.")]
    fn test_missing_start_field() {
        let content = r#"
        {
            "offsets": [
                { "x": 0.5, "y": 0.5 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "missing_start_field.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    #[should_panic(expected = "Field offsets needs to be in pencil object.")]
    fn test_missing_offsets_field() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 }
        }
        "#;
        let path = create_temp_json_file(content, "missing_offsets_field.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    #[should_panic(expected = "There should be 2 paths.")]
    fn test_invalid_path_number() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": 0.5, "y": 0.5 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "valid_single_pencil.json");
        setup_env(2, &path);

        main();
    }

    #[test]
    fn test_valid_pencil_deserialization() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": 0.5, "y": 0.5 },
                { "x": -1.0, "y": -1.0 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "valid_pencil_deserialization.json");
        setup_env(1, &path);

        let result = std::panic::catch_unwind(|| {
            main();
        });
        assert!(result.is_ok(), "Pencil should be deserialized successfully");
    }

    #[test]
    fn test_pencil_equality() {
        let pencil1 = Pencil {
            start: Point { x: 1.0, y: 2.0 },
            offsets: vec![Point { x: 0.5, y: 0.5 }]
        };
        let pencil2 = Pencil {
            start: Point { x: 1.0, y: 2.0 },
            offsets: vec![Point { x: 0.5, y: 0.5 }]
        };
        assert_eq!(pencil1, pencil2, "Pencils should be equal");
    }
}

#[cfg(test)]
mod batch_2 {
    use std::{env, fs};
    use json::{object, JsonValue, array};
    use crate::{main, Pencil, Point, Serializer};

    fn setup_env(n: i32, paths: &str) {
        env::set_var("n", n.to_string());
        env::set_var("paths", paths);
    }

    fn create_temp_json_file(content: &str, file_name: &str) -> String {
        let path = format!("./{}", file_name);
        fs::write(&path, content.as_bytes()).expect("Failed to write to file");
        path
    }

    #[test]
    fn test_valid_single_pencil() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": 0.5, "y": 0.5 },
                { "x": -1.0, "y": -1.0 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "valid_single_pencil.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    fn test_valid_multiple_pencils() {
        let content1 = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": 0.5, "y": 0.5 }
            ]
        }
        "#;
        let content2 = r#"
        {
            "start": { "x": 3.0, "y": 4.0 },
            "offsets": [
                { "x": 1.5, "y": 1.5 },
                { "x": -2.0, "y": -2.0 }
            ]
        }
        "#;
        let path1 = create_temp_json_file(content1, "valid_multiple_pencil1.json");
        let path2 = create_temp_json_file(content2, "valid_multiple_pencil2.json");
        let paths = format!("{},{}", path1, path2);
        setup_env(2, &paths);

        main();
    }

    #[test]
    #[should_panic(expected = "The pencil needs to be a json object.")]
    fn test_invalid_json_format() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": "invalid_format"
        }
        "#;
        let path = create_temp_json_file(content, "invalid_json_format.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    #[should_panic(expected = "Field start needs to be in pencil object.")]
    fn test_missing_start_field() {
        let content = r#"
        {
            "offsets": [
                { "x": 0.5, "y": 0.5 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "missing_start_field.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    #[should_panic(expected = "Field offsets needs to be in pencil object.")]
    fn test_missing_offsets_field() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 }
        }
        "#;
        let path = create_temp_json_file(content, "missing_offsets_field.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    #[should_panic(expected = "There should be 2 paths.")]
    fn test_invalid_path_number() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": 0.5, "y": 0.5 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "valid_single_pencil.json");
        setup_env(2, &path);

        main();
    }

    #[test]
    fn test_valid_pencil_deserialization() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": 0.5, "y": 0.5 },
                { "x": -1.0, "y": -1.0 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "valid_pencil_deserialization.json");
        setup_env(1, &path);

        let result = std::panic::catch_unwind(|| {
            main();
        });
        assert!(result.is_ok(), "Pencil should be deserialized successfully");
    }

    #[test]
    fn test_pencil_equality() {
        let pencil1 = Pencil {
            start: Point { x: 1.0, y: 2.0 },
            offsets: vec![Point { x: 0.5, y: 0.5 }]
        };
        let pencil2 = Pencil {
            start: Point { x: 1.0, y: 2.0 },
            offsets: vec![Point { x: 0.5, y: 0.5 }]
        };
        assert_eq!(pencil1, pencil2, "Pencils should be equal");
    }

    #[test]
    #[should_panic(expected = "Field x needs to be a number in point object.")]
    fn test_invalid_x_in_start() {
        let content = r#"
        {
            "start": { "x": "invalid", "y": 2.0 },
            "offsets": [
                { "x": 0.5, "y": 0.5 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "invalid_x_in_start.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    #[should_panic(expected = "Field y needs to be a number in point object.")]
    fn test_invalid_y_in_start() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": "invalid" },
            "offsets": [
                { "x": 0.5, "y": 0.5 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "invalid_y_in_start.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    #[should_panic(expected = "Field x needs to be a number in point object.")]
    fn test_invalid_x_in_offsets() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": "invalid", "y": 0.5 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "invalid_x_in_offsets.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    #[should_panic(expected = "Field y needs to be a number in point object.")]
    fn test_invalid_y_in_offsets() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": 0.5, "y": "invalid" }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "invalid_y_in_offsets.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    #[should_panic(expected = "The pencil needs to be a json object.")]
    fn test_not_a_json_object() {
        let content = r#"
        "this is not a json object"
        "#;
        let path = create_temp_json_file(content, "not_a_json_object.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    fn test_empty_offsets() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": []
        }
        "#;
        let path = create_temp_json_file(content, "empty_offsets.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    fn test_large_offsets() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": 1000000.0, "y": 1000000.0 },
                { "x": -1000000.0, "y": -1000000.0 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "large_offsets.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    fn test_zero_offsets() {
        let content = r#"
        {
            "start": { "x": 0.0, "y": 0.0 },
            "offsets": [
                { "x": 0.0, "y": 0.0 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "zero_offsets.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    fn test_negative_coordinates() {
        let content = r#"
        {
            "start": { "x": -1.0, "y": -2.0 },
            "offsets": [
                { "x": -0.5, "y": -0.5 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "negative_coordinates.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    fn test_valid_pencil_serialization() {
        let pencil = Pencil {
            start: Point { x: 1.0, y: 2.0 },
            offsets: vec![
                Point { x: 0.5, y: 0.5 },
                Point { x: -1.0, y: -1.0 }
            ]
        };
        let serialized = pencil.serialize();
        let expected = object!{
            "start" => object!{
                "x" => 1.0,
                "y" => 2.0
            },
            "offsets" => array![
                object!{
                    "x" => 0.5,
                    "y" => 0.5
                },
                object!{
                    "x" => -1.0,
                    "y" => -1.0
                }
            ]
        };
        assert_eq!(JsonValue::Object(serialized), expected);
    }

    #[test]
    #[should_panic(expected = "N needs to be greater than 0.")]
    fn test_zero_n_value() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": 0.5, "y": 0.5 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "zero_n_value.json");
        setup_env(0, &path);

        main();
    }

    #[test]
    fn test_no_offsets() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": []
        }
        "#;
        let path = create_temp_json_file(content, "no_offsets.json");
        setup_env(1, &path);

        main();
    }

    #[test]
    fn test_single_offset() {
        let content = r#"
        {
            "start": { "x": 1.0, "y": 2.0 },
            "offsets": [
                { "x": 0.5, "y": 0.5 }
            ]
        }
        "#;
        let path = create_temp_json_file(content, "single_offset.json");
        setup_env(1, &path);

        main();
    }
}