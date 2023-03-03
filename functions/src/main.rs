fn main() {
    println!("Hello, world!");
    let x = five();
    another_function(x);

    println!("This is just to break things up");
    let condition: bool = true;
    let number: i32 = if condition {
        5
    } else {6};

    println!("The value of number is: {number}");
}

fn another_function(x: i32) {
    println!("This is within another function: {x}");
}// end function another_function

fn five() -> i32 {
    5
}//end function five
