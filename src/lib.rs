#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]
use std::collections::VecDeque;
use std::ops::{Coroutine, CoroutineState};
use std::pin::{pin, Pin};
use std::borrow::Cow;
// dungeon_banker::App
pub struct App {
    // requests: VecDeque<Request>,
    // coroutine: Box<dyn Coroutine<Yield = Request, Return = ()>>,
}

impl Default for App {
    fn default() -> Self {
        Self {}
        // let mut coroutine = #[coroutine] move || {
        //     yield Request::Print { message: "hi".into() };
        //     yield Request::Print { message: "there".into() };
        //     return ()
        // };
        // Self {
        //     requests: VecDeque::default(),
        //     coroutine: Box::new(coroutine),
        // }
    }

}

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
 // Allow coroutine to be self-referential (not `Unpin`)
 // vvvvvv        so that locals can cross yield points.
    #[coroutine] static |response: Response| {
        // let foo = String::from("foo");
        // let foo_ref = &foo; // ------+
        // yield 0;                  // | <- crosses yield point!
        // println!("{foo_ref}"); // <--+
        // yield foo.len();
        //
        // let response: Response = yield Request::Print { message: "hi".into() };
        let response = yield Request::Print { message: "Hi".into() };
        let response = yield Request::QueryText { question: "What's your name? ".into() };
        let response = yield Request::Print { message: format!("Oh, {} is a nice name!", &response.as_str()) };
        // eprintln!(" got response {response:?}");

        return ()
    }
}


// impl Coroutine<Response> for App {
//     type Yield = Request;
//     type Return = ();
//     fn resume(self: Pin<&mut Self>, resume: Response) -> CoroutineState<Self::Yield, Self::Return> {
//         CoroutineState::Yielded(Request::Print("hi".into()))


//     }

// }

impl App {

    // pub fn update(&mut self, response: Response) -> Request {
    //     match Pin::new(&mut self.coroutine).resume(response) {
    //         CoroutineState::Yielded(y) => {
    //             println!("got one {:?}", &y);
    //         }
    //         _ => panic!("unexpected value from resume"),
    //     }
    //     // match Pin::new(&mut coroutine).resume(()) {
    //     //     CoroutineState::Complete("foo") => {

    //     //         println!("got foo");
    //     //     }
    //     //     _ => panic!("unexpected value from resume"),
    //     // }
    // }

}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
