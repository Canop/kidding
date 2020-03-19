use {
    crossterm::style::Colorize,
    rand::Rng,
    std::io::*,
};

struct QA {
    question: String,
    answer: i64,
}
impl QA {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 5) {
            0 => {
                // halving
                let a = rng.gen_range(3, 20);
                Self {
                    question: format!("{} / 2", a*2),
                    answer: a,
                }
            }
            1 => {
                // substraction (result may be negative)
                let (a, b) = (rng.gen_range(1, 30), rng.gen_range(1, 20));
                Self {
                    question: format!("{} - {}", a, b),
                    answer: a - b,
                }
            }
            2 => {
                // multiplication
                let (a, b) = (rng.gen_range(2, 6), rng.gen_range(1, 10));
                Self {
                    question: format!("{} x {}", a, b),
                    answer: a * b,
                }
            }
            _ => {
                // addition
                let (a, b) = (rng.gen_range(1, 50), rng.gen_range(1, 50));
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
