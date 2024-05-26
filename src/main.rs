use poker_rs::{get_minimum_profitable_bluff_win_rate, get_minimum_profitable_call_win_rate};

fn main() {
    let pot_before_bet = 50.0;
    let bet = 30.0;

    let call_win_percentage = get_minimum_profitable_call_win_rate(pot_before_bet, bet);
    println!("The win percentage needed for the call to be profitable is {:.2}%", call_win_percentage * 100.0);

    let bluff_success_percentage = get_minimum_profitable_bluff_win_rate(pot_before_bet, bet);
    println!("The success percentage needed for the bluff to be profitable is {:.2}%", bluff_success_percentage * 100.0);
}