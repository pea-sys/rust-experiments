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