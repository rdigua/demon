//mod demon::file;
use demon::files;

fn main() {
    let s = files::file_to_string("C.toml");
    match s {
        Ok(string) => println!("{}", string),
        Err(err) => println!("{}", err),
    }
    //println!("{:#?}",s);
}