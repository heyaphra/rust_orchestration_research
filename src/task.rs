pub trait Callable<T> {
    fn run(&self) -> T;
}

pub struct Task<'a, T: ?Sized> {
    handle: &'a dyn Fn() -> T,
}

impl<T: ?Sized> Task<'_, T> {
    pub fn new(handle: &dyn Fn() -> T) -> Task<T> {
        Task { handle }
    }
}

impl<T> Callable<T> for Task<'_, T> {
    fn run(&self) -> T {
        (self.handle)()
    }
}
