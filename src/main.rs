// my hello world program
fn main() {
    //Chapter 1
    println!("Hello, world!");

    //Chapter 2
    let mut x:i8 = -1;
    println!("x is {}",x);
    x = 20;
    println!("x is {}",x);

    let y = 10.23;
    println!("{}",y);

    // let a:f64 = 10.0 / 3.0;
    // println!("{}",a);

    //  let b:f32 = 10.0 / 3.0;
    let a = 10;
    let b: f64 = 3.0;
    let sol = a as f64 / b; // casting
    // println!("{:.4}",sol);
    print!("{sol}");
    let mut value = 0b1111_0101u8;
    println!("value is {}",value);
    println!("value is {:08b}",value);
    value = !value;
    println!("value is {}",value);
    println!("value is {:08b} \n",value);

    let true_val = true; 
    let false_var = false;
    let c = (true_val ^ false_var) || (true_val & false_var);
    println!("{c}\n");
    let dino = '\u{1F996}';
    println!("{dino}");
    
    //Challenge from Chapter 2
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    /* YOUR CODE GOES HERE */
    let average: f64= (a as f64 + b + c as f64)/3.0; 
    assert_eq! (average, 45.1); 
    println! ("Test passed!");

    //Chapter 3
    let mut names = ["abe","alex","mark"];
    let mut first_name = names[0];
    println!("{first_name}");

    names[0] = "Abe Smith";
    first_name = names[0];
    println!("{first_name}");

    let index: usize = names.len() - 1;
    let last_name = names[index];
    println!("{last_name}");

    let parking_lot = [[0,1,2],
                                    [3,4,5],
                                    [6,7,8]];

    println!("{}",parking_lot[0][2]);

    let stuff = (14,"bob",5.61);
    let first_item = stuff.0;
    // let sec_item = stuff.1;
    println!("{first_item}");

    //chapter 4
    say_a_number(123);

    let square_output = square(7);
    println!("{square_output}");

    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    assert_eq! (fahrenheit_temp, 73.4);
    println! ("Test passed!");

    //Chapter 5
    let mut num = 45;
    if num >= 45{
        println!("num is greater than 45")
    }
    num = 20;
    if num < 45{
        println!("num is less than 45")
    }

    if x + 1 >= 21{
        println!("num is greater than 21");
    }

    let x = 10;
    let y = 10;
    if y > x{
        println!("y is greater than x");
    }
    else if y == x{
        println!("y is eqaul to x");
    }

    // loop{
    //     println!("infinate")
    // }

    for name in names{
        println!("Nice to meet you {}",name)
    }
    
}

fn say_a_number(number:i32){
    println!("{number}");
}

fn square(i:i32) -> i32{
    return i * i;
}

fn celsius_to_fahrenheit(x:f32) -> f32{
    let mut answer = (1.8 * x) + 32 as f32;
    answer = (answer * 10.0).round() / 10.0;
    return answer; 
}