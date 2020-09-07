//use futures::executor::block_on;

//async fn hello_world(show:u32) {
//    println!("hello, world! {} ",show);
//}
use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use std::fs::File;
use std::io::Read;
//use std::io::Write;
//use std::path::Path;
//use std::io::prelude::*;
use std::io::Error;
use first::functions::*;
//use byteorder::LittleEndian;

fn main() -> std::io::Result<()> {
    let mut string_file:String="".to_string();
    //let mut num:usize=0;
    //let f_c:Result<usize>;
    let mut buffer: std::vec::Vec<u8>=Vec::new();
    //let mut a_u8: [u8; 4] = [0, 0, 0, 0];
    //let mut u32_buffer:Vec<u32>=Vec::new();
    if let Ok(mut f)=File::open("D:\\wk\\stockdata\\sh999999.day"){
        let f_c=f.read_to_end(&mut buffer);
        println!("{:#?}",f_c);
    } else {
        println!("It was opened file error.");
        return Err(Error::last_os_error())
    }
    let u32_buffer:Vec<u32>=u8_to_u32(buffer.clone());



    //let y:[u32;9] = [1,2,3,4,5,6,7,8,9];
//    let split_y: Vec<_> = buffer.chunks(4).collect();
    //print!("{:?}", split_y); // [[1, 2, 3, 4], [5, 6, 7, 8], [9]]
//    for i in split_y.iter(){
 //        u32_buffer.push(u32::from_le_bytes(i));
 //   }
    /*
    let mut byte_u8:[u8;4]=[0;4];
    for (i, n) in buffer.iter().enumerate() {
        let k = (i %4) as usize;
        if (i % 4)==0 && i !=0 {
            u32_buffer.push(u32::from_le_bytes(byte_u8));
          //  print!("{:#?},{}",byte_u8,n);
        };
        byte_u8[k]=*n;
    }
    u32_buffer.push(u32::from_le_bytes(byte_u8));

    for _i in buffer.iter(){

        if (num % (4 as usize)) == 0 {

            match buffer.iter().nth(num){
                Some(ok) => a_u8[0] = *ok,
                _ => return Err(Error::last_os_error())
            }
            match buffer.iter().nth(num+1){
                Some(ok) => a_u8[1]= *ok,
                _ => return Err(Error::last_os_error())
            }
            match buffer.iter().nth(num+2){
                Some(ok) => a_u8[2] = *ok,
                _ => return Err(Error::last_os_error())
            }
            match buffer.iter().nth(num+3){
                Some(ok) => a_u8[3] = *ok,
                _ => return Err(Error::last_os_error())
            }
            /*
                a_u8[0]=*buffer.iter().nth(num).unwrap();
                a_u8[1]=*buffer.iter().nth(num+1).unwrap();
                a_u8[2]=*buffer.iter().nth(num+2).unwrap();
                a_u8[3]=*buffer.iter().nth(num+3).unwrap();
            //println!("{}",u32::from_le_bytes(a_u8));

             */
                u32_buffer.push(u32::from_le_bytes(a_u8));

        }
        num += 1;
    }

     */



    println!("{}",buffer.len());
    println!("u32_buffer length is {}",u32_buffer.len());
    //num=0;
    for (n,i) in u32_buffer.iter().enumerate(){
        if n % 8 ==0 && n !=0 {
            string_file.push_str("\n ");
        }
        string_file.push_str(&i.to_string());
        string_file.push_str(" ");
    }
    /*
    for i in u32_buffer.iter(){
        if (num%8)==0 && num != 0 {
            string_file.push_str("\n ");
        }
        string_file.push_str(&i.to_string());
        string_file.push_str(" ");

        num += 1;
    }
    */


    if string_file.len()>0 {
        str_file("sh999999.txt",&string_file);
    }
    let d = Connection::open("f.db");
    match d {
        Ok(db) => println!("{:#?}", db),
        Err(e) => println!("{:#?}", e),
    }
    let d = Connection::open("f.db");
    match d {
        Ok(db) => {
            if let  Ok(k)=db.execute(
                "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
                NO_PARAMS,
            ) {
                println!("{:#?}",k);
            } else {
                return Err(Error::last_os_error());
            }
        },
        Err(e) => println!("{:#?}", e),
    }
    //from_le_bytes
    Ok(())

}

/*
fn str_file(p: &str, save_str: &str) -> bool {
    if !Path::new(p).exists() {
        let mut f = File::create(p).expect("It is not create file.");
        f.write_all(save_str.as_bytes()).expect("Wrote failed.");
        return true;
    }
    false
}

 */