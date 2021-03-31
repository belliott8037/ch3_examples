use std::io;
fn main() {
    /*
        Three examples in chapter 3
        1) Convert temperatures between Fahrenheit and Celsius
        2) Generate the nth Fibonacci number
        3) Print the lyrics to the Christmas carol "The Twelve Days of Christmas" taking advantage of the repitition in the song
    */
    loop {
        println!("Which example? 1, 2, or 3? ");

        let mut example = String::new();
        io::stdin()
        .read_line(&mut example)
        .expect("Failed to read line");

        let example: u32 = match example.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        if example == 1 {
            first_example();
        }
    }
}
fn first_example() {
    // Convert temperature given from Fahrenheit and Celsius
    let temp = get_temp();
    let degree = get_degree();
    if degree == true {
        println!("Converting {} Farenheit to Celsius ", temp);
        let new_temp = (temp - 32) * 5/9;
        println!("{} is {} in Celsius ",temp, new_temp);
    } else {
        println!("Converting {} Celsius to Farenheit ", temp);
        let new_temp = (temp * 9/5) + 32;
        println!("{} is {} in Celsius ",temp, new_temp);
    }
    
}
fn get_temp() -> u32 {
    loop {
        println!("What temperature?");
        let mut temp = String::new();
        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

        let temp: u32 = match temp.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        return temp;
    }
}
fn get_degree() -> bool {
    let farenheit;
    loop { 
        println!("What degree? 'f' or 'c'?");
        
        let mut degree = String::new();
        io::stdin()
        .read_line(&mut degree)
        .expect("Failed to read line");
        
        let degree = degree.trim();

        match degree {
            "f" | "F" => farenheit = true,
            "c" | "C" => farenheit = false,
            _ => {
                println!("you blow");
                continue   
            }
        }
        return farenheit;
    }
}