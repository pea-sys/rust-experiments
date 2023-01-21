use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            // 予想の値は1から100の範囲でなければなりませんが、{}でした
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");          // 数を当ててごらん

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The secret number is: {}", secret_number);    //秘密の数字は次の通り: {}

    loop {
        println!("Please input your guess.");   // ほら、予想を入力してね

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");     // 行の読み込みに失敗しました

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess_c = Guess::new(guess);

        println!("You guessed: {}", guess_c.value);     // 次のように予想しました: {}
        
        match guess_c.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),       //小さすぎ！
            Ordering::Greater => println!("Too big!"),      //大きすぎ！
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
