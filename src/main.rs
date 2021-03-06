extern crate sdl2;

pub mod app;
pub mod cursor;
pub mod drawable;
pub mod editor;
pub mod mark;
pub mod position;
pub mod renderer;

use app::App;

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
    let app = App::init(&sdl_context)?;

    app.run()?;

    Ok(())
}
