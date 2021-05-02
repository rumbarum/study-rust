fn main() {
    let mut s1 = String::from("hello");

    //let len = cal_length(&s1);

    //println!("'{}'의 길이는 {}", s1, len)
    change(&s1);

}

//fn cal_length(s: &String) -> usize {
//    s.len()
//}

fn change(some_string: &String){
    //some_string.push_str(", world!");
    println!("{}",some_string)
}
