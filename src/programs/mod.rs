mod raymarcher;
mod quad;

pub use self::raymarcher::Raymarcher;
pub use self::quad::Quad;

pub trait Program: Sized {
    fn new() -> Result<Self, ()>;
    fn update(&mut self, frame: u64);
    fn resize(&mut self, width: i32, height: i32);
    fn render(&self, frame: u64);
}
