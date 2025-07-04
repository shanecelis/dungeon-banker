#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]
use std::collections::VecDeque;
use std::ops::{Coroutine, CoroutineState};
use std::pin::{pin, Pin};
use std::borrow::Cow;
// dungeon_banker::App
#[derive(Debug, Default)]
pub struct App {}

#[derive(Debug)]
pub enum Request {
    Print { message: String },
    QueryText { question: String },
    QueryOne { question: String, options: Vec<String> },
    QueryMany { question: String, options: Vec<String> },
}

#[derive(Debug)]
pub enum Response {
    Unit,
    One { index: usize },
    Many { indices: Vec<usize> },
    Text { content: String },
}

impl Response {
    pub fn as_str(&self) -> Cow<'_, str> {
        match self {
            Response::Unit => "".into(),
            Response::One { index } => todo!(),
            Response::Many { indices } => todo!(),
            Response::Text { content } => content.into()
        }
    }
}

pub fn level0() -> impl Coroutine<Response, Yield = Request, Return = ()> /* not Unpin */ {
    #[coroutine] static |response: Response| {
        let response = yield Request::Print { message: "Hi".into() };
        let response = yield Request::QueryText { question: "What's your name? ".into() };
        let response = yield Request::Print { message: format!("Oh, {} is a nice name!", &response.as_str()) };
        return ()
    }
}
