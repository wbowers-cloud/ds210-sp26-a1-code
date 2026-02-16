use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        let mut maximum = max;
        let mut minimum = min;
        loop{
        let mut guess = (maximum + minimum) / 2;

        let mut result = player.ask_to_compare(guess);

        if result == 0{
            return guess;
        } else if result == 1{
            minimum = guess;
        } else{
            maximum = guess;
        }

        }
       
    }
}
