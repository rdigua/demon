
use demon::functions::{DirFile,exist_file,file_exist,exist_dir};
fn main() {
   match exist_file("go") {
       DirFile::File=>{println!("Readme.md is file.")},
       DirFile::Directory=>{println!("Readme.md is directory.")},
       DirFile::None=>{println!("Readme.md is not exists.")},
   }
    println!("{}",file_exist("go"));

}
