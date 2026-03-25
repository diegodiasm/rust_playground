#[derive(Debug)] // so we can inspect the state in a minute
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter (UsState),
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarterv(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main(){

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // let mut count = 0;
    let coins : [Coin; 5]= [Coin::Penny,
                            Coin::Nickel,
                            Coin::Dime,
                            Coin::Quarter (UsState::Alabama),
                            Coin::Quarter (UsState::Alaska)];



    // for coin in coins
    // {
    //   if let Coin::Quarter(state) = coin {
    //       describe_state_quarter(coin);
    //   } else {
    //       count += 1;
    //   }
    // }

    for coin in coins
    {
        //println!("{:?}", describe_state_quarter(coin));
        println!("{:?}", describe_state_quarterv(coin));
    }


    // println!("Counted {count} non State quarter coins.");
}