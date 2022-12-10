fn main() {
    let gifts = [
        "A partridge in a pear tree",
        "turtle dove",
        "French hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    for day in 1..13 {
        println!(
            "\n On the {} day of Christmas my true love sent to me",
            num_to_word_and_position(day).1
        );
        for gift in 1..13 {
            if gift > day {
                break;
            };
            match gift {
                1 => println!("{}", gifts[(gift - 1) as usize]),
                _ => println!(
                    "And {} {}",
                    num_to_word_and_position(gift).0,
                    gifts[(gift-1) as usize]
                ),
            }
        }
    }
}

fn num_to_word_and_position(num: u8) -> (String, String) {
    match num {
        1 => ("one".to_string(), "first".to_string()),
        2 => ("two".to_string(), "second".to_string()),
        3 => ("three".to_string(), "third".to_string()),
        4 => ("four".to_string(), "fourth".to_string()),
        5 => ("five".to_string(), "fifth".to_string()),
        6 => ("six".to_string(), "sixth".to_string()),
        7 => ("seven".to_string(), "seventh".to_string()),
        8 => ("eight".to_string(), "eighth".to_string()),
        9 => ("nine".to_string(), "nineth".to_string()),
        10 => ("ten".to_string(), "tenth".to_string()),
        11 => ("eleven".to_string(), "eleventh".to_string()),
        12 => ("twelve".to_string(), "twelveth".to_string()),
        _ => ("".to_string(), "".to_string()),
    }
}
