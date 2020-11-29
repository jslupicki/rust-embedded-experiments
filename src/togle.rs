pub trait Togle {
    fn togle(&mut self);
}

impl Togle for bool {
    fn togle(&mut self) {
        *self ^= true;
    }
}