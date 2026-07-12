use std::{char, io::{Write, stdin, stdout}};

static WORD: &str = "hello";
static TRIES: u64 = 5;

#[derive(PartialEq, Debug)] // For tests
enum GameState {
    WaitingForInput,      // ждём, пока игрок введёт букву
    CheckingGuess(char),  // получили букву, проверяем её (буква лежит внутри варианта)
    Won,                  // слово угадано целиком
    Lost,                 // попытки кончились
}

struct Game {
    word: String,
    guessed: Vec<char>,
    used_letters: Vec<char>,
    tries: u64,
}

impl Game {

    fn new(word: String) -> Self {
        let guessed = vec!['_'; word.chars().count()];
        Self {
            word,
            guessed,
            used_letters: Vec::new(),
            tries: TRIES,
        }
    }

    fn check_guess(&mut self, c: char) -> GameState {

        if self.used_letters.iter().any(|ch| ch.eq_ignore_ascii_case(&c)) {
            println!("You already tried this letter");
            return GameState::WaitingForInput;
        }

        let mut char_exists = false;
        for (index, ch) in self.word.chars().enumerate() {
            if ch.eq_ignore_ascii_case(&c) {
                self.guessed[index] = ch;
                char_exists = true;
            }
        }

        self.used_letters.push(c);

        if !char_exists {
            println!("Wrong letter!");
            self.tries -= 1;
        } else {
            println!("Letter exists!")
        }

        if !self.guessed.contains(&'_') {
            GameState::Won
        } else if self.tries == 0 {
            GameState::Lost
        } else {
            GameState::WaitingForInput
        }
    }
}

fn main() {


    let mut game = Game::new(String::from(WORD));
    let mut state = GameState::WaitingForInput;

    loop {

        state = match state {
            GameState::WaitingForInput => {

                println!("Word: {}", game.guessed.iter().collect::<String>());
                println!("Used letters: {}", game.used_letters.iter().collect::<String>());
                println!("Tries: {}", game.tries);
                print!("Input letter: ");
                
                let _ = stdout().flush();


                let mut input = String:: new();
                stdin().read_line(&mut input).expect("Can't read letter");
                println!("--------------");


                match input.trim().chars().next() {
                    Some(c) => GameState::CheckingGuess(c),
                    None => GameState::WaitingForInput
                }
            },
            GameState::CheckingGuess(ch) => {
                game.check_guess(ch) 
            }, 
            GameState::Lost => {
                println!("You lost! Correct word: {}", game.word);
                break;
            },
            GameState::Won => {
                println!("You won! The word was: {}", game.word);
                break;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_game_created() {
        let game = Game::new(String::from(WORD));
        // слово ещё не открыто — все буквы скрыты
        assert_eq!(game.guessed, vec!['_', '_', '_', '_', '_']);
        assert_eq!(game.tries, TRIES);
        assert_eq!(game.word, WORD);
        assert!(game.used_letters.is_empty());
    }

    #[test]
    fn correct_guess_reveals_all_matching_letters() {
        let mut game = Game::new(String::from(WORD));
        let state = game.check_guess('l');
        assert_eq!(game.guessed, vec!['_', '_', 'l', 'l', '_']); 
        assert_eq!(game.tries, TRIES); // верная буква не тратит попытку
        assert_eq!(state, GameState::WaitingForInput);
        assert!(game.used_letters.contains(&'l'));
    }

    #[test]
    fn correct_guess_is_case_insensitive() {
        let mut game = Game::new(String::from(WORD));
        game.check_guess('H');
        assert_eq!(game.guessed[0], 'h');
        assert_eq!(game.tries, TRIES);
    }

    #[test]
    fn wrong_guess_decrements_tries() {
        let mut game = Game::new(String::from(WORD));
        let state = game.check_guess('q');

        assert_eq!(game.tries, TRIES - 1);
        assert_eq!(game.guessed, vec!['_', '_', '_', '_', '_']);
        assert!(game.used_letters.contains(&'q'));
        assert!(matches!(state, GameState::WaitingForInput));
    }

    #[test]
    fn repeated_letter_does_not_cost_a_try() {
        let mut game = Game::new(String::from("hello"));
        game.check_guess('z');
        assert_eq!(game.tries, TRIES - 1);

        let state = game.check_guess('z');
        assert_eq!(game.tries, TRIES - 1);
        assert!(matches!(state, GameState::WaitingForInput));
    }

    #[test]
    fn guessing_all_letters_wins() {
        let mut game = Game::new(String::from("abc"));
        game.check_guess('a');
        game.check_guess('b');
        let state = game.check_guess('c');

        assert_eq!(state, GameState::Won);
        assert!(!game.guessed.contains(&'_'));
    }

    #[test]
    fn running_out_of_tries_loses() {
        let mut game = Game::new(String::from("abc"));
        let wrong = ['z', 'x', 'q', 'w', 'v'];
        let mut last_state = GameState::WaitingForInput;
        for &c in wrong.iter() {
            last_state = game.check_guess(c);
        }

        assert_eq!(game.tries, 0);
        assert_eq!(last_state, GameState::Lost);
    }
}