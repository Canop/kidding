use {
    crossterm::style::Colorize,
    rand::Rng,
    std::io::*,
};

fn int() -> i64 {
    rand::thread_rng().gen_range(10, 50)
}

struct QA {
    question: String,
    answer: i64,
}
impl QA {
    fn new() -> Self {
        let (a, b) = (int(), int());
        Self {
            question: format!("{} + {}", a, b),
            answer: a + b,
        }
    }
}

fn main() {
    let stdin = stdin();
    let mut input = stdin.lock().lines().map(|l| l.unwrap());
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
                    println!("{}", "Wrong".red());
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
