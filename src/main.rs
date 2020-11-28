use std::io;
use std::collections::HashMap;
use hex::encode;
use sha2::{Sha512, Digest};

fn main() {
    let mut creds: HashMap<String, String>  = HashMap::new();
    let expect = "Failed to readline";
    creds.insert("admin".to_string(), "123".to_string());
    let mut loggedin = false;

    loop{
        println!("{:?}", creds);
        let mut pin = String::new();
        let mut username  = String::new();
        let mut acc_action = String::new();

        println!("\nPress 1 to login \nPress 2 to create account");
        io::stdin().read_line(&mut acc_action).expect(expect);
        
        if acc_action.trim()  == "1" {
            println!("\nEnter username");
            io::stdin().read_line(&mut username).expect(expect);

            println!("Enter pin");
            io::stdin().read_line(&mut pin).expect(expect);

            let creds_pin = creds.get(&username.trim().to_string());

            pin = hash_pin(&pin.trim().to_string());

            if creds_pin == Some(&pin){
                loggedin = true;
                println!("\nHello {}", username);
            }else{
                println!("Invalid login");
                continue;
            }
        }else if acc_action.trim() == "2" {
            create_account(&mut creds);
        }else{
            continue;
        }

        if loggedin {
            loop {
                let mut balance: i32 = 100;
                let mut action = String::new();
                println!("Press 1 to withdraw \nPress 2 to deposit \nPress 3 to view account \nPress 4 to logout");
                io::stdin().read_line(&mut action).expect(expect);
                let action = action.trim();

                if action == "1" {
                    let mut amount = String::new();
                    io::stdin().read_line(&mut amount).expect(expect);
                    balance = withdraw_money(&amount, &balance);
                    view_acc(&balance);
                }else if action == "2" {
                    let mut amount = String::new();
                    io::stdin().read_line(&mut amount).expect(expect);
                    balance = deposit_money(&amount, &balance);
                    view_acc(&balance);
                }else if action == "3" {
                    view_acc(&balance);
                }else if action == "4" {
                    break;
                }else{
                    loggedin = false;
                    continue;
                }
            }
        }
    }
}

fn create_account(creds: &mut HashMap<String, String>) {
    let expect = "Failed to readline";

    loop{
        let mut un = String::new();
        let mut pn = String::new();
        let mut repin = String::new();

        println!("\nEnter username");
        io::stdin().read_line(&mut un).expect(expect);

        println!("Enter pin");
        io::stdin().read_line(&mut pn).expect(expect);

        println!("Re-enter your pin");
        io::stdin().read_line(&mut repin).expect(expect);

        pn = hash_pin(&pn.trim().to_string());
        repin = hash_pin(&repin.trim().to_string());

        if pn == repin {
            creds.insert(un.trim().to_string(), pn);
            println!("Account created");
            break;
        }else{
            println!("Pins do not match please try again");
            continue;
        }
    }
}

fn hash_pin(pin: &String) -> String {
    let mut hasher_pin = Sha512::new();
    hasher_pin.update(pin.as_bytes());
    let result = hasher_pin.finalize();
    encode(result)
}

fn withdraw_money(amnt: &String, balance: &i32) -> i32 {
    let with_amnt: i32 = amnt.trim().parse::<i32>().unwrap();
    balance - with_amnt
}

fn deposit_money(amnt: &String, balance: &i32) -> i32 {
    let dep_amnt: i32 = amnt.trim().parse::<i32>().unwrap();
    balance + dep_amnt
}

fn view_acc(balance: &i32) {
    println!("Your balance is {}", balance);
}

