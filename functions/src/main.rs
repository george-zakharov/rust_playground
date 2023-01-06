fn main() {
    println!("Hello, world!");

    another_function();
    and_another_one();
}

fn another_function() {
    println!("Another function!");
}

fn and_another_one() {
    let mut s = String::from("Hello");
    s.push_str(", pretty world!");
    println!("{}", s);
}