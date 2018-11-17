use std::io;

fn main() {

    //let mut a : i32 = 0;
    let mut result = 0;
    //let mut vec : Vec<String> ;
    {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                //println!("{}",trimmed);
                result = trimmed.parse::<i32>().unwrap();
            }
            Err(error) => println!("error: {}", error),
        }
    }

    {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                //println!("Input is {}", input);
                let trimmed = input.trim();
                let mut iter = trimmed.split_whitespace();
                let b: i32 = iter.next().unwrap().parse().unwrap();
                //println!("{}", b);
                result = result + b;
                let c: i32 = iter.next().unwrap().parse().unwrap();
                //println!("{}", c);
                result = result + c;
                //result += iter.next().unwrap().parse().unwrap();
            }
            Err(error) => println!("error: {}", error),
        }
    }

    {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                println!("{} {}", result,trimmed);
            }
            Err(error) => println!("error: {}", error),
        }
    }


}