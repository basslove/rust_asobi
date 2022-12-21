use rand::prelude::*;
use std::cell::RefCell;
use std::fmt;

trait Strategy {
    fn next_hand(&mut self) -> Hand;
    fn study(&mut self, win: bool);
}

#[derive(Clone)]
struct WinningStrategy {
    won: bool,
    prev_hand: Hand,
    rng: ThreadRng,
}

impl WinningStrategy {
    fn new() -> WinningStrategy {
        WinningStrategy {
            won: false,
            prev_hand: Hand::new(HandValue::Guu),
            rng: thread_rng(),
        }
    }
}

impl Strategy for WinningStrategy {
    fn next_hand(&mut self) -> Hand {
        if !self.won {
            let n = self.rng.gen_range(0..3);
            self.prev_hand = Hand::get_hand(n);
        }

        self.prev_hand.clone()
    }

    fn study(&mut self, win: bool) {
        self.won = win;
    }
}

#[derive(Clone)]
struct ProbStrategy {
    prev_hand: Hand,
    current_hand: Hand,
    rng: ThreadRng,
    history: [[u32; 3]; 3],
}

impl ProbStrategy {
    fn new() -> ProbStrategy {
        ProbStrategy {
            prev_hand: Hand::new(HandValue::Guu),
            current_hand: Hand::new(HandValue::Guu),
            rng: thread_rng(),
            history: [[1, 1, 1], [1, 1, 1], [1, 1, 1]],
        }
    }

    fn get_sum(&self) -> u32 {
        let prev_hand_value = self.current_hand.hand_value as usize;
        self.history[prev_hand_value].iter().sum()
    }
}

impl Strategy for ProbStrategy {
    fn next_hand(&mut self) -> Hand {
        let current_hand_value = self.current_hand.hand_value as usize;

        let sum = self.get_sum();
        let bet = self.rng.gen_range(0..sum);

        let hand_value = if bet < self.history[current_hand_value][0] {
            0
        } else if bet < (self.history[current_hand_value][0] + self.history[current_hand_value][1]) {
            1
        } else {
            2
        };

        self.prev_hand = Hand::get_hand(current_hand_value as u32);
        self.current_hand = Hand::get_hand(hand_value);
        Hand::get_hand(hand_value)
    }

    fn study(&mut self, win: bool) {
        let prev_hand_value = self.prev_hand.hand_value as usize;
        let current_hand_value = self.current_hand.hand_value as usize;

        if win {
            self.history[prev_hand_value][current_hand_value] += 1;
        } else {
            self.history[prev_hand_value][(current_hand_value + 1) % 3] += 1;
            self.history[prev_hand_value][(current_hand_value + 2) % 3] += 1;
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum HandValue {
    Guu,
    Cho,
    Paa,
}

#[derive(PartialEq, Clone)]
struct Hand {
    hand_value: HandValue,
    name: String,
}

impl Hand {
    fn new(hand_value: HandValue) -> Self {
        match hand_value {
            HandValue::Guu => Hand {
                hand_value,
                name: "グー".to_string(),
            },
            HandValue::Cho => Hand {
                hand_value,
                name: "チョキ".to_string(),
            },
            HandValue::Paa => Hand {
                hand_value,
                name: "パー".to_string(),
            },
        }
    }

    fn get_hand(hand_value: u32) -> Hand {
        match hand_value {
            0 => Hand {
                hand_value: HandValue::Guu,
                name: "グー".to_string(),
            },
            1 => Hand {
                hand_value: HandValue::Cho,
                name: "チョキ".to_string(),
            },
            2 => Hand {
                hand_value: HandValue::Paa,
                name: "パー".to_string(),
            },
            _ => panic!("random number is over raange."),
        }
    }

    fn is_stronger_than(&self, hand: &Hand) -> bool {
        self.fight(hand) == 1
    }

    fn is_weaker_than(&self, hand: &Hand) -> bool {
        self.fight(hand) == -1
    }

    fn fight(&self, hand: &Hand) -> i32 {
        let self_hand_value = self.hand_value as u32;
        let vs_hand_value = hand.hand_value as u32;

        if self_hand_value == vs_hand_value {
            0
        } else if ((self_hand_value + 1) % 3) == vs_hand_value {
            1
        } else {
            -1
        }
    }
}

struct Player {
    name: String,
    strategy: RefCell<Box<dyn Strategy>>,
    win_count: u32,
    lose_count: u32,
    game_count: u32,
}

impl Player {
    fn new(name: &str, strategy: Box<dyn Strategy>) -> Player {
        Player {
            name: name.to_string(),
            strategy: RefCell::new(strategy),
            win_count: 0,
            lose_count: 0,
            game_count: 0,
        }
    }

    fn next_hand(&mut self) -> Hand {
        self.strategy.borrow_mut().next_hand()
    }

    fn win(&mut self) {
        self.strategy.borrow_mut().study(true);
        self.win_count += 1;
        self.game_count += 1;
    }

    fn lose(&mut self) {
        self.strategy.borrow_mut().study(false);
        self.lose_count += 1;
        self.game_count += 1;
    }

    fn even(&mut self) {
        self.game_count += 1;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}:{} games, {} win, {} lose]", self.name, self.game_count, self.win_count, self.lose_count)
    }
}

pub fn execute() {
    println!("composite");

    let mut player1 = Player::new("Taro", Box::new(WinningStrategy::new()));
    let mut player2 = Player::new("Hana", Box::new(ProbStrategy::new()));

    for _ in 0..100 {
        let next_hand1 = player1.next_hand();
        let next_hand2 = player2.next_hand();

        if next_hand1.is_stronger_than(&next_hand2) {
            println!("Winner:{}", player1);
            player1.win();
            player2.lose();
        } else if next_hand1.is_weaker_than(&next_hand2) {
            println!("Winner:{}", player2);
            player2.win();
            player1.lose();
        } else {
            println!("Even...");
            player1.even();
            player2.even();
        }
    }

    println!("Total result:");
    println!("{}", player1);
    println!("{}", player2);
}
