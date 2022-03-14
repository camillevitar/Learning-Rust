//If a type yuou want to use isn't in the Prelude, you have to bring that type into scope of every program
use std::io;
//The Rng trait defines methods that random number generatos implement. This must be in scope for us to use it.
use rand::Rng;
use std::cmp::Ordering;

//The fn() syntax declares a new function
fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
    	println!("Please input your guess.");

//Variables are declared with let and are  inmutable as default, to make it mutable, we must add mut before the variable name
    
	let mut guess = String::new();

//we'll call the stdin function from the io module, which will allow us to handle the user input
//& refers to a REFERENCE and as variables, you have to add mut to make it mutable
    	io::stdin()
	    .read_line(&mut guess)
	    .expect("Failed to read line");

    	let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};  

    	println!("You guessed {}", guess);
    
    	match guess.cmp(&secret_number) {
	    Ordering::Less => println!("Too small"),
	    Ordering::Greater => println!("Too big"),
	    Ordering::Equal => { 
		println!("You win");
		break;
	    }
    	}
    }
}
