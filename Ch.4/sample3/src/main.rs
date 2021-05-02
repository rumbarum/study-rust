fn main() {
    let s = String::from("hello");

    take_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("Hello, world!");
}

fn take_ownership(string:String) {
    println!("{}", string)
}

fn makes_copy(integer:i32) {
    println!("{}", integer)
}
