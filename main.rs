#![allow(unused)]

use std::io::{*, self};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;
fn main() -> io::Result<()>
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

        /*
        if(guess == rand_num) { 
            println!("You guessed correctly");
            break;
        }
        else {
            print!("Incorrect! ");
            if guess > rand_num {println!("Too high")} else {println!("Too low");}
            println!("Try again");
            io::stdout().flush();

        }*/

        match guess.cmp(&rand_num) {
            Ordering::Less => {
                println!("Incorrect! Too low. Try again.");
                io::stdout().flush();
            },
            Ordering::Greater => {
                println!("Incorrect! Too high. Try again.");
                io::stdout().flush();
            },
            Ordering::Equal => { println!("Correct!"); break; }
        }
    }

    let mut arr: [i32; 5] = [0; 5];
    {
        let mut x = 0;

        loop {
            let mut inputter: String = String::new();

            io::stdin().read_line(&mut inputter).expect("Bad input");
            arr[x] = inputter.trim().parse::<i32>().unwrap();
            inputter.clear();

            x+=1;

            if x > 4 { break; }
        }
    }

    let mut arrIterator = 0;

    while arrIterator < arr.len() {
        let arrSize = arr.len() - 1;

        if arrIterator < arrSize { print!("{}, ", arr[arrIterator])}
        else if arrIterator == arrSize { print!("{}", arr[arrIterator]); io::stdout().flush();}

        arrIterator+=1;
    }

    Ok(())
}
