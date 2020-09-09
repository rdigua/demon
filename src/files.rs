///It is functions about files.
use std::fs;
use std::io;
use std::path::Path;
pub fn file_to_string<P: AsRef<Path>>(path: P) ->io::Result<String>{
	 fs::read_to_string(path)
}

	/*
	pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String>
	
	pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fn inner(path: &Path) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut string = String::with_capacity(initial_buffer_size(&file));
        file.read_to_string(&mut string)?;
        Ok(string)
    }
    inner(path.as_ref())
	}
	use std::fs;
	use std::net::SocketAddr;

	fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let foo: SocketAddr = fs::read_to_string("address.txt")?.parse()?;
    Ok(())
	}
	
	```python
	#open text file in read mode
	text_file = open("D:/data.txt", "r")
 
	#read whole file to a string
	data = text_file.read()
 
	#close file
	text_file.close()
 
	print(data)
	
	
	 
	#open text file in read mode
	text_file = open("D:/data123.txt", "r")
 
	#read whole file to a string
	data = text_file.read()
 
	#close file
	text_file.close()
 
	print(data)
	 
import os
 
file_path = "D:/data123.txt"
 
#check if file is present
if os.path.isfile(file_path):
    #open text file in read mode
    text_file = open(file_path, "r")
 
    #read whole file to a string
    data = text_file.read()
 
    #close file
    text_file.close()
 
    print(data)
 
	```
	*/

//fn getfiletostr()->String{


//}