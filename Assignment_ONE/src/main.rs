fn main() {

    
    // Printing my name
    name("Leonard");

    // Printing my Mat No
    mat_no("ENG2204523");

    // Printing my favorite Emoji
    favorite_emoji('🌹');

     // The difference of two entered numbers
    let num1 = 64;
    let num2 = 32;
    let value = difference(num1, num2);

    // The difference expressed as fraction/decimal
    formatted(value);
}

fn name(name: &str){  
    println!("My name is: {name}");
}

fn mat_no(num: &str) {
    println!("My Matriculation number is: {num}");
}

fn favorite_emoji(favorite: char) {
    println!("My favorite emoji is: {favorite}");
}

fn difference(num1: i32, num2: i32) -> i32 {
   
    let difference = num1 - num2;
    println!("The difference between the two numbers is: {difference}");
    difference as i32
}

fn formatted(result: i32) {
    //making the result of the dfference into a fraction/decimal
    let integer = result as f64;
    let formatted = format!("{:.2}", integer);

    println!("Formatted float: {formatted}");
}