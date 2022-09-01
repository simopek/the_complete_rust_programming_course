fn main() {
    let val1 = 5;
    let val2 = 2;

    let ans = val1 % val2;

    println!("{}", ans);

    let mut vec1 = vec![2, 4, 6, 8, 10];

    println!("{:?}", vec1);

    vec1.remove(4);
    vec1.push(12);

    println!("{:?}", vec1);

    let hello = String::from("Hello");
    println!("{}", concat_string(hello));

    control_flow(40);
    control_flow(60);
}

fn concat_string(input: String) -> String {
    input + " World"
}

fn control_flow(input: i32) {
   
    if input == 1 {
        println!("The value is one");
    } else if input < 25 {
        println!("The value is less than 25");
    } else if input > 25 && input < 50 {
        println!("The value is greater than 25 but less than 50");
    } else if input > 50 {
        println!("The value is greater than 50");
    } else {
        println!("The value is just a value!");
    }

}
