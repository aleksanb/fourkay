mod raymarcher;
mod simple_shader;

pub use self::raymarcher::Raymarcher;
pub use self::simple_shader::Quad;

pub trait Program: Sized {
    fn new() -> Result<Self, ()>;
    fn update(&mut self, frame: u64);
    fn render(&self, frame: u64);
}
