#[derive(Debug)] // so we can inspect the state in a minute
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter (UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter (state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(v) => Some (v+1),
        None    => None
    }
}

// Matches are exhaustive.
// fn plus_one_compilation_error(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }


fn main()
{
    let c : Coin = Coin::Quarter(UsState::Alabama);
    let value : u8 = value_in_cents(&c);
    println!("Coin = {c:?}, Value in Cents = {value}");

    let five = Some(5);
    println!("five = {five:?}");
    let six = plus_one(five);
    println!("six = {six:?}");
    let none = plus_one(None);
    println!("none = {none:?}");

    let dice_roll = 9;
    // Catch-all
    match dice_roll {
      3 => add_fancy_hat(),
      7 => remove_fancy_hat(),
      _ => (),
    }
}


fn add_fancy_hat() {}
fn remove_fancy_hat() {}