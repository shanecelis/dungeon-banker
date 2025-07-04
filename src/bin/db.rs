#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]
use dungeon_banker::*;

use std::ops::{Coroutine, CoroutineState};
use std::pin::{pin, Pin};
use std::io::{self, Write};

fn main() {
    let mut app = dungeon_banker::App::default();
    let mut coroutine = pin!(dungeon_banker::level0());
    let mut response = Response::Unit;
    loop {
        match Pin::new(&mut coroutine).resume(response) {
            CoroutineState::Yielded(request) => {
                match request {
                    Request::Print { message } => {
                        println!("{}", &message);
                        response = Response::Unit;
                    }
                    Request::QueryText { question } => {
                        println!("{}", &question);
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        input.truncate(input.trim().len());
                        response = Response::Text { content: input };
                    }
                    Request::QueryOne { question, options } => todo!(),
                    Request::QueryMany { question, options } => todo!(),
                }
            }
            CoroutineState::Complete(_) => {
                println!("COMPLETED");
                break;
            }
            _ => panic!("unexpected value from resume"),
        }
    }
    // app.run();
}
