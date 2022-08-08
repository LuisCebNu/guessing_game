use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main(){
    println!("Guess the number!");//println! is a Macro that let's you print with a new line
    println!("Can you guess my number from 1 in 100?");

    let secret_number = rand::thread_rng().gen_range(1,101); //Generates a random number from 1 to 100, it doesnt include the last number

    // println!("The secret number is {}", secret_number); USE FOR TESTING

    loop{
        println!("Please input a guess: ");
        let mut guess = String::new(); //It inserts a new empty string into a variable
        //mut means it can be change, without it, the variable is almost a constant

        io::stdin().read_line(&mut guess) //lets you insert input into a variable. 
            .expect("Failed to read line");//This catch errors and print the messages you want to print

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>{
                println!("Please input a valid number!");
                continue
            } 
            
        };
        println!("You guess: {}", guess); 

        /*
            Uses the imported Ordering library to compare the result of the 
            comparasing of the secret number and the use number
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            }
        }
    }
}