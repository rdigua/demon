fn main(){
    let p1="hello world".to_string();
    let p2="hello world";
    let other=p2;
    println!("{}",other);
    let other=p1;
    println!("{}",other);
}