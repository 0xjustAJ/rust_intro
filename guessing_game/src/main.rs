extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");
    loop{
    println!("please input your guess.");
    println!("--------------------------------------");
    //////// we make guess variable an empty string by doing the following:
    let mut guess = String::new();
    //let guess: u32= guess.trim().parse().expect("please type a number!");

    ////////by using the input and output library, this line of code makes it easy for us to recieve users input
    io::stdin().read_line(&mut guess).expect("failed to read line");
    let guess: u32= match guess.trim().parse(){
        Ok(num)=> num,
        Err(_)=> continue,
    };
     
     println!("you guessed: {}", guess);

     ////the first part of the game is done 
     
     // the next part is to generate a random number/ a secret number//////
     let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
     //println!("the secret number is {}", secret_number);
     
     
     /////next we compare the guess to a secret number
     
     match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {println!("You win");
                              break;
                              }
     }
     //println!("the secret number is {}", secret_number);
    println!(".......................................")
     
     
}
}



