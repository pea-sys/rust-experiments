#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

fn main() {
    //整数のベクタをソートする
    let mut vec = vec![1, 5, 10, 2, 15];
    println!("original:{:?}",vec);
    vec.sort();
    println!("sorted:{:?}",vec);
    //floatのベクタをソートする
    let mut vec_f = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    println!("original:{:?}",vec_f);
    vec_f.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("sorted:{:?}",vec_f);
    //構造体のベクタをソートする
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    println!("original:{:?}",people);
    // Sort people by derived natural order (Name and age)
    people.sort();
    println!("sorted by name :{:?}",people);
    // Sort people by age
    people.sort_by(|a, b| b.age.cmp(&a.age));
    println!("sorted by age :{:?}",people);
}
