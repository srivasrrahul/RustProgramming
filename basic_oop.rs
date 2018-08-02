use std::env;
use std::vec;
#[derive(Debug)]
struct Rational {
    numerator : i32,
    denominator : i32
}

impl Rational {
    fn reduce(num : i32,den : i32) -> (i32,i32) {
        let gcd_val = Rational::gcd(num.abs(),den.abs());
        return (num/gcd_val,den/gcd_val);
    }

    fn new(num : i32, den : i32) -> Rational {
        let (x,y) = Rational::reduce(num,den);
        return Rational {numerator : x,denominator : y};
    }

    fn gcd(x : i32,y : i32) -> i32 {
        fn gcd_rec(a : i32, b : i32) -> i32 {
            let d = b % a;
            if d == 0 {
                return a;
            }else {
                return gcd_rec(d,a);
            }
        }

        if x < y {
            return gcd_rec(x,y);
        }else {
            return gcd_rec(y,x);
        }
    }
//
//    fn lcm(x : i32,y : i32) -> i32 {
//        let gcd_val = gcd(x,y);
//    }

    pub fn plus(&self,b : Rational) -> Rational {
        let new_den = self.denominator * b.denominator;
        let new_num1 = new_den/self.numerator;
        let new_num2 = new_den/b.numerator;
        return Rational::new(new_num1+new_num2,new_den);
    }

    pub fn minus(&self,b : Rational) -> Rational {
        let new_den = self.denominator * b.denominator;
        let new_num1 = new_den/self.numerator;
        let new_num2 = new_den/b.numerator;
        return Rational::new(new_num1-new_num2,new_den);
    }


}

#[derive(Debug)]
struct Date {
    day : i32,
    month : i32,
    year : i32
}

impl Date {

    fn new_date(mm : i32,dd : i32,yy : i32) -> Date {
        return Date {day : dd,month : mm,year : yy};
    }
    //MM/DD/YYYY
    fn parse_date(date : &str) -> Option<Date> {
        let vec : Vec<&str>= date.split("/").collect();
        println!("{}",vec.len());
        if vec.len() != 3 {
            return None
        }else {
            println!("{}",vec[0]);
            let mm = vec[0].parse::<i32>();
            //println!("{}",mm.unwrap());
            match mm {
                Ok(m) => {
                    //println!("Hello");
                    match vec[1].parse::<i32>() {
                        Ok(d) => {

                            match vec[2].parse::<i32>() {
                                Ok(y) => return Some(Date::new_date(m,d,y)),
                                Err(e) => return None

                            }

                        },
                        Err(e) => return None
                    }

                },
                Err(e) => {
                    //println!("None1");
                    return None;
                }
            }
            return None;

        }
    }
}

fn main() {
    let r1 = Rational::new(1,2);
    let r2 = Rational::new(2,3);
    let r3 = r2.plus(r1);
    println!("{:?}",r3);

    let args: Vec<String> = env::args().collect();
    println!("{}",args[1]);
    println!("{:?}",Date::parse_date(&args[1]));

}