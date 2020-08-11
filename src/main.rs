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