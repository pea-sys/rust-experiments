extern crate ansi_term;

use ansi_term::Colour;
use ansi_term::Style;


//ansi_termももうメンテナンスされていないので
//代替を探す必要があるかもしれません
fn main() {
    //色付き文字を出力する
    println!("This is {} in color, {} in color and {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));
    //ターミナルで太文字]
    println!("{} and this is not",
    Style::new().bold().paint("This is Bold"));
    //ターミナルで色付き太文字
    println!("{}, {} and {}",
    Colour::Yellow.paint("This is colored"),
    Style::new().bold().paint("this is bold"),
    Colour::Yellow.bold().paint("this is bold and colored"));

    
}