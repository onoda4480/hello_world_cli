use std::env;

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let arg1 = env::args().nth(1).expect("nameがありません");
    let arg2 = env::args().nth(2).expect("ageがありません");
    let args = User {
        name: arg1,
        age: arg2.parse().expect("age must be a number"),
    };
    println!("「:?」を使った場合  {:?}", args);
    println!("「:#?」を使った場合 is {:#?}", args);
    println!("「1:?」 : {1:?}, 「0:?」 : {0:?}", args.name, args.age);
}
