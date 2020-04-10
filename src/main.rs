use {
    crossterm::style::Colorize,
    rand::Rng,
    std::{
        fmt::Write,
        io::*,
    },
};

struct QA {
    question: String,
    answer: i64,
}
impl QA {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 7) {
            0 => {
                // halving
                let a = rng.gen_range(4, 60);
                Self {
                    question: format!("{} / 2", a*2),
                    answer: a,
                }
            }
            1 => {
                // substraction (result may be negative)
                let (a, b) = (rng.gen_range(1, 40), rng.gen_range(1, 30));
                Self {
                    question: format!("{} - {}", a, b),
                    answer: a - b,
                }
            }
            2 => {
                // multiplication
                let (a, b) = (rng.gen_range(2, 9), rng.gen_range(2, 12));
                Self {
                    question: format!("{} x {}", a, b),
                    answer: a * b,
                }
            }
            3 => {
                // multiple small additions and substractions
                let mut answer = rng.gen_range(1, 21);
                let mut question = format!("{}", answer);
                for _ in 0..rng.gen_range(2, 6) {
                    let a = rng.gen_range(1, 17);
                    if rng.gen_bool(0.7) {
                        answer += a;
                        write!(&mut question, " + {}", a).unwrap();
                    } else {
                        answer -= a;
                        write!(&mut question, " - {}", a).unwrap();
                    }
                }
                Self {
                    question,
                    answer,
                }
            }
            4 => {
                // addition of two big numbers
                let a = rng.gen_range(20, 60) * rng.gen_range(25, 55) * rng.gen_range(2, 15);
                let b = rng.gen_range(25, 50) * rng.gen_range(15, 65) * rng.gen_range(2, 10);
                Self {
                    question: format!("{} + {}", a, b),
                    answer: a + b,
                }
            }
            _ => {
                // addition
                let (a, b) = (rng.gen_range(1, 120), rng.gen_range(1, 80));
                Self {
                    question: format!("{} + {}", a, b),
                    answer: a + b,
                }
            }
        }
    }
}

fn main() {
    let stdin = stdin();
    let mut input = stdin.lock().lines().flatten();
    let (mut good, mut bad) = (0, 0);
    loop {
        let qa = QA::new();
        println!("{} = ?", qa.question);
        match input.next().unwrap().parse::<i64>() {
            Ok(answer) => {
                if answer == qa.answer {
                    println!("{}", "Right!".green());
                    good += 1;
                } else {
                    println!("{} {} = {}", "Wrong!".red(), qa.question, qa.answer);
                    bad += 1;
                }
            }
            Err(_) => { // anything not a number quits
                println!("You got {} / {}", good, good + bad );
                break;
            }
        }
    }
}
