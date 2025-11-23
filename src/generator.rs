use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

pub struct Generator<C, T>
where C: Coroutine<Yield = T, Return = ()>,
{
    coroutine: Pin<Box<C>>,
    done: bool,
}

impl<C, T> Iterator for Generator<C, T>
where
    C: Coroutine<Yield = T, Return = ()>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        match self.coroutine.as_mut().resume(()) {
            CoroutineState::Yielded(val) => Some(val),
            CoroutineState::Complete(()) => {
                self.done = true;
                None
            }
        }
    }
}

pub fn create_generator<C, T>(coroutine: C) -> Generator<C, T>
where
    C: Coroutine<Yield = T, Return = ()>,
{
    Generator { coroutine: Box::pin(coroutine), done: false }
}
