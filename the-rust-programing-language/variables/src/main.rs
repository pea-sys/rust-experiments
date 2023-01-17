fn main() {
    //可変性
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    //定数
    const MAX_POINTS: u32 = 100_000;
    //シャドーイング
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    //浮動小数点型
    let x = 2.0; // f64
    println!("The value of x is: {:.1}", x);
    let y: f32 = 3.0; // f32
    println!("The value of y is: {:.1}", y);
    //数値演算
    // addition
    // 足し算
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);
    // subtraction
    // 引き算
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);
    // multiplication
    // 掛け算
    let product = 4 * 30;
    println!("The value of  product is: {}", product);
    // division
    // 割り算
    let quotient = 56.7 / 32.2;
    println!("The value of  quotient is: {:.1}", quotient);
    let floored = 2 / 3; // Results in 0
    println!("The value of  floored is: {:.1}", floored);
    // 結果は0

    // remainder
    // 余り
    let remainder = 43 % 5;

    //論理値型
    let t = true;

    let f: bool = false; // with explicit type annotation
                         // 明示的型注釈付きで
                         //文字型
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; //ハート目の猫

    //タプル型
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
