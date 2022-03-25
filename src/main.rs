extern crate sdl2;

pub mod document;
pub mod renderer;

use renderer::Renderer;

use std::thread::sleep;
use std::time::Duration;

pub struct SdlContext {
    pub sdl: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub ttf: sdl2::ttf::Sdl2TtfContext,
    _image: sdl2::image::Sdl2ImageContext,
}

impl SdlContext {
    // Initializes the SDL context
    fn init() -> Result<SdlContext, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;
        Ok(SdlContext {
            sdl,
            video_subsystem,
            ttf: sdl2::ttf::init().map_err(|e| e.to_string())?,
            _image: sdl2::image::init(sdl2::image::InitFlag::PNG)?,
        })
    }
}

fn main() -> Result<(), String> {
    let sdl_context = SdlContext::init()?;

    let mut renderer = Renderer::new(&sdl_context)?;
    renderer.clear();
    renderer.update();

    println!("{:?}", renderer.dimensions());

    sleep(Duration::new(5, 0));

    Ok(())
}
