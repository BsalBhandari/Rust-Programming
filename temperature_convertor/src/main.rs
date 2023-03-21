use std::io;
fn main() {

    println!("Do you want to conver to Celsius or Fahrenheit? Input C or F");

    let mut convert_type = String::new();

    io::stdin().read_line(&mut convert_type)
        .expect("Failed to read the conversion type");

    let t = convert_type.trim();
    println!("You want to convert to {}", t);
    
    println!("What temeperature would you like to convert?");

    let mut temp = String::new();

    io::stdin().read_line(&mut temp)
        .expect("Invalid Temperature");

    let temp : i32 = match temp.trim().parse(){
        Ok(temp) => temp,
        Err(_e) => panic!("That was now a valid input"),
    };

    match t{
        "C" => println!("{}",fah_into_celsius(temp)),
        "F" => println!("{}", celsius_into_fah(temp)),
        _ => println!("Invalid conversion type {}", t),
    }

}

fn fah_into_celsius(f : i32)-> i32{
    (f-32) * 5/9
}

fn celsius_into_fah(c : i32) -> i32{
    (c * (9/5)) + 32
}