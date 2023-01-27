pub trait Flip {
    fn flip(&mut self) -> bool;
}

impl Flip for bool {
    fn flip(&mut self) -> bool {
        *self = !*self;

        *self
    }
}