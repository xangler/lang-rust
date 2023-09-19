use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn greet(x:i32) -> i32{
    return x + 4;
}

#[derive(Clone)]
struct Student {
    odd: bool,
    name: String,
    age: i32
}

trait Dtrait {
    fn print_stu(&self);
}

impl Dtrait for Student {
    fn print_stu(&self){
        match self{
            Student{odd:true, name, age} => println!("odd {} {}", name, age),
            Student{odd:false, name, age} => println!("even {} {}", name, age)
        }
        // if let Student{odd:true, name, age} = obj{
        //     println!("odd {} {}", name, age);
        // } else if let Student{odd:false, name, age} = obj{
        //     println!("even {} {}", name, age);
        // }
    }
}

impl Dtrait for i32 {
    fn print_stu(&self){
        println!("i32: {}", self);
    }
}

fn print<T: std::fmt::Display>(value: T) {
    println!("value = {}", value);
}

fn compare<T>(left: T, right: T)
where
    T: std::fmt::Debug + PartialEq,
{
    println!("{:?} {} {:?}", left, if left == right { "==" } else { "!=" }, right);
}

fn main() {
    println!("hello world");
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8].iter().map(|x| x + 3).fold(0, |x, y| x + y);
    println!("{}", x);
    let y = 2;
    println!("{}", greet(y));
    println!("{}", std::cmp::min(4,28));

    let z1 = Student{odd:true, name:"John".to_string(), age: 28};
    let z2 = Student{odd:false, name:"Jim".to_string(), age: 15};
    let mut z3 = z1.clone();
    z3.age = 10;

    y.print_stu();
    z1.print_stu();
    // z1.print_stu(); // print_stpu 接口得带上&
    z2.print_stu();
    z3.print_stu();

    print::<i32>(y);
    compare("demo", "demo");
    compare("stpu", "dcu");
    
    let stdout = stdout();
    let message = String::from("Hello World!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
} 
