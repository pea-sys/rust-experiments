extern crate nom;
use nom::{
    branch::{alt, permutation},
    bytes::complete::{is_a, is_not, tag, tag_no_case, take, take_till, take_until, take_while},
    character::complete::{alpha1, char, digit1, none_of, one_of},
    character::is_alphabetic,
    error::ErrorKind,
    error::ParseError,
    multi::{count, many0, many_m_n,many_till,separated_list0,fold_many0,fold_many_m_n,length_count},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    Err::Error,
    IResult,
    combinator::map,
    number::complete::u8,
};

fn char_parser(i: &str) -> IResult<&str, char> {
    char('a')(i)
}
fn is_a_parser(s: &str) -> IResult<&str, &str> {
    is_a("1234567890ABCDEF")(s)
}
fn is_not_parser(s: &str) -> IResult<&str, &str> {
    is_not(" \t\r\n")(s)
}
fn one_of_parser(s: &str) -> IResult<&str, char> {
    one_of("abcde")(s)
}
fn none_of_parser(s: &str) -> IResult<&str, char> {
    none_of("abcde")(s)
}
fn tag_parser(s: &str) -> IResult<&str, &str> {
    tag("Hello")(s)
}
fn tag_no_case_parser(s: &str) -> IResult<&str, &str> {
    tag_no_case("hello")(s)
}

fn take_parser(s: &str) -> IResult<&str, &str> {
    take(6usize)(s)
}

fn take_while_parser(s: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(is_alphabetic)(s)
}

fn take_till_parser(s: &str) -> IResult<&str, &str> {
    take_till(|c| c == ':')(s)
}

fn take_until_parser(s: &str) -> IResult<&str, &str> {
    take_until("eof")(s)
}
fn alt_parser(input: &str) -> IResult<&str, &str> {
    alt((alpha1, digit1))(input)
}

fn permutation_parser(input: &str) -> IResult<&str, (&str, &str)> {
    permutation((alpha1, digit1))(input)
}

fn delimited_parser(input: &str) -> IResult<&str, &str> {
    delimited(char('('), is_not(")"), char(')'))(input)
}

fn preceded_parser(input: &str) -> IResult<&str, &str> {
    preceded(tag("abc"), tag("efg"))(input)
}

fn terminated_parser(input: &str) -> IResult<&str, &str> {
    terminated(tag("abc"), tag("efg"))(input)
}
fn pair_parser(input: &str) -> IResult<&str, (&str, &str)> {
    pair(tag("abc"), tag("efg"))(input)
}
fn separated_pair_parser(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(tag("abc"), tag("|"), tag("efg"))(input)
}

fn count_parser(input: &str) -> IResult<&str, Vec<&str>> {
    count(tag("abc"), 2)(input)
}
fn many0_parser(input: &str) -> IResult<&str, Vec<&str>> {
    many0(tag("abc"))(input)
}
fn many_m_n_parser(input: &str) -> IResult<&str, Vec<&str>> {
    many_m_n(0, 2, tag("abc"))(input)
}
fn many_till_parser(input: &str) -> IResult<&str, (Vec<&str>, &str)> {
    many_till(tag("abc"), tag("end"))(input)
  }

  
fn separated_list0_parser(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag("|"), tag("abc"))(input)
  }

  fn fold_many0_parser(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many0(
      tag("abc"),
      Vec::new,
      |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
      }
    )(input)
  }
  fn fold_many_m_n_parser(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many_m_n(
      0,
      2,
      tag("abc"),
      Vec::new,
      |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
      }
    )(input)
  }
  
fn length_count_parser(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    length_count(map(u8, |i| {
       println!("got number: {}", i);
       i
    }), tag("abc"))(input)
  }
fn main() {
    let mut input = "abc";
    match char_parser(&input) {
        Ok(result) => println!("char_parser: {} = {} {} ", &input, result.0, result.1),
        Err(e) => println!("err value = {}", e),
    }

    input = "BADBABEsomething";
    match is_a_parser(&input) {
        Ok(result) => println!("is_a_parser: {} = {} {} ", &input, result.0, result.1),
        Err(e) => println!("err value = {}", e),
    }

    input = "Hello, World!";
    match is_not_parser(&input) {
        Ok(result) => println!("is_not_parser: {} = {} {} ", &input, result.0, result.1),
        Err(e) => println!("err value = {}", e),
    }

    input = "aht";
    match one_of_parser(&input) {
        Ok(result) => println!("one_of_parser: {} = {} {} ", &input, result.0, result.1),
        Err(e) => println!("err value = {}", e),
    }

    input = "z";
    match none_of_parser(&input) {
        Ok(result) => println!("none_of_parser: {} = {} {} ", &input, result.0, result.1),
        Err(e) => println!("err value = {}", e),
    }

    input = "Hello, World!";
    match tag_parser(&input) {
        Ok(result) => println!("tag_parser: {} = {} {} ", &input, result.0, result.1),
        Err(e) => println!("err value = {}", e),
    }

    input = "Hello, World!";
    match tag_no_case_parser(&input) {
        Ok(result) => println!(
            "tag__no_case_parser: {} = {} {} ",
            &input, result.0, result.1
        ),
        Err(e) => println!("err value = {}", e),
    }

    input = "1234567";
    match take_parser(&input) {
        Ok(result) => println!("take_parser: {} = {} {} ", &input, result.0, result.1),
        Err(e) => println!("err value = {}", e),
    }

    let byte_input = b"latin123";
    match take_while_parser(byte_input) {
        Ok(result) => println!(
            "take_while_parser: {:?} = {:?} {:?} ",
            &byte_input, &result.0, &result.1
        ),
        Err(e) => println!("err value = {}", e),
    }

    input = "latin:123";
    match take_till_parser(&input) {
        Ok(result) => println!(
            "take_till_parser: {} = {} {} ",
            &input, &result.0, &result.1
        ),
        Err(e) => println!("err value = {}", e),
    }

    input = "hello, worldeof";
    match take_until_parser(&input) {
        Ok(result) => println!(
            "take_until_parser: {} = {} {} ",
            &input, &result.0, &result.1
        ),
        Err(e) => println!("err value = {}", e),
    }

    input = "abc";
    match alt_parser(&input) {
        Ok(result) => println!("alt_parser: {} = {} {} ", &input, &result.0, &result.1),
        Err(e) => println!("err value = {}", e),
    }

    input = "abc123";
    match permutation_parser(&input) {
        Ok(result) => println!(
            "permutation_parser: {} = {} {} ",
            &input, &result.1 .0, &result.1 .1
        ),
        Err(e) => println!("err value = {}", e),
    }

    input = "(c12)";
    match delimited_parser(&input) {
        Ok(result) => println!(
            "permutation_parser: {} = {} {} ",
            &input, &result.0, &result.1
        ),
        Err(e) => println!("err value = {}", e),
    }

    input = "abcefg";
    match preceded_parser(&input) {
        Ok(result) => println!("preceded_parser: {} = {} {} ", &input, &result.0, &result.1),
        Err(e) => println!("err value = {}", e),
    }

    input = "abcefg";
    match terminated_parser(&input) {
        Ok(result) => println!(
            "terminated_parser: {} = {} {} ",
            &input, &result.0, &result.1
        ),
        Err(e) => println!("err value = {}", e),
    }

    input = "abcefg";
    match pair_parser(&input) {
        Ok(result) => println!(
            "pair_parser: {} = {} {} {} ",
            &input, &result.0, &result.1 .0, &result.1 .1
        ),
        Err(e) => println!("err value = {}", e),
    }

    input = "abc|efg";
    match separated_pair_parser(&input) {
        Ok(result) => println!(
            "separated_pair_parser: {} = {} {} {} ",
            &input, &result.0, &result.1 .0, &result.1 .1
        ),
        Err(e) => println!("err value = {}", e),
    }

    input = "abcabc";
    match count_parser(&input) {
        Ok(result) => println!("count_parser: {} = {} {:?}", &input, &result.0, &result.1),
        Err(e) => println!("err value = {}", e),
    }

    input = "abcabc";
    match many0_parser(&input) {
        Ok(result) => println!("many0_parser: {} = {} {:?}", &input, &result.0, &result.1),
        Err(e) => println!("err value = {}", e),
    }
    input = "abcabc";
    match many_m_n_parser(&input) {
        Ok(result) => println!("many_m_n_parser: {} = {} {:?}", &input, &result.0, &result.1),
        Err(e) => println!("err value = {}", e),
    }
    input="abcabcend";
    match many_till_parser(&input){
        Ok(result) => println!("many_till_parser: {} = {} {:?}", &input, &result.0, &result.1),
        Err(e) => println!("err value = {}", e),
    }
    input = "abc|abc|abc";
    match separated_list0_parser(&input){
        Ok(result) => println!("separated_list0_parser: {} = {} {:?}", &input, &result.0, &result.1),
        Err(e) => println!("err value = {}", e),
    }
    input = "abcabc";
    match fold_many0_parser(&input){
        Ok(result) => println!("fold_many0_parser: {} = {} {:?}", &input, &result.0, &result.1),
        Err(e) => println!("err value = {}", e),
    }
    
    input = "abc123";
    match fold_many_m_n_parser(&input){
        Ok(result) => println!("fold_many_m_n_parser {} = {} {:?}", &input, &result.0, &result.1),
        Err(e) => println!("err value = {}", e),
    }
    
    let byte_input2= b"\x02abcabcabc";
    match length_count_parser(&byte_input2[..]){
        Ok(result) => println!("length_count_parser {:?} = {:?} {:?}", &byte_input2, &result.0, &result.1),
        Err(e) => println!("err value = {}", e),
    }
}
