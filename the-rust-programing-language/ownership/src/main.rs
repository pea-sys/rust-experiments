fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    //ヒープメモリに配置されるオブhジェクトは = だと所有権がs2に移るので、もう使用不可
    //println!("{}, world!", s1); 
    println!("{}, world!", s2); 

    //オブジェクトをコピーしたい場合は、cloneでディープコピーする必要がある
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    //スタックを使用する型は、ディープコピー
    //dropトレイトがない型はこのような動作になる
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫

    let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
                                    // ムーブする

    let s2 = String::from("hello");     // s2がスコープに入る

    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ 戻り値もs3にムーブされる

    // 借用
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
                                 
    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s1, len);

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    //可変な参照
    //借用はデフォルトでは書き換え不可
    let mut s = String::from("hello");

    change(&mut s);

    //複数箇所から同じ可変ヒープオブジェクトを参照することはできない
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる

    let r2 = &mut s;
    
    //スライス型
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
                               // wordの中身は、値5になる

    s.clear(); // this empties the String, making it equal to ""
               // Stringを空にする。つまり、""と等しくする

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    // wordはまだ値5を保持しているが、もうこの値を正しい意味で使用できる文字列は存在しない。
    // wordは今や完全に無効なのだ！

    //文字列スライス
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    
    println!("{} {}",hello,world);

    let s = String::from("hello");

    let slice_a = &s[0..2];
    let slice_b = &s[..2];
    println!("{} {}",slice_a,slice_b);

    let s = String::from("hello");

    let len = s.len();

    let slice_a = &s[3..len];
    let slice_b = &s[3..];
    println!("{} {}",slice_a,slice_b);

    let s = String::from("hello");

    let len = s.len();

    let slice_a = &s[0..len];
    let slice_b = &s[..];
    println!("{} {}",slice_a,slice_b);

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    // first_wordは`String`のスライスに対して機能する
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    // first_wordは文字列リテラルのスライスに対して機能する
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    // 文字列リテラルは「それ自体すでに文字列スライスなので」、
    // スライス記法なしでも機能するのだ！
    let word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  // 

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
    // 呼び出した関数にムーブする

let some_string = String::from("hello"); // some_stringがスコープに入る

    some_string                              // some_stringが返され、呼び出し元関数に
    // ムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。

    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}