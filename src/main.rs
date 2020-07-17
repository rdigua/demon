use demon::functions::{DirFile, exist_file};
use rusqlite::{Connection, Result, NO_PARAMS};
use std::io::Error;

//use demon::functions::{DirFile,exist_file,file_exist,exist_dir};
fn main() {
    let day_db = Connection::open("day.db");
    match day_db {
        Ok(db) => {
            if let Ok(k) = db.execute(
                "CREATE TABLE day_dealing(number integer,date integer,open integer,high integer,low integer,close integer,amount integer,vol integer,reservation integer, constraint keyname primary key (number,date));",
                NO_PARAMS,
            ) {
                println!("{:#?}", k);
            } else {
                // eprintln!(Error::last_os_error());
                println!("some errors.");
            }
        }
        Err(e) => println!("{:#?}", e),
    }
    match exist_file("src") {
        DirFile::File => { println!("Readme.md is file.") }
        DirFile::Directory => { println!("src is directory.") }
        DirFile::None => { println!("Readme.md is not exists.") }
    }
    //println!("{}",file_exist("go"));
}
