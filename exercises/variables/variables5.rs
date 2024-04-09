// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

// rust的隐藏属性，同一个变量可以多次使用，但是后续的使用需要跟第一次是使用时一样，需要使用let关键字进行复赋值
fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
