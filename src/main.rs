mod shapes;

use crate::shapes::{rect::Rect, circ};

fn main() {
    let rect = Rect::default();
    let rect2 = Rect::default();

    let circ = circ::Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };
    let circ2 = circ::Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };
    
    println!("{}", rect);
}