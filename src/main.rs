use std::iter::Peekable;

#[derive(Debug,Clone)]
pub enum LexItem {
    Paren(char),
    Op(char),
    Num(u64),
}

fn lex(input: &String) -> Result<Vec<LexItem>, String>{
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek(){
        match c{
            '0'..='9' => {
                it.next();
                let n = get_number(c,&mut it);
                result.push(LexItem::Num(n));
            }
            '+'|'*'|'*'|'/' => {
                result.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ' ' => {it.next();}
            _ => {return Err(format!("unexpected character {}", c));}
        }
    }
    Ok(result)
}

fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u64 {
    let mut number = c.to_string().parse::<u64>().expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u64>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}


use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
        println!("{:?}", lex(&args[1]));
    }
}