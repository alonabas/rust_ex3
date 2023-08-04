use std::io;

fn fahrenheit_celsius_convert() {
    println!("### Convert temperatures between Fahrenheit and Celsius. ###");

    loop {
        let mut unit = String::new();
        println!("Please input your unit. To stop write 'exit'");
            io::stdin()
                .read_line(&mut unit) 
                .expect("Failed to your input");
        let unit = unit.trim();
        if unit.eq("exit") {
            break;
        }
        if unit != "f" && unit != "c" {
            println!("Provided unit can be 'c' for celcius or 'f' for fahrenheit");
            continue;
        }

        let mut temp = String::new();
        println!("Please input your temperature.");
            io::stdin()
                .read_line(&mut temp) 
                .expect("Failed to your input");
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Cant transform your temperature value to number.");
                continue;
            }
        };

        println!("You entered {temp}{unit}");
        if unit == "f" {
            let result: f32 = (temp - 32.0) * 0.5556;
            println!("{:.2}{unit} = {:.2}c", temp, result);
        }
        else if unit == "c" {
            let result: f32 = (temp * 1.8) + 32.0;
            println!("{:.2}{unit} = {:.2}f", temp, result);
        }
    }
}

fn fibonachi_rec(n: i32, v1: i64, v2: i64) -> i64 {
    if n == -1 {
        return v1;
    } else if n == 0 {
        return v2;
    }
    return fibonachi_rec(n - 1,v2, v1 + v2);
}

fn fibonachi() {
    println!("### Generate the nth Fibonacci number. ###");

    loop {
        let mut n = String::new();
        println!("Please input your n. To stop write 'exit'");
            io::stdin()
                .read_line(&mut n) 
                .expect("Failed to your input");
        
        let n = n.trim();
        if n.eq("exit") {
            break;
        }
        let n: i32 = match n.parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Cant transform your input to number");
                continue;
            }
        };
        println!("Your n is {n}.");
        if n <= 0 {
            println!("Provided n should be positive");
            continue;
        }
        else if n > 93 {
            println!("Provided n should be less than 93 to avoid overflow.");
            continue;
        }

        let number = fibonachi_rec(n - 2, 0 , 1);
        println!("{n}th fibonachi is {number}");
    }
}

fn christmas_carol() {
    println!("### Print the lyrics to the Christmas carol “The Twelve Days of Christmas” ###");
    let days: [String; 9] = [
        String::from("first"), 
        String::from("second"), 
        String::from("third"), 
        String::from("fourth"), 
        String::from("fifth"), 
        String::from("sixth"), 
        String::from("seventh"), 
        String::from("eighth"), 
        String::from("nineth")
    ];
    let phrases: [String; 9] = [ 
        String::from("A partridge in a pear tree"),
        String::from("Two turtle doves"),
        String::from("Three French hens"),
        String::from("Four calling birds"),
        String::from("Five goldenen rings"),
        String::from("Six geese a-laying"),
        String::from("Seven swans a-swimming"),
        String::from("Eight maids a-milking"),
        String::from("Nine ladies dancing")
    ];
    let second_line: String = String::from("My true love brought to me");

    let mut index = 1;
    for day in days {
        println!("On the {day} day of Christmas");
        println!("{second_line}");
        for number in (0..index).rev() {
            if number == 0 && index > 1 {
                println!("And {}", phrases[number].to_lowercase());
            } else {
                println!("{}", phrases[number]);
            }
        }
        index += 1;
        println!("");
    }
}

fn main() {
    fahrenheit_celsius_convert();
    fibonachi();
    christmas_carol();
}
