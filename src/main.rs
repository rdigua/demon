// import everything we need
use std::{
    fs::File,
    io::{Error, Read},
};

// main is fallible, it can fail because of an I/O error
fn main() -> Result<(), Error> {
    // open the file, return early if it fails.
    // the `file` binding is `mut`, which means mutable,
    // so we can take a mutable reference to it later.
    let mut file = File::open("/etc/hosts")?;
    // create an empty string, have a mutable binding to it,
    // so we can take a mutable reference to it later.
    let mut text = String::new();
    // call `Read::read_to_string` on the file, pass it
    // a mutable reference to our destination string.
    // The signature for `read_to_string` takes `&mut self`,
    // so this line actually takes a mutable reference to file too.
    file.read_to_string(&mut text)?;
    // at this point, `file` can be dropped, because we don't
    // use it anymore. this also frees OS resources associated with it.

    // call the println macro, our format string is just `{}`,
    // which will format an argument that implements the `std::fmt::Display`
    // trait. In our case, `String` just prints its contents as, well, text.
    println!("{}", text);

    // everything went fine, signal sucess with an empty (tuple) result.
    Ok(())
}
/*
use std::{
    io::Error,
    fs::File,
};

fn main() -> Result<(), Error> {
    // `File::open()` *also* returns a `Result<File, Error>`.
    // The `?` sigil means: if it returns an error, then we
    // should also return that error. If it returns a result,
    // then assign it to file and carry on.
    let file = File::open("/etc/hosts")?;

    Ok(())
}

use std::{
    fs::File,
    io::{Error, Read},
};

fn main() -> Result<(), Error> {
    let mut file = File::open("/etc/hosts")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    println!("{}", text);

    Ok(())
}

const MY_DATA: [i8; 3] = [1, 2, 3];

fn main() {
    let mut my_data = [2; 3];
    my_data[0] = 1;
    my_data[2] = 3;
	println!("MY_DATA is :::  {:#?}",MY_DATA);
	println!("my_data is :::{:#?}",my_data);
//    assert_eq!(MY_DATA, my_data);
    let s = "112225322555";
    let n = 2;

    let i = s
        .chars()
        .map(|v| Some(v))
        .chain(std::iter::once(None))
        .scan((0, None), |(count, ch), v| match ch {
            Some(c) if *c == v => {
                *count += 1;
                Some((None, *count))
            }
            _ => Some((ch.replace(v), std::mem::replace(count, 1))),
        })
        .filter_map(|(ch, count)| match ch {
            Some(Some(ch)) if count >= n => Some((ch, count)),
            _ => None,
        });

    for (ch, num) in i {
        println!("There are {} repititions of {}", num, ch);
    }

    let lines = ["1", "2", "a","4","b","5"];

    let sum: i32 = lines
        .iter()
        .map(|line| line.parse::<i32>())
        .inspect(|num| {
            if let Err(ref e) = *num {
                println!("Parsing error: {}", e);
            }
        })
        .filter_map(Result::ok)
        .sum();

    println!("Sum: {}", sum);

}

 */