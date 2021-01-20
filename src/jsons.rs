extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Line {
    points: Vec<Point>,
    valid: bool,
    length: f32,
    pub desc: String
}

// // pub fn test1() {
// //     let point1: Point = Point {x:1.0, y:2.0};
// //     let point2: Point = Point {x:3.0, y:4.0};
// //
// //     let line = Line { points: vec![point1, point2], valid: true, length: 1.0, desc: "a thin line".to_string() };
// //     let line2 = Line { points: vec![point1, point2], valid: true, length: 1.0, desc: "a thin line".to_string() };
// //     let line3 = Line { points: vec![point1, point2], valid: true, length: 1.0, desc: "a thin line".to_string() };
// //
// //     let lines = vec![line, line2, line3];
// //     let serializedLines = serde_json::to_string(&lines).unwrap();
// //     println!("serializedLines {}", serializedLines);
// //
// //     let lined: Vec<Line> = serde_json::from_str(&serializedLines).unwrap();
// //     println!("result of  {}", lined[0].desc);
// // }
//
pub fn getJsonBinary() -> Vec<u8> {
    let point1: Point = Point {x:1.0, y:2.0};
    let point2: Point = Point {x:3.0, y:4.0};

    let line = Line { points: vec![point1, point2], valid: true, length: 1.0, desc: "a thin line".to_string() };
    let line2 = Line { points: vec![point1, point2], valid: true, length: 1.0, desc: "a thin line".to_string() };
    let line3 = Line { points: vec![point1, point2], valid: true, length: 1.0, desc: "a thin line".to_string() };

    let lines = vec![line, line2, line3];
    serde_json::to_string(&lines).unwrap().into_bytes()
}