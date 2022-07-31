#![allow(unused)]

use std::io::{*, self};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

/*
println!("Enter a number: ");

    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Bad input");

    let x: i32 = num.trim().parse::<i32>().unwrap();
    let mut num: i32 = 1;
    loop {
        if(num % 2 == 0) { println!("{}", num);}
        num += 1;

        if num == x {break;}
    }*/

fn main()
{
    let rand_num = rand::thread_rng().gen_range(1..11);
    let mut guesses: i32 = 0;
    let mut guess: String = String::new();

    loop {
        guesses+=1;
        assert!(guesses < 6);

        println!("Enter guess between 1 and 10");
        guess.clear();
        io::stdin().read_line(&mut guess).expect("Bad input");

        let mut guess: i32 = guess.trim_end().parse::<i32>().unwrap();

        if(guess == rand_num) { 
            println!("You guessed correctly");
            break;
        }
        else {
            print!("Incorrect! Try again.");
            io::stdout().flush();
        }
    }
}
