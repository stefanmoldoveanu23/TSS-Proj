use std::fmt::{Debug, Display, Formatter};
use std::{env, fs};
use json::{JsonValue, object::Object};
mod functional;
mod mutants;
mod structural;
mod chatgpt_tests;

fn main() {
    let n :i32= env::var("n").unwrap().parse().unwrap();
    assert!(n > 0, "N needs to be greater than 0.");

    let binding = env::var("paths").unwrap();
    let mut paths = binding.split(",");

    for _ in 0..n {
        let path = paths.next().expect(&*format!("There should be {} paths.", n));
        let json = json::parse(std::str::from_utf8(fs::read(path).unwrap().as_slice()).unwrap()).unwrap();

        let pencil_end :Result<Pencil, String>= if let JsonValue::Object(object) = json {
            Deserializer::deserialize(&object)
        } else {
            Err("The pencil needs to be a json object.".into())
        };

        let pencil_end = pencil_end.unwrap();

        println!("{}", pencil_end);
    }

    assert!(paths.next().is_none(), "There should only be {} paths.", n);
}

fn test(pencil: Pencil)
{
    let n :i32= env::var("n").unwrap().parse().unwrap();
    assert!(n > 0, "N needs to be greater than 0.");

    let binding = env::var("paths").unwrap();
    let mut paths = binding.split(",");

    for _ in 0..n {
        let path = paths.next().expect(&*format!("There should be {} paths.", n));
        let json = json::parse(std::str::from_utf8(fs::read(path).unwrap().as_slice()).unwrap()).unwrap();

        let pencil_end :Result<Pencil, String>= if let JsonValue::Object(object) = json {
            Deserializer::deserialize(&object)
        } else {
            Err("The pencil needs to be a json object.".into())
        };

        let pencil_end = pencil_end.unwrap();

        assert_eq!(pencil, pencil_end);

        println!("{}", pencil_end);
    }

    assert!(paths.next().is_none(), "There should only be {} paths.", n);
}

trait Serializer<T> {
    fn serialize(&self) -> T;
}

trait Deserializer<T> {
    fn deserialize(document: &T) -> Result<Self, String> where Self:Sized;
}

#[derive(Default, Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl Eq for Point {}

impl Display for Point
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("x: {}, y: {}", self.x, self.y))
    }
}

impl Debug for Point
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Serializer<Object> for Point {
    fn serialize(&self) -> Object {
        let mut point = Object::new();

        point.insert("x", JsonValue::Number(self.x.into()));
        point.insert("y", JsonValue::Number(self.y.into()));

        point
    }
}

impl Deserializer<Object> for Point {
    fn deserialize(document: &Object) -> Result<Self, String> {
        let mut point = Point { x: 0.0, y: 0.0 };

        if let Some(x) = document.get("x") {
            if let JsonValue::Number(x) = x {
                point.x = From::from(*x);
            } else {
                return Err("Field x needs to be a number in point object.".into());
            }
        } else {
            return Err("Field x needs to be in point object.".into());
        }

        if let Some(y) = document.get("y") {
            if let JsonValue::Number(y) = y {
                point.y = From::from(*y);
            } else {
                return Err("Field y needs to be a number in point object.".into());
            }
        } else {
            return Err("Field y needs to be in point object.".into());
        }

        Ok(point)
    }
}

#[derive(Default)]
struct Pencil {
    start: Point,
    offsets: Vec<Point>,
}

impl PartialEq for Pencil
{
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start) && (self.offsets.len() == other.offsets.len()) &&
            self.offsets.iter().zip(other.offsets.iter()).fold(
                true,
                |result, (point_1, point_2)|
                    result && (*point_1 == *point_2)
            )
    }
}

impl Display for Pencil
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("start: {}\noffsets: {:?}", self.start, self.offsets))
    }
}

impl Debug for Pencil
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Serializer<Object> for Pencil {
    fn serialize(&self) -> Object {
        let mut pencil = Object::new();

        pencil.insert("start", JsonValue::Object(self.start.serialize()));
        pencil.insert("offsets", JsonValue::Array(
            self.offsets.iter().map(|offset| JsonValue::Object((*offset).serialize())).collect()
        ));

        pencil
    }
}

impl Deserializer<Object> for Pencil {
    fn deserialize(document: &Object) -> Result<Self, String> where Self: Sized {
        let mut pencil = Pencil {
            start: Point { x: 0.0, y: 0.0 },
            offsets: vec![],
        };

        if let Some(start) = document.get("start") {
            if let JsonValue::Object(start) = start {
                match Point::deserialize(start) {
                    Ok(start) => { pencil.start = start; }
                    Err(message) => { return Err(message); }
                }
            } else {
                return Err("Field start needs to be an object in pencil object.".into());
            }
        } else {
            return Err("Field start needs to be in pencil object.".into());
        }

        if let Some(offsets) = document.get("offsets") {
            if let JsonValue::Array(offsets) = offsets {
                let offsets = offsets.iter()
                    .fold(Ok(vec![]),
                    |result, offset| {
                        match result {
                            Ok(mut offsets) => {
                                if let JsonValue::Object(object) = offset {
                                    let offset = Point::deserialize(object);
                                    match offset {
                                        Ok(offset) => {
                                            offsets.push(offset);
                                            Ok(offsets)
                                        }
                                        Err(message) => Err(message)
                                    }
                                } else {
                                    return Err("Field offset needs to only have objects in pencil object.".into());
                                }
                            }
                            err => err
                        }
                    }
                );

                match offsets {
                    Ok(offsets) => {
                        pencil.offsets = offsets;
                    }
                    Err(message) => {
                        return Err(message);
                    }
                }
            } else {
                return Err("Field offsets needs to be an array in pencil object.".into());
            }
        }

        Ok(pencil)
    }
}


