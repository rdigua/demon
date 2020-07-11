///Functions of project.
//use std::fmt;
use std::path::Path;
pub enum DirFile{
    None,
    Directory,
    File,
}

/*
impl fmt::Debug for MyEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyEnum::AB => f.pad("AB"),
        }
    }
}

 */

pub fn vec_u8_u32(s: Vec<u8>) -> Option<Vec<u32>> {
    let mut r: Vec<u32> = Vec::new();
    //if s.len()==0 || !(s.len()%4==0)
    if s.is_empty() || s.len() % 4 != 0 {
        return None;
    }
    for i in 0..s.len() {
        if (i % 4 == 0) && (i + 4 < s.len()) {
            let a: u32 = u32::from_le_bytes([s[i], s[i + 1], s[i + 2], s[i + 3]]);
            r.push(a);
        }
    }
    /*
        let mut u32_buffer:Vec<u32>=Vec::new();
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
    //! Or
    use std::convert::TryInto;
    buffer.iter_mut().into_slice().chunks(4).map(|x|u32::from_le_bytes(x.try_into().unwrap())).collect::<Vec<u32>>()
     */
    if s.is_empty() {
        None
    } else {
        Some(r)
    }
}
/// The directory is or not exist;
pub fn exist_dir(p: &str) -> bool {
    Path::new(p).exists() && Path::new(p).is_dir()
}

// The file is or not exist;
pub fn file_exist(p: &str) -> bool {
    Path::new(p).exists() && Path::new(p).is_file()
}

pub fn exist_file(p:&str)->DirFile{

    if !Path::new(p).exists() {
        return DirFile::None;
    }
    if Path::new(p).is_file(){
        return DirFile::File;
    }

    if Path::new(p).is_dir(){
        return DirFile::Directory;
    }

    DirFile::None
}