use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut leavegame = false;
    let insults = ["idiot", "moron", "fool", "jerkface", "imbecile"];
    
    let mut money: u32 = 10;

    loop {
        if leavegame {
            break;
        }

        let difficulty = choose_difficulty();
        let number_of_chances = 8 - difficulty;
        if number_of_chances > 1 {
            println!("You will have {} chances to guess the number.",  number_of_chances);
        }
        else {
            println!("You will have {} chance to guess the number.",  number_of_chances);
        }

        let bet = get_bet(money, &insults);

        let secret_number = get_secret_number();
        println!("Guess the number from 1 to 100!");

        for x in 0..number_of_chances {
            
            let mut guess = String::new();
            println!("Please input your guess or 'quit' to quit.");
            println!("If you quit you lose your bet.");

            io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    match guess.trim(){
                        "quit" => {
                            println!("Quitting the game and losing your bet, you loser!");
                            lost_bet(&mut money, bet);
                            leavegame = true;
                            break;
                        }
                        _ => {
                            println!("Please input a number from 1 to 100, not '{}'.", guess.trim());
                            println!(" Can't you read instructions?");
                            continue;
                        }
                    }
                }
            };

            if process_guess(guess, secret_number, &insults) {
                won_bet(&mut money, difficulty, bet);
                break;
            }

            process_guesses_left(x, number_of_chances, &mut money, bet);
            
        }
        println!("The secret number was {}.", secret_number);
        if money == 0 {
            println!("You have nothing left, you degenerate gambler! I'm kicking you out!");
            break;
        }

    }

    fn get_secret_number() -> u32 {
        let secret_number = rand::thread_rng().gen_range(1, 101);
        //println!("The secret number is: {}", secret_number);
        secret_number  
    }

    fn get_bet(money: u32, insults: &[&str]) -> u32 {
        println!("You can place a bet up to {}. Please enter your bet.", money);
        let _bet_made: u32;
        loop {
            let mut bet = String::new();
            io::stdin().read_line(&mut bet)
                    .expect("Failed to read line");
            let _bet: u32 = match bet.trim().parse() {
                Ok(num) => {
                    if num > money {  
                        let insult = insults[rand::thread_rng().gen_range(0, insults.len())];
                        println!("You cannot place a bet higher than {}, you {}!", money, insult);
                        continue;
                    }  
                    else {
                        _bet_made = num;
                        break;
                    }
                }
                Err(_) => {
                    println!("Please input a number from 0 to {}, not {}.", money, bet.trim());
                    println!(" Can't you read instructions?");
                    continue;
                }
            };   
            
        }
        _bet_made
    }

    fn process_guess (guess: u32, secret_number: u32, insults: &[&str]) -> bool{
        let insult = insults[rand::thread_rng().gen_range(0, insults.len())];
        println!("You guessed: {}", guess);
        let mut retval = false;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, you {}!", insult),
            Ordering::Greater => println!("Too big, you {}!", insult),
            Ordering::Equal => {
                retval = true;
            }
        }
        retval
    }

    fn process_guesses_left(x: u32, number_of_chances: u32, money_ref: &mut u32, bet: u32) {
        let guesses_left = (number_of_chances - x) - 1;
        if guesses_left > 1 {
            println!("You have {} guesses left.", guesses_left);
        }
        else if guesses_left > 0 {
            println!("You have {} guess left.", guesses_left);
        }
        else {
            lost_bet(money_ref, bet);
        }
    }

    fn choose_difficulty () -> u32 {
        let max = 5;
        println!("Choose difficulty from 1 (easiest) to {} (hardest).", max);
        println!(" The difficulty multiplies your bet for the payout.");
        let _subtract_number: u32;
        loop {
            let mut difficulty = String::new();
            io::stdin().read_line(&mut difficulty)
                    .expect("Failed to read line");
            let _difficulty: u32 = match difficulty.trim().parse() {
                Ok(num) => {
                    if num > max {  
                        println!("Please input a number from 1 to {} or be stuck in this hell forever, not '{}'", max, num);
                        continue;
                    }  
                    else if num < 1 {
                        println!("Please input a number from 1 to {} or be stuck in this hell forever, not '{}'", max, num);
                        continue;
                    }
                    else {
                        _subtract_number = num;
                        break;
                    }
                }
                Err(_) => {
                    println!("Please input a number from 0 to 10 or be stuck in this hell forever, not '{}'", difficulty.trim());
                    continue;
                }
            };   
            
        } 
        _subtract_number
    }

    fn won_bet(money_ref: &mut u32, difficulty: u32, bet: u32) {
        let winnings = difficulty * bet;
        *money_ref += winnings;
        println!("You guessed right! You won {}. You now have {}.", winnings, *money_ref);
    }

    fn lost_bet (money_ref: &mut u32, bet: u32) {
        *money_ref -= bet;
        println!("You lost {}, you degenerate gambler! You have {} left.", bet, *money_ref);
    }
    
}

