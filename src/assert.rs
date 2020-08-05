/*
fn return_number(number: i32) -> i32 {
    println!("Here is your number.");
    number
}

fn main() {
    let number = return_number(5);
}

fn return_number<MyType>(number: MyType) -> MyType {
    println!("Here is your number.");
    number
}

fn main() {
    let number = return_number(5);
}

//use std::fmt::Debug;
fn print_number<T: std::fmt::Debug>(number: T) {
    println!("Here is your number: {:?}", number); // ⚠️
}

fn main() {
    print_number(5);
}


fn main() {
    fn main() {
        let my_name = "Loki Laufeyson";

        assert!(
            my_name == "Loki Laufeyson",
            "{} should be Loki Laufeyson",
            my_name
        );
        assert_eq!(
            my_name, "Loki Laufeyson",
            "{} and Loki Laufeyson should be equal",
            my_name
        );
        assert_ne!(
            my_name, "Mithridates",
            "{} must not equal Mithridates",
            my_name
        );
    }
        let my_name = "Mithridates";

        assert_ne!(
            my_name, "Mithridates",
            "{} must not equal Mithridates",
            my_name
        );
}


fn main() {
    let my_vec = vec![9, 0, 10];
    let fourth = get_fourth(&my_vec);
}

fn get_fourth(input: &Vec<i32>) -> i32 {
    let fourth = input.get(3).expect("Input vector needs at least 4 items");
    *fourth
}

fn try_two_unwraps(input: Vec<Option<i32>>) {
     println!("Index 0 is: {}", input[0].unwrap());
     println!("Index 1 is: {}", input[1].unwrap());
 }

 fn main() {
     let vector = vec![None, Some(1000)]; // This vector has a None, so it will panic
     try_two_unwraps(vector);
 }
*/
fn main() {
    let my_vec = vec![8, 9, 10,11];

    let fourth = my_vec.get(3).unwrap_or(&0); // If .get doesn't work, we will make the value &0.
                                              // .get returns a reference, so we need &0 and not 0
                                              // to keep the types the same.
                                              // You can write "let *fourth" with a * if you want fourth to be
                                              // a 0 and not a &0, but here we just print so it doesn't matter

    println!("{}", fourth);
}