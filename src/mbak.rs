//use std::io;
//#![feature(inclusive_range_syntax)]

fn main() {
    /*
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);
    */
    let mut s = String::from("abc");
    unsafe { s.as_mut_vec().swap(0, 1); }
    println!("{}", s);
    let mut n: i32 = 0;
    for _ in 0..10 {
        n += 1;
    }
    println!("num = {}", n);

    println!("num = {}", (0..10).count());

    println!("0到20之间的偶数序列:");
    for i in (0..21).filter(|x| (x % 2 == 0)) {
        print!("{} ", i);
    }
    println!("0到20之间的一系列整数，它们除以2和3得到余数：");
    for i in (0..21).filter(|x| (x % 2 == 0) && (x % 3 == 0)) {
        print!("{} ", i);
    }
    println!("反转:");
    for i in (0..11).rev() {
        print!("{} ", i);
    }
    println!("从1到10的数字的正方形序列：");
    for i in (1..11).map(|x| x * x) {
        print!("{} ", i);
    }
    println!("//1到5的数字的平方和:");

    let result = (1..=5).fold(0, |acc, x| acc + x * x);
    println!("result = {}", result);

    let mut acc = 0;

    for x in 1..=5 {
        acc += x * x;
    }

    let result = acc;
    println!("result = {}", result);

    println!("数组迭代（Iterating over Arrays）");
    let cities = ["Toronto", "New York", "Melbourne"];

    for city in cities.iter() {
        print!("{}, ", city);
    }

    println!("`10`到`0`之间,步长`2`:");

    for i in (0..=10).rev().filter(|x| (x % 2 == 0)) {
        print!("{} ", i);
    }
    println!("不连续的Range:");
    let c = (1..4).chain(6..9);

    for i in c {
        print!("{} ", i);
    }

    println!("第一个递增和过滤，另一个 是递减:");
    let r = (1..20)
        .filter(|&x| x % 5 == 0)
        .chain((6..9).rev());

    for i in r {
        print!("{} ", i);
    }

    println!("两个迭代器合并为一个:");
    let cities = ["Toronto", "New York", "Melbourne"];
    let populations = [2_615_060, 8_550_405, 4_529_500];

    let matrix = cities.iter().zip(populations.iter());

    for (c, p) in matrix {
        println!("{:10}: population = {}", c, p);
    }

    let s:String="Д Е Ж З И Й К Л М Н О П".to_string();
    for c in s.chars(){
        print!("{} ", c);
    }
    println!("可变地迭代一个矢量，使处理中的每个元素加倍:");
    let mut nums = vec![1, 2, 3, 4, 5];
    for i in &mut nums {
        *i *= 2;
    }
    println!("{:?}", nums);
    println!("从一个迭代器获取值并将它们转换为所需类型的集合:");
    let v = (1..11).collect::<Vec<i32>>();
    for i in v.iter(){
        print!("{} ",i)
    }
    println!("获得向量的元素及其索引:");
    let v = vec![2, 3, 4];
    for (i, n) in v.iter().enumerate() {
        println!("v[{}] = {}", i, n);
    }

    println!("向量元素的最小值和最大值:");

    let v = vec![3, 5, 0, -2, 3, 4, 1];
    let max = v.iter().max();
    let min = v.iter().min();

    println!("max = {:?}, min = {:?}", max, min);

    println!("所有值的总和:");
    let grades = vec![4, 5, 6, 9, 7, 4, 8];
    let sum: i32 = grades.iter().sum();
    let gpa = sum as f64 / grades.len() as f64;

    println!("sum = {}, gpa = {}", sum, gpa);

    println!("返回可以被`5`整除的整数的正方形序列中的前`10`个:");
    let v = (1..)
        .map(|x| x * x)
        .filter(|x| x % 5 == 0 )
        .take(10)
        .collect::<Vec<i32>>();

    println!("{:?} ", v);
    //
    //
    println!("定制迭代器（Creating Your Own Iterators）:");

    struct FahrToCelc {
        father: f32,
        step: f32,
    }

    impl FahrToCelc {
        fn new(far: f32, step: f32) -> FahrToCelc {
            FahrToCelc { father: far, step: step }
        }
    }

    impl Iterator for FahrToCelc {
        type Item = (f32, f32);

        fn next (&mut self) -> Option<Self::Item> {
            let curr_fahr = self.father;
            let curr_celc = (self.father - 32.0) / 1.8;
            self.father = self.father + self.step;
            Some((curr_fahr, curr_celc))
        }
    }
    // pass the starting temperature and step to the initializer function
    let ftc = FahrToCelc::new(0.0, 5.0);

    // produce the iterator table of first 5 values
    let temp_table = ftc.take(5);

    // print out the temperature table nicely
    for (f, c) in temp_table {
        println!("{:7.2} °F = {:7.2} °C", f, c);
    }

    println!("A String is always {:?} bytes. It is Sized.", std::mem::size_of::<String>()); // std::mem::size_of::<Type>() gives you the size in bytes of a type
    println!("And an i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
    println!("And an f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());
    println!("But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.", std::mem::size_of_val("서태지")); // std::mem::size_of_val() gives you the size in bytes of a variable
    println!("And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Adrian Fahrenheit Țepeș"));

    let country = String::from("Austria");
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country);

    fn print_country1(country_name: String) {
        println!("{}", country_name);
    }
    let country = String::from("Austria");
    print_country1(country); // We print "Austria"
//    print_country(country); // ⚠️ That was fun, let's do it again!

    fn print_country(country_name: &String) {
        println!("{}", country_name);
    }
    let country = String::from("Austria");
    print_country(&country); // We print "Austria"
    print_country(&country); // That was fun, let's do it again!

    fn add_hungary(country_name: &mut String) {
        country_name.push_str("-Hungary"); // push_str() adds a &str to a String
        println!("Now it says: {}", country_name);
    }

    let mut country = String::from("Austria");
    add_hungary(&mut country);

    fn adds_hungary(mut country: String) { // but adds_hungary takes the string and it is mutable!
        country.push_str("-Hungary");
        println!("{}", country);
    }
    let country = String::from("Austria"); // country is not mutable
    adds_hungary(country);
}