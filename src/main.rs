const MY_DATA: [i8; 3] = [1, 2, 3];

fn main() {
    let mut my_data = [2; 3];
    my_data[0] = 1;
    my_data[2] = 3;
	println!("MY_DATA is :::  {:#?}",MY_DATA);
	println!("my_data is :::{:#?}",my_data);
//    assert_eq!(MY_DATA, my_data);
}