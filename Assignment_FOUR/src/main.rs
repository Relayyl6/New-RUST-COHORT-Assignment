use std::io;

fn main() {
    println!("Pick an operation: \n 1. addition \n 2. Division \n 3. Subtraction \n 4. Multiplication \n 5. CGPA calculator");

    let mut option_type = String::new();
    io::stdin()
        .read_line(&mut option_type)
        .expect("Enter a valid option Type");
        
    let option_type = option_type.trim();

    let operation: [&str; 5] = [
        "Addition",
        "Division",
        "Subtraction",
        "Multiplication",
        "CGPA calculator"
    ];

    // for num in operation {
    //     let new _operation = operation[] - 1;
    // }
    
    let option_type: f64 = option_type.trim()
                                      .parse()
                                      .expect("Please enter a valid number");
    
    // let index = option_type + 1.0;
    let index: usize = (option_type + 1.0).round() as usize;
    println!("Selected operation = {}", operation[index]);
    
    let option_type = option_type.to_string();
    
    match option_type.parse::<u8>() {
        Ok(1..=4) => {
            //To read the input from the command line
        println!("Enter the first number: ");
        let mut first_input = String::new();
        io::stdin()
            .read_line(&mut first_input)
            .expect("Failed to read line");

        
        println!("Print the second number");
        let mut second_input = String::new();
        io::stdin()
            .read_line(&mut second_input)
            .expect("Failed to read line");
            
            
        let first_input: f64 = first_input.trim()
                                        .parse()
                                        .expect("Please enter a valid number");
        let second_input: f64 = second_input.trim()
                                        .parse()
                                        .expect("Please enter a valid number");

        if option_type == "1" {
            let result = first_input + &second_input;
            println!("The sum is {result}");
        } else if option_type == "2" {
            if first_input == 0.0 {
                println!("Error: Division by zero is not allowed");
            } else if second_input == 0.0 {
                println!("Error: Division by zero is not allowed");
            } else {
                let result = first_input / second_input;
                println!("The sum is {result}");
            }
        } else if option_type == "3" {
            let result = first_input - second_input;
            println!("The sum is {result}");
        } else if option_type == "4" {
            let result = first_input * second_input;
            println!("The sum is {result}");
        }},
        Ok(5) => {
            cgpa_calc()
        }
        Ok(_) => println!("Number out of range (must be 1-4)"),
        Err(_) => println!("Invalid operation. Please use +,-,/,*"),
    }

    // if option_type == (1..=4) {

        
    // } else if option_type == "5" {
    //     cgpa_calc()
    // } else {
    //     panic!("Invalid operation. Please use +,-,/,*");
    // }

}

fn cgpa_calc() {

    println!("How many courses do you have?");
    
    let mut num_of_courses = String::new();
    io::stdin()
        .read_line(&mut num_of_courses)
        .expect("Enter a valid student number");
    
    let num_of_courses: i32 = num_of_courses.trim()
                                            .parse()
                                            .expect("Please enter a valid number");
    println!("This is the number of courses: {}", num_of_courses);

    // let count: &i32 = &num_of_courses;

    let mut count = 0;
    let mut total_credit_load = 0;
    let mut total_grade_points  = 0;
    let remaining = 22;

    while count < num_of_courses {
        println!("Credit load left = {}", remaining-total_credit_load);

        // Course code 
        println!("Enter Course Code: ");
        let mut course_code = String::new();
        io::stdin() 
            .read_line(& mut course_code)
            .expect("Failed to read input");
        let _course_code: i32 = course_code.trim()
                                          .parse()
                                          .expect("Make sure to enter correct code");

        // Course credit
        println!("Enter Course Credit: ");
        let mut course_credit = String::new();
        io::stdin() 
            .read_line(& mut course_credit)
            .expect("Failed to read input");
        let course_credit: i32 = course_credit.trim()
                                              .parse()
                                              .expect("Make enter a valid number");

        // Grade input
        println!("Enter Grade (A-F)");
        let mut grade_score = String::new();
        io::stdin() 
            .read_line(& mut grade_score)
            .expect("Failed to read input");
        // .chars() turns a string slice into an iterator over its characters, .next() gets the first character from that iterator
        let grade_score = grade_score.trim()
                                     .chars()
                                     .next()
                                     .expect("No grade entered");

        let grade_point =  grade_calculation(grade_score);
        let grade_point: i32 = grade_point.trim()
                                     .parse()
                                     .expect("Invalid grade point");

        total_grade_points += grade_point * course_credit;
        total_credit_load += course_credit;
        count += 1;
                             
        if total_credit_load > remaining {
            panic!("Exceeded maximum credit load");
        }


        let cgpa = total_grade_points as f64 / total_credit_load as f64;
        let formatted_cgpa = format!("{:.2}", cgpa);
        println!("The overall cgpa of the {} number of courses is {}", num_of_courses, formatted_cgpa)
    }



    // if count != 12 {
    //     // let current_course_num = &num;
    //     let remaining = 22;
    //         'counting_cgpa: loop {
    //             println!("Credit load = {} left", remaining);
    //             if count < num_of_courses{
    //                 let course_code: String = String::new();
    //                     io::stdin()
    //                         .read_line(&mut course_code)
    //                         .expect("Enter a valid student number");


    //                 let course_credit: String = String::new();
    //                     io::stdin()
    //                         .read_line(&mut course_code)
    //                         .expect("Enter a valid student number");

                        
    //                 let grade_average: String = String::new();
    //                     io::stdin()
    //                         .read_line(&mut course_code)
    //                         .expect("Enter a valid student number");
                       
                       
    //             let grade: String = grade_calculation(grade_average)
    //             count = count + 1;

    //             if count == 11 {
    //                 break 'counting_cgpa;
    //             }
    //             }   
    //             current_credit = course_credit.sum();
    //             remainder = remaining - current_credit;
    //         }          
    // } else {
    //     panic!("Beyond course threshold, please check again")
    // }
    
    // let total_credit_load = remaining;
    // let total_course_credits = 
    
    
    // println!("You have {} number of courses", num_of_courses);

    // let cgpa = calculated_cgpa(total_course_credits, total_credit_load);
    // println!("Overall CGPA for the {} number of courses is {}", num_of_courses, cgpa)
}



fn grade_calculation(param: char) -> String {
    match param.to_ascii_uppercase() {
        'A' => String::from("5"),
        'B' => String::from("4"),
        'C' => String::from("3"),
        'D' => String::from("2"),
        'E' => String::from("1"),
        'F' => String::from("0"),
        _ => panic!("Invalid day")
    }
}

