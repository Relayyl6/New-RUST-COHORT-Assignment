fn main() {
    
    let student_record: [i32; 10] = [23, 24, 54, 76, 87, 54, 34, 89, 54, 70];
    
    let total_score = total_student_score(student_record);
    println!("The total number of scores is: {total_score}");
    
    let average_score = average_student_score(total_score, student_record);
    println!("The average number of scores is: {average_score}");
    
    let _highest_scoring_student = highest_scoring_student(student_record);
    let _passing_rate = passing_rate(student_record);
}

fn total_student_score(total_record: [i32; 10]) -> i32 {
    let total_record: i32 = total_record.iter().sum();
    // println!("{total_record}");
    total_record
}

fn average_student_score(average_record: i32, student_score: [i32; 10]) -> f64 {
    let average_student_record = average_record as f64/student_score.len() as f64;
    // println!("{average_student_record}");
    average_student_record
}

fn highest_scoring_student(student_score: [i32; 10]) {
    match student_score.iter().max() {
        Some(max_value) => println!("The maximum value is: {max_value}"),
        None => println!("The array is Empty")
    }
    // let highest_score = student_score.iter().max().unwrap()
}

fn passing_rate(student_passing_ratio: [i32; 10]) {

    let mut failing_score: i32 = 0;
    let mut passing_score: i32 = 0;
    
    for element in student_passing_ratio.iter() {
        if *element < 50 {
            println!("{element} failed");
            failing_score = failing_score + 1
        } else {
            passing_score = passing_score + 1
        }
    }
    println!("The number of passing scores are {}", passing_score);
    println!("The number of failing scores are {}", failing_score);
}