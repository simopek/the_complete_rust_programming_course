enum Shape {
    Triangle,
    Square,
    Pentagon,
    Octagon,
}

impl Shape {
    fn corners(&self) -> i8 {
        match self {
            Shape::Square => 4,
            Shape::Octagon => 8,
            Shape::Triangle => 3,
            Shape::Pentagon => 5,
        }
    }
}

fn main() {
    println!("{}", Shape::Triangle.corners());
    println!("{}", Shape::Octagon.corners());
    println!("{}", Shape::Pentagon.corners());
    println!("{}", Shape::Square.corners());
}
