fn main() {
    let lyrics: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    // let mut song: String = String::new();

    // // let mut count: i32 = 0;

    // for (day, lyric) in lyrics.iter().enumerate() {
    //    song.push_str(lyric);
    //    song.push_str("\n");

    //    println!("On the {} day of Christmas, my true love gave to me:", christmas_days(day + 1));
    //    println!({song})
    // }

    for day in 0..12 { // This iterates over days: 0 to 11, which current_day converts to readable and understadnable days: 1 to 12
        let current_day = day + 1;
        let mut song = String::new(); // Initilise a new mutable empty string that can grow as song lyrics are being appended, and loop/start afresh each day/iteration

        //Building the lyric in reverse order // take i as the index of the lyrics
        for i in (0..=day).rev() { // iterate from the current day, down to the 0 day(first day) // rev( reverse) is used to reverse the order of the iteration // i.e. day = 2 ->  i = 2, 1, 0
            // such that, eg day = 2 (again), it becomes, for i in (0..=2).rev() is 2, 1, 0 instead of 0, 1, 2
            // This is done to ensure that the first line of the song is printed last, and the last line of the song is printed first
            if i == 0 && day != 0 { // Adds and "And" before the lyric of the first day
                song.push_str("And "); 
            }
            song.push_str(lyrics[i]);
            song.push_str("\n");
        }
        println!("On the {} day of Christmas, my true love gave to me:\n{}", christmas_days(current_day), song);
    }
}


fn christmas_days(param: usize) -> String {
    match param {
        1 => String::from("first"),
        2 => String::from("second"),
        3 => String::from("third"),
        4 => String::from("fourth"),
        5 => String::from("fifth"),
        6 => String::from("sixth"),
        7 => String::from("seventh"),
        8 => String::from("eighth"),
        9 => String::from("ninth"),
        10 => String::from("tenth"),
        11 => String::from("eleventh"),
        12 => String::from("twelfth"),
        _ => panic!("Invalid day")
    }
}