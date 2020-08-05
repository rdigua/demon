fn do_something<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn main() {
    do_something(|| {
        let some_vec = vec![9, 8, 10];
        some_vec
            .iter()
            .for_each(|x| println!("The number is: {}", x));
    })
}