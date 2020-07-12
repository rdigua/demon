
use demon::functions::*;

#[test]
fn file_dir() {
    let mut right:bool=false;
    fn f(p:&str)->bool{
        let mut r:bool=false;
        match exist_file(p){
           DirFile::File => r=true,
            DirFile::Directory => r=false,
            DirFile::None => r=false,
        }
        r
    }
    fn d(p:&str)->bool{
        let mut r:bool=false;
        match exist_file(p){
            DirFile::File => r=false,
            DirFile::Directory => r=true,
            DirFile::None => r=false,
        }
        r
    }

    fn n(p:&str)->bool{
        let mut r:bool=false;
        match exist_file(p){
            DirFile::File => r=false,
            DirFile::Directory => r=false,
            DirFile::None => r=true,
        }
        r
    }

    assert_eq!(true,f("Readme.md"));
    assert_eq!(true,d("src"));
    assert_eq!(true,n("k"));
    assert_eq!(false,d("Readme.md"));
    assert_eq!(false,f("src"));
    //assert_eq!(functions::DirFile::None,functions::exist_file(""));
}
