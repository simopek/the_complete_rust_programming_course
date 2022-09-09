trait TimesTwo {
    fn times_two(&mut self);
}

#[derive(Debug)]
struct MyNum {
    val: i32,
}

impl TimesTwo for MyNum {
    fn times_two(&mut self) {
        self.val *= 2;
    }
}

#[derive(Debug)]
struct MyString {
    val: String,
}

impl TimesTwo for MyString {
    fn times_two(&mut self) {
        self.val = format!("{}{}", self.val, self.val);
    }
}

fn main() {
    let mut my_num = MyNum { val: 10 };
    let mut my_string = MyString {
        val: String::from("Hello World!"),
    };

    println!("{:?}", my_num);
    println!("{:?}", my_string);

    my_num.times_two();
    my_string.times_two();

    println!("{:?}", my_num);
    println!("{:?}", my_string);
}
