
fn main(){
    let mut v:Vec<u8>=Vec::new();
    v.push(01);
    v.push(02);
    v.push(03);
    v.push(04);
    let mut v32:Vec<u32>=Vec::new();
    for i in v.iter(){
        println!("{}",i);
    }
}