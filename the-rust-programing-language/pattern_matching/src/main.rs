struct Point {
    x: i32,
    y: i32,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}
fn foo(_: i32, y: i32) {
    // このコードは、y引数を使うだけです: {}
    println!("This code only uses the y parameter: {}", y);
}
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        // あなたのお気に入りの色、{}を背景色に使用します
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        // 火曜日は緑の日！
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            // 紫を背景色に使用します
            println!("Using purple as the background color");
        } else {
            // オレンジを背景色に使用します
            println!("Using orange as the background color");
        }
    } else {
        // 青を背景色に使用します
        println!("Using blue as the background color");
    }


    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (x, y, z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);

    
    if let x = 5 {
        println!("{}", x);
    };

    let x = 1;

    match x {
        1 => println!("one"),       // 1
        2 => println!("two"),       // 2
        3 => println!("three"),     // 3
        _ => println!("anything"),  // なんでも
    }


    let x = Some(5);
    let y = 10;

    match x {
        // 50だったよ
        Some(50) => println!("Got 50"),
        // マッチしたよ
        Some(y) => println!("Matched, y = {:?}", y),
        // 既定のケース
        _ => println!("Default case, x = {:?}", x),
    }

    // 最後にはx = {}, y = {}
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;

    match x {
        // 1か2
        1 | 2 => println!("one or two"),
        // 3
        3 => println!("three"),
        // なんでも
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        // 1から5まで
        1..=5 => println!("one through five"),
        // それ以外
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        // ASCII文字前半
        'a'..='j' => println!("early ASCII letter"),
        // ASCII文字後半
        'k'..='z' => println!("late ASCII letter"),
        // それ以外
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);


    match p {
        // x軸上の{}
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // y軸上の{}
        Point { x: 0, y } => println!("On the y axis at {}", y),
        // どちらの軸上でもない: ({}, {})
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            // Quit列挙子には分配すべきデータがない
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                // x方向に{}、y方向に{}だけ動く
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        // テキストメッセージ: {}
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                // 色を赤{}, 緑{}, 青{}に変更
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();

    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });

    foo(3, 4);


    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
        // 既存の値の変更を上書きできません
        println!("Can't overwrite an existing customized value");
        }
        _ => {
        setting_value = new_setting_value;
        }
    }
    // 設定は{:?}です
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
        // 何らかの数値: {}, {}, {}
        println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }


    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }
    
    println!("{:?}", s);

    
    let origin = Point { x: 0, y: 0};
    
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }


    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);


    let mut robot_name = Some(String::from("Bors"));

    match robot_name {
        // 別の名前
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);


    let num = Some(4);

    match num {
        // 5未満です: {}
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);


    let x = 4;
    let y = false;

    match x {
        // はい
        4 | 5 | 6 if y => println!("yes"),
        // いいえ
        _ => println!("no"),
    }


    enum Message2 {
        Hello { id: i32 },
    }
    
    let msg = Message2::Hello { id: 5 };
    
    match msg {
        Message2::Hello { id: id_variable @ 3..=7 } => {
            // 範囲内のidが見つかりました: {}
            println!("Found an id in range: {}", id_variable)
        },
        Message2::Hello { id: 10..=12 } => {
            // 別の範囲内のidが見つかりました
            println!("Found an id in another range")
        },
        Message2::Hello { id } => {
            // それ以外のidが見つかりました
            println!("Found some other id: {}", id)
        },
    }
}
fn print_coordinates(&(x, y): &(i32, i32)) {
    // 現在の位置: ({}, {})
    println!("Current location: ({}, {})", x, y);
}