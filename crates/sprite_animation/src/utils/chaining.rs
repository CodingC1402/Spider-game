/// Used to chain functions together
pub trait Chaining<T, R> where T : Fn() -> R {
    fn chain(&self, f: T) -> R;
    fn chain_some(&self, r: R) -> R;
}

impl<T, R> Chaining<T, R> for () where T : Fn() -> R {
    fn chain(&self, f: T) -> R {
        f()
    }

    fn chain_some(&self, r: R) -> R {
        r
    }
}