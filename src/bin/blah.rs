#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

fn my_coroutine() -> impl Coroutine<usize, Yield=u32, Return=()> {
    #[coroutine] |input| {
        let mut sum = 0;
        let mut input: usize = yield 1; // First yield, receives initial input

        loop {
            sum += input;
            input = yield sum as u32; // Subsequent yields, receives updated input
        }
    }
}

fn main() {
    let mut co = my_coroutine();

    // Start the coroutine with an initial resume value
    let mut state = Pin::new(&mut co).resume(10);

    // Resume and interact with the coroutine
    while let CoroutineState::Yielded(yielded_value) = state {
        println!("Coroutine yielded: {}", yielded_value);

        // Pass a new value upon resuming
        state = Pin::new(&mut co).resume(yielded_value as usize + 5);
    }

    // When the coroutine is completed, the final value is printed
    if let CoroutineState::Complete(return_value) = state {
        println!("Coroutine completed with return value: {:?}", return_value); // In this case, ()
    }
}
