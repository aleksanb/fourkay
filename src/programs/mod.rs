mod quad;
pub use self::quad::Quad;

pub trait Program: Sized {
    fn new() -> Result<Self, ()>;
    fn update(&mut self, frame: u64);
    fn render(&self, frame: u64);
}
