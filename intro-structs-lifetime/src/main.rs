#[derive(Debug)]
struct Car {
    mpg: String,
    color: String,
    top_speed: f64,
}

impl Car {
    fn set_mpg(&mut self, value: String) {
        self.mpg = value;
    }

    fn set_color(&mut self, value: String) {
        self.color = value;
    }

    fn set_top_speed(&mut self, value: f64) {
        self.top_speed = value;
    }
}

fn main() {
    let mut car = Car {
        mpg: String::from("Mpg value"),
        color: String::from("Red"),
        top_speed: 223.0,
    };
    println!("{:?}", car);
    car.set_mpg(String::from("new mpg value"));
    car.set_color(String::from("black"));
    car.set_top_speed(100.0);
    println!("{:?}", car);
}
