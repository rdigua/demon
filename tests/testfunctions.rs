
use demon::functions::*;

#[test]
fn file_dir() {
    let mut right:bool=false;

        match exist_file("Readme.md"){
        DirFile::File => right=true,
            _ => right=false,
    }
    assert!(right);
    //assert_eq!(functions::DirFile::None,functions::exist_file(""));
}
