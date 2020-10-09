use std::io;
use std::fs::{self, DirEntry,};
use std::path::Path;

extern crate rustc_serialize;
use rustc_serialize::json;


// json encoding: 
#[derive(Debug, RustcEncodable)]
struct Doc {
    path: String,
    filename: String,
}


fn main() {
    let target_path = Path::new("/Users/interaction/workspace/temp/testeddocs");
    let mut docs: Vec<Doc> = Vec::new();

    fn create_handler<'a>(docs: &'a mut Vec<Doc>) -> Box<FnMut(&DirEntry) + 'a> {
        let handler = move |entry: &DirEntry| -> () {
            let doc = Doc {
                path: entry.path().to_str().unwrap().to_string(),
                filename: entry.file_name().into_string().unwrap(),
            };
            docs.push(doc);
        };

        Box::new(handler)
    }

    {
        let mut handler = create_handler(&mut docs);
        visit_dirs(&target_path, &mut |entry: &DirEntry|{
            handler(entry)
        });
    }
    println!("result json is: {}", json::encode(&docs).unwrap());
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &mut FnMut(&DirEntry)) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(visit_dirs(&entry.path(), cb));
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}