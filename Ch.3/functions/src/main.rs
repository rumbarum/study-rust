
fn main() {

    println!("안녕하세요!");

    another_function(5, "강아지 강아지 고양이".to_string());
}

fn another_function(x: i32, y: &str ) {

    println!("x's val: {}", x);
    println!("y's val: {}", y);

}
