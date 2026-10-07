use std::env;
use std::io::{self, Write};

use crate::tester;

struct Exercise {
    id: usize,
    name: &'static str,
    run: fn(),
}

const EXERCISES: &[Exercise] = &[
    Exercise {
        id: 0,
        name: "Add / Subtract / Scale",
        run: tester::ex00::run,
    },
    Exercise {
        id: 1,
        name: "Linear Combination",
        run: tester::ex01::run,
    },
    Exercise {
        id: 2,
        name: "Linear Interpolation",
        run: tester::ex02::run,
    },
    Exercise {
        id: 3,
        name: "Dot Product",
        run: tester::ex03::run,
    },
    Exercise {
        id: 4,
        name: "Norms",
        run: tester::ex04::run,
    },
    Exercise {
        id: 5,
        name: "Cosine",
        run: tester::ex05::run,
    },
    Exercise {
        id: 6,
        name: "Cross Product",
        run: tester::ex06::run,
    },

];

pub fn run() {
    let args: Vec<String> = env::args().skip(1).collect();

    if let Some(command) = args.first() {
        execute_command(command);
        return;
    }

    interactive_mode();
}

fn interactive_mode() {
    loop {
        print_menu();

        print!("Choose an exercise: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }

        let command = input.trim();

        match command {
            "q" | "quit" | "exit" => break,

            _ => execute_command(command),
        }

        println!();
        println!("Press Enter to continue...");

        let mut pause = String::new();
        let _ = io::stdin().read_line(&mut pause);
    }
}

fn execute_command(command: &str) {
    match command {
        "all" => run_all(),

        "help" | "-h" | "--help" => print_help(),

        _ => {
            if let Ok(id) = command.parse::<usize>() {
                run_exercise(id);
            } else {
                println!("Unknown command: {}", command);
            }
        }
    }
}

fn run_exercise(id: usize) {
    match EXERCISES.iter().find(|exercise| exercise.id == id) {
        Some(exercise) => {
            println!();
            println!("========================================");
            println!(
                " Exercise {:02} - {}",
                exercise.id,
                exercise.name
            );
            println!("========================================");

            (exercise.run)();
        }

        None => {
            println!("Exercise {:02} does not exist.", id);
        }
    }
}

fn run_all() {
    for exercise in EXERCISES {
        println!();
        println!("========================================");
        println!(
            " Exercise {:02} - {}",
            exercise.id,
            exercise.name
        );
        println!("========================================");

        (exercise.run)();
    }
}

fn print_menu() {
    println!();
    println!("========================================");
    println!("             MATRIX TESTER");
    println!("========================================");

    for exercise in EXERCISES {
        println!(
            " {:02} - {}",
            exercise.id,
            exercise.name
        );
    }

    println!();
    println!(" all  - Run all exercises");
    println!(" help - Show help");
    println!(" q    - Quit");
    println!("========================================");
}

fn print_help() {
    println!();
    println!("Usage:");
    println!("  cargo run");
    println!("  cargo run -- 0");
    println!("  cargo run -- 02");
    println!("  cargo run -- all");
}