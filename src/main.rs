use {
    crossterm::style::Colorize,
    rand::Rng,
    std::{
        fmt::Write,
        io::*,
    },
};

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Lang {
    Fr,
    En,
}

// This file is MEANT to be modified. Run it, change it for your kid, and run it again.

const LANG: Lang = Lang::En; // <- change this to Fr to have the messages in French

struct QA {
    question: String,
    answer: i64,
}
impl QA {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 7) {
            // Generate a random number between 0 and 6
            // you may comment out a case to remove it from the possible operations if you
            // don't like it, you may also add more cases to add more operations (but then
            // you should also change the range of the random number just above)
            0 => {
                // halving
                let a = rng.gen_range(4, 60);
                Self {
                    question: format!("{} / 2", a * 2),
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
                let (a, b) = (rng.gen_range(2, 5), rng.gen_range(2, 6));
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
                Self { question, answer }
            }
            4 => {
                // addition of two big numbers
                let a = rng.gen_range(20, 60) * rng.gen_range(10, 75) * rng.gen_range(2, 15)
                    + rng.gen_range(1, 100);
                let b = rng.gen_range(25, 45) * rng.gen_range(15, 60) * rng.gen_range(2, 10)
                    + rng.gen_range(0, 9000);
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
    let mut input = stdin.lock().lines().map_while(Result::ok);
    let (mut good, mut bad) = (0, 0);
    let mut successive_good = 0;
    let min_successive_good = 3; // <- change this to set the number of good answers in a row to win
    let min_good = 10; // <- change this to set the number of good answers to win
    let goal_percent = 80; // <- change this to set the percentage of good answers to win
    let mut qa = QA::new();
    match LANG {
        Lang::Fr => {
            println!(
                "Pour gagner, il faut avoir {} bonnes réponses de suite, {} bonnes réponses au total, ET au moins {}% de réussite",
                min_successive_good,
                min_good,
                goal_percent,
            );
        }
        Lang::En => {
            println!(
                "To win, you need {} successive right answers, a total of {} right answers, and a win rate at least {}%",
                min_successive_good,
                min_good,
                goal_percent,
            );
        }
    }
    loop {
        println!("{} = ?", qa.question);
        match input.next().unwrap().trim().parse::<i64>() {
            Ok(answer) => {
                if answer == qa.answer {
                    match LANG {
                        Lang::Fr => println!("{}", "Juste".green()),
                        Lang::En => println!("{}", "Right".green()),
                    }
                    good += 1;
                    successive_good += 1;
                    if good >= min_good {
                        if successive_good >= min_successive_good {
                            let percent = 100 * good / (good + bad);
                            let won = percent >= goal_percent;
                            println!("{} réussites sur {} : {}%", good, good + bad, percent);
                            match LANG {
                                Lang::Fr => {
                                    println!(
                                        "{} réussites sur {} : {}%",
                                        good,
                                        good + bad,
                                        percent
                                    );
                                }
                                Lang::En => {
                                    println!("{} wins over {} : {}%", good, good + bad, percent);
                                }
                            }
                            if won {
                                println!("BRAVO!");
                                break;
                            }
                        }
                    } else {
                        match LANG {
                            Lang::Fr => println!("{} bonnes réponses sur {}", good, min_good),
                            Lang::En => println!("{} right answers over {}", good, min_good),
                        }
                    }
                } else {
                    match LANG {
                        Lang::Fr => println!("{} {} = {}", "Faux!".red(), qa.question, qa.answer),
                        Lang::En => println!("{} {} = {}", "Wrong!".red(), qa.question, qa.answer),
                    }
                    bad += 1;
                    successive_good = 0;
                }
                qa = QA::new();
            }
            Err(_) => { // anything not a number quits if you uncomment the following line
                 //break;
            }
        }
    }
}
