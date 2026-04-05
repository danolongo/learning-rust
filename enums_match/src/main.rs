#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("pinga challenge");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {state:?}");
            25
        }
    }
}

fn plus1(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let malika_coin = Coin::Quarter(UsState::Alabama);
    value_in_cents(&malika_coin);

    let daniel = Coin::Penny;
    value_in_cents(&daniel);

    let five = Some(5);
    let six = plus1(five);
    let none = plus1(None);

    println!("{:#?}", six.unwrap());
    println!("{:#?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        _ => (),
    }
    fn add_hat() {}
    fn remove_hat() {}
    // fn move_player() {
    //     println!("hola malika");
    // }

    let mut count = 0;
    if let Coin::Quarter(state) = &malika_coin {
        println!("manzana {state:?}");
    } else {
        count += 1;
    }

    fn describe_state_quarter(coin: &Coin) -> Option<String> {
        let Coin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} old asf"))
        } else {
            Some(format!("{state:?} young n turnt"))
        }
    }

    println!("{:?}", describe_state_quarter(&malika_coin).unwrap());
}
