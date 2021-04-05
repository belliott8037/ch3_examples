use std::io;
fn main() {
    /*
        Three examples in chapter 3
        1) Convert temperatures between Fahrenheit and Celsius
        2) Generate the nth Fibonacci number
        3) Print the lyrics to the Christmas carol "The Twelve Days of Christmas" taking advantage of the repitition in the song
    */
    loop {
        println!("Which example? 1, 2, or 3? 'E' to exit");
        let mut example = String::new();

        io::stdin()
        .read_line(&mut example)
        .expect("Failed to read line");

        let example = example.trim();
        if example == "E" || example == "e" {
            break;
        }
        let example: u32 = match example.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        if example == 1 {
            first_example();
        } else if example == 2 {
            second_example();
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
        println!("{} is {} in Farenheit ",temp, new_temp);
    }
    
}
fn get_temp() -> i32 {
    loop {
        println!("What temperature?");
        let mut temp = String::new();
        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

        match temp.trim().parse(){
            Ok(num) => return num,
            Err(_) => continue,
        }
        
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
fn second_example() {
    println!("What Fibonacci entry would you like? ");
    
    let mut n = String::new();

    io::stdin()
    .read_line(&mut n)
    .expect("Failed to read line");

    let n: i32 = match n.trim().parse(){
        Ok(num) => num,
        Err(_) => return,
    };
    if n <= -2 {
        let n = n * -1;
        let value = find_number(n-1) + find_number(n-2);
        if sign(n){
            let value = value * -1;
            println!("Value is: {}",value);
        }else{
            println!("Value is: {}",value);
        }
    }
    else if n >= -1 && n <= 2 {
        if n < 0 {
            let n = n*-1;
            let value = find_number(n);
            println!("Value is: {}",value);
        }else{
            let value = find_number(n);
            println!("Value is: {}",value);
        }
    }else{
        let value = find_number(n-1) + find_number(n-2);
        println!("Value is: {}",value);
    }
    
}
// passed in n and calculates until we get values
fn find_number(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }else if n == 1 || n == 2 {
        return 1;
    }else{
        return find_number(n-1) + find_number(n-2);
    }
}
fn sign(n: i32) -> bool{
    if n % 2 == 0 {
        return true;
    }else{
        return false;
    }
}