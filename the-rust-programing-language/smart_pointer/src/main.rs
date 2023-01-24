enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};
//独自のスマートポインタを定義する
use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // CustomSmartPointerをデータ`{}`とともにドロップするよ
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    //Box<T>を使ってヒープにデータを格納する
    let b = Box::new(5);
    println!("b = {}", b);
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    //参照外し演算子で値までポインタを追いかける
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    //参照外し型強制
    hello(&m);
    //hello(&(*m)[..]);

    let c = CustomSmartPointer { data: String::from("my stuff") };      // 俺のもの
    let d = CustomSmartPointer { data: String::from("other stuff") };   // 別のもの
    drop(c);
    println!("CustomSmartPointers created.");                           // CustomSmartPointerが生成された
    
}


fn hello(name: &str) {
    println!("Hello, {}!", name);
}