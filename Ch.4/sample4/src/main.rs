fn main() {
    let s1 = g_ownership();

    let s2 = String::from("hello");

    let s3 = t_and_g_back(s2);
    
    println!(
        "{}, {}", s1,  s3
            )
}

fn g_ownership() -> String {
    let some_str = String::from("hello");
    some_str
}

fn t_and_g_back(a_str: String) -> String {
    a_str
}
