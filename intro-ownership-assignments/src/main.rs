fn main() {
    let mut vec1 = vec![1,3,5,7];
    if first_vec_value_is_one(&vec1) {
        println!("first elem is 1");
    } else {
        println!("first elem is not 1");
    }

    vec1.push(15);
    println!("{:?}", vec1);

    let num1 = 50;
    println!("{}", add_two(num1));
}

fn first_vec_value_is_one(val: &Vec<i32>) -> bool {
    val[0] == 1 
}

fn add_two(num: i8) -> i8 {
    num + 2
}
