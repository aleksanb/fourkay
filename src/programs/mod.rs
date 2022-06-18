mod quad;
mod raymarcher;

pub use self::quad::Quad;
pub use self::raymarcher::Raymarcher;

pub trait Program: Sized {
    fn update(&mut self, time: f32);
    fn resize(&mut self, width: i32, height: i32);
    fn render(&self, time: f32);
}
