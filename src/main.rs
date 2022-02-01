use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first: String = args.nth(1).unwrap();

    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();
    let firstn = first.parse::<f32>().unwrap();
    let secondn = second.parse::<f32>().unwrap();
    let result = operate(operator, firstn, secondn);
    println!("{}", output(firstn, operator, secondn, result));
}

fn operate(operator: char, firstn: f32, secondn: f32) -> f32 {
    match operator {
        '+' => firstn + secondn,
        '-' => firstn - secondn,
        '/' => firstn / secondn,
        '*' => firstn * secondn,
        _ => panic!("invalid operator"),
    }
}

fn output(firstn: f32, operator: char, secondn: f32, result: f32) -> String {
    format!("{} {} {} = {}", firstn, operator, secondn, result)
}
