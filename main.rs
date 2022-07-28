use std::io;

fn main()
{
    println!("Enter a number: ");

    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Bad input");

    let x: i32 = num.trim().parse::<i32>().unwrap();
    let mut num: i32 = 1;
    loop {
        if(num % 2 == 0) { println!("{}", num);}
        num += 1;

        if num == x {break;}
    }
}
