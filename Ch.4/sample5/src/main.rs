fn main() {
    let s1 = String::from("hello");

    let (s2, len) = cal_length(s1);
    

    println!("'{}'의 길이는 {}",s2, len);
}

fn cal_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
