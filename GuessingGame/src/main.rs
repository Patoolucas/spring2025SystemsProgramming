fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        0
    }
    else if guess > secret{
        1
    } else{
        -1
    }
}


fn main() {
    //hard code the number
    let secret: i32 = 12;
    //store the supossed guesses 
    let guesses = [32,42,30];
    //track the guessses
    let mut g_Count = 0;
    //simulate the guessing
    let mut i = 0;
    loop{
        //controler
        if i >= guesses.len(){
            println!("No more guesses");
            break;
        }

        //make a guess
        let guess = guesses[i];
        g_Count +=1;
        //check the guess with the function
        let result = check_guess(guess,secret);

        //determine the guess if it is low or high
        if result == 0 {
            println!("You are correct! {} is the secret number.", guess);
            break;
        } else if result == 1 {
            println!("Your guess of {} is too high.", guess);
        } else {
            println!("Your guess of {} is too low.", guess);
        }

        //move to the next number
        i += 1
    }

    println!("It took you {} guesses.", g_Count);
}

