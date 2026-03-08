use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::window::{Window, WindowBuilder};
use winit::event_loop::EventLoop;

pub struct Display {
  pixels: Pixels,
  window: Window,
}

impl Display {
  pub fn new(event_loop: &EventLoop<()>) -> Self {
    let window = WindowBuilder::new()
      .with_title("CHIP-8")
      .with_inner_size(LogicalSize::new(640, 320))
      .build(event_loop)
      .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(
      window_size.width, 
      window_size.height, 
      &window
    );

    let pixels = Pixels::new(64, 32, surface_texture).unwrap();

    Display { pixels, window }
  }

  pub fn draw(&mut self, display: &[bool; 64  * 32]) {
    let frame = self.pixels.frame_mut();

    for (pixel, &on) in frame.chunks_exact_mut(4).zip(display.iter()) {
      let color = if on {
        [255, 255, 255, 255]
      } else {
        [0, 0, 0, 255]
      };
      pixel.copy_from_slice(&color);
    }

    self.pixels.render().unwrap();
  }
}
