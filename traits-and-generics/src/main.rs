use std::fmt::Debug;

#[derive(Debug)]
struct Car {
    mpg: f64,
    color: String,
    top_speed: f64,
}

#[derive(Debug)]
struct Motorcycle {
    mpg: f64,
    color: String,
    top_speed: f64,
}

trait Vehicle {
    fn set_mpg(&mut self, value: f64);
    fn set_color(&mut self, value: String);
    fn set_top_speed(&mut self, value: f64);
}

impl Vehicle for Car {
    fn set_mpg(&mut self, value: f64) {
        self.mpg = value;
    }

    fn set_color(&mut self, value: String) {
        self.color = value;
    }

    fn set_top_speed(&mut self, value: f64) {
        self.top_speed = value;
    }
}

impl Vehicle for Motorcycle {
    fn set_mpg(&mut self, value: f64) {
        self.mpg = value;
    }

    fn set_color(&mut self, value: String) {
        self.color = value;
    }

    fn set_top_speed(&mut self, value: f64) {
        self.top_speed = value;
    }
}

fn print_value<T: Debug>(value: &T) {
    println!("{:?}", value);
}

fn main() {
    let mut car = Car {
        mpg: 100.2,
        color: String::from("Red"),
        top_speed: 223.0,
    };
    println!("{:?}", car);
    car.set_mpg(30.2);
    car.set_color(String::from("black"));
    car.set_top_speed(100.0);
    println!("{:?}", car);

   
    let mut motorcycle = Motorcycle {mpg: 123.2, color: String::from("black"), top_speed: 203.0};
    println!("{:?}", motorcycle);
    motorcycle.set_color(String::from("cyan"));
    println!("{:?}", motorcycle);

    print_value(&car); 
    print_value(&23);
}
