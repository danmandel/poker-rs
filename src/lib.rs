/// Calculates the win percentage required for calling to be profitable.
/// Referenced from Page 5 of "Applications of No-Limit Hold 'em".
/// 
/// ## Arguments
/// * `pot_before_bet` - The size of the pot, not including the player's bet.
/// * `bet` - The amount the player is considering calling.
///
///  ## Returns
///  The win percentage required in order for calling to be profitable.
pub fn get_minimum_profitable_call_win_rate(pot_before_bet: f64, bet: f64) -> f64 {
    let total_pot = pot_before_bet + bet;
    bet / (total_pot + bet)


}

/// Calculates the win percentage required for bluffing to be profitable.
/// Referenced from Page 5 of "Applications of No-Limit Hold 'em".
/// 
/// ## Arguments
/// * `pot_before_bet` - The size of the pot, not including the player's bet.
/// * `bet` - The amount the player is considering bluffing.
///
///  ## Returns
///  The win percentage required in order for bluffing to be profitable.
pub fn get_minimum_profitable_bluff_win_rate(pot_before_bet: f64, bet: f64) -> f64 {
    bet / (pot_before_bet + bet)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_minimum_profitable_call_win_rate(){
        let pot_before_bet = 50.0;
        let bet = 30.0;
        let required_win_rate = 0.273;
        let result = get_minimum_profitable_call_win_rate(pot_before_bet, bet);
        println!("{}", required_win_rate);
        println!("{}", result);
        assert!((result - required_win_rate).abs() < 0.001)
    }

    #[test]
    fn test_get_minimum_profitable_bluff_win_rate() {
        let pot_before_bet = 50.0;
        let bet = 30.0;
        let required_win_rate = 0.375;
        let result = get_minimum_profitable_bluff_win_rate(pot_before_bet, bet);
        println!("{}", required_win_rate);
        println!("{}", result);
        assert!((result - required_win_rate).abs() < 0.001);
    }
}
