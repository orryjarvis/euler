use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

pub struct Generator<C, T>
where C: Coroutine<Yield = T, Return = ()>,
{
    coroutine: Pin<Box<C>>,
    done: bool,
}

pub struct InfiniteGenerator<C, T>
where C: Coroutine<Yield = T, Return = !>,
{
    coroutine: Pin<Box<C>>
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

impl<C, T> Iterator for InfiniteGenerator<C, T>
where
    C: Coroutine<Yield = T, Return = !>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.coroutine.as_mut().resume(()) {
            CoroutineState::Yielded(val) => Some(val),
            CoroutineState::Complete(never) => match never {}
        }
    }
}

pub fn create_generator<C, T>(coroutine: C) -> Generator<C, T>
where
    C: Coroutine<Yield = T, Return = ()>,
{
    Generator { coroutine: Box::pin(coroutine), done: false }
}

pub fn create_infinite_generator<C, T>(coroutine: C) -> InfiniteGenerator<C, T>
where
    C: Coroutine<Yield = T, Return = !>,
{
    InfiniteGenerator { coroutine: Box::pin(coroutine) }
}
