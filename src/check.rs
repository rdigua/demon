/*
#![allow(dead_code)] // Tell the compiler to be quiet
//#[derive(Debug)]

use std::mem::size_of; // This gives the size of a type

trait JustATrait {} // We will implement this on everything

trait JustATrait:std::fmt::Debug{
fn JustATrait(&self) -> String;
}


trait Alias: Trait<Item=char> {}
impl<T: Trait<Item=char>> Alias for T {}

impl<EnumOfNumbers: JustATrait:std::fmt::Debug>JustATrait for EnumOfNumbers {
    fn JustATrait(&self) -> String{
        i8.to_string()
    }
}


enum EnumOfNumbers {
    I8(i8),
    AnotherI8(i8),
    OneMoreI8(i8),
}

impl JustATrait for EnumOfNumbers{}

struct StructOfNumbers {
    an_i8: i8,
    another_i8: i8,
    one_more_i8: i8,
}
impl JustATrait for StructOfNumbers {}

enum EnumOfOtherTypes {
    I8(i8),
    AnotherI8(i8),
    Collection(Vec<String>),
}
impl JustATrait for EnumOfOtherTypes {}

struct StructOfOtherTypes {
    an_i8: i8,
    another_i8: i8,
    a_collection: Vec<String>,
}
impl JustATrait for StructOfOtherTypes {}

struct ArrayAndI8 {
    array: [i8; 1000], // This one will be very large
    an_i8: i8,
    in_u8: u8,
}
impl JustATrait for ArrayAndI8 {}

impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value a: {}, value b: {})", self.a, self.b)
    }
}

fn returns_just_a_trait() -> Box<dyn JustATrait> {
    let some_enum = EnumOfNumbers::I8(8);
    Box::new(some_enum)
}



fn main() {
    println!(
        "{}, {}, {}, {}, {}",
        size_of::<EnumOfNumbers>(),
        size_of::<StructOfNumbers>(),
        size_of::<EnumOfOtherTypes>(),
        size_of::<StructOfOtherTypes>(),
        size_of::<ArrayAndI8>(),
    );
    //println!("{:#?}",returns_just_a_trait());
    let x=&mut vec![0];
    dbg!(x);
    let x=vec![3];
    dbg!(x);
    let mut x=vec![0];
    dbg!(x);
    x=vec![3];
    dbg!(x);
}


struct Item {
    number: u8,
}

impl Item {
    fn compare_number(&self, other_number: u8) { // takes a reference to self
        println!("Are {} and {} equal? {}", self.number, other_number, self.number == other_number);
        // We don't need to write *self.number
    }
}

fn main() {
    let item = Item {
        number: 8,
    };

    let reference_item = &item; // This is type &Item
    let reference_item_two = &reference_item; // This is type &&Item

    item.compare_number(8); // the method works
    reference_item.compare_number(8); // it works here too
    reference_item_two.compare_number(8); // and here

}


#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

impl Animal {
    fn new() -> Self {
        // Self means Animal.
        //You can also write Animal instead of Self

        Self {
            // When we write Animal::new(), we always get a cat that is 10 years old
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn change_to_dog(&mut self) {
        // use .change_to_dog() to change the cat to a dog
        // with &mut self we can change it
        println!("Changing animal to dog!");
        self.animal_type = AnimalType::Dog;
    }

    fn change_to_cat(&mut self) {
        // use .change_to_dog() to change the cat to a dog
        // with &mut self we can change it
        println!("Changing animal to cat!");
        self.animal_type = AnimalType::Cat;
    }

    fn check_type(&self) {
        // we want to read self
        match self.animal_type {
            AnimalType::Dog => println!("The animal is a dog"),
            AnimalType::Cat => println!("The animal is a cat"),
        }
    }
}


#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}
fn main() {
    let mut new_animal = Animal::new(); // Associated method to create a new animal
    // It is a cat, 10 years old
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();
    use std::collections::HashMap;


    let book_collection = vec!["L'Allemagne Moderne", "Le Petit Prince", "Eye of the World", "Eye of the World"];

        let mut book_hashmap = HashMap::new();

        for book in book_collection {
            let return_value = book_hashmap.entry(book).or_insert(0);
            *return_value +=1;
        }

        for (book, number) in book_hashmap {
            println!("{:?}, {:?}", book, number);
        }

}
*/

use std::collections::VecDeque;

fn check_remaining(input: &VecDeque<(&str, bool)>) { // Each item is a (&str, bool)
    for item in input {
        if item.1 == false {
            println!("You must: {}", item.0);
        }
    }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
    let mut task_done = input.pop_back().unwrap(); // pop off the back
    task_done.1 = true;	                           // now it's done - mark as true
    input.push_front(task_done);                   // put it at the front now
}

fn main() {
    let mut my_vecdeque = VecDeque::new();
    let things_to_do = vec!["send email to customer", "add new product to list", "phone Loki back"];

    for thing in things_to_do {
        my_vecdeque.push_front((thing, false));
    }

    done(&mut my_vecdeque);
    done(&mut my_vecdeque);

    check_remaining(&my_vecdeque);

    for task in my_vecdeque {
        print!("{:?} ", task);
    }

    let mut weather_vec = vec!["Berlin", "cloudy", "5", "-7", "78"];
    while let Some(information) = weather_vec.pop() { // This means: keep going until you can't pop anymore
        // When the vector reaches 0 items, it will return None
        // and it will stop.
        if let Ok(number) = information.parse::<i32>() { // Try to parse the variable we called information
            // This returns a result. If it's Ok(number), it will print it
            println!("The number is: {}", number);
        }
    }
    let a = [1, 2, 3];

    let mut iter = a.iter();
    for i in iter{
        print!("{}",i);
    }
    for i in a.iter(){
        print!("{}",i);
    }
   // println!("{:#?}",iter);
}