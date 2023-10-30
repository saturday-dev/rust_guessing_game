// fn main() {
//     println!("Hello, world!");
// }

extern crate rand;

use std::io;
use rand::Rng;
// use rand::{thread_rng, Rng};
// use rand::distributions::Uniform;
use std::cmp::Ordering;


fn main(){
    print!("Guess the number!");

    let mut  secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number : {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to red line");
    
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)  => continue,
        };
            // .expect("Please type a number!");
    
        println!("Youe guessed: {}", guess);
    
        // 숫자를 비교한다.
        match guess.cmp(&secret_number){
            Ordering::Less  => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }


}


fn main3(){
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess)

}


// fn main2(){
//     let foo = 5; // 기본적으로 immutable (변경안됨)
//     let mut bar = 5; // mut를 붙이면 mutable(변경됨)
//     foo = 35;
//     bar = 23;
// }