fn main() {
    for i in 0..10 { // set up ten threads
	let k=i.clone();
        let handle=std::thread::spawn(move || {
            println!("I am printing something {}",k.clone());
        });
 
 handle.join();
    //for _ in 0..1_000_000 { // make the program declare "let x = 9" one million times
    //    let _x = 9;
    //}
   }   // Now the threads start.
}    