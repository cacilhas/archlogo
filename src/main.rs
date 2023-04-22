use std::error;

use fltk::{prelude::*, *};
use ::image::{load_from_memory_with_format};



#[cfg(target_os = "linux")]
fn main() {
    let screen_size = app::screen_size();
    let mut logo = get_logo().unwrap();
    let width = logo.width();
    let height = logo.height() + 30;

    let winx = ((screen_size.0 as i32)-width) / 2;
    let winy = ((screen_size.1 as i32)-height) / 2;

    let app = app::App::default();
    let mut win= window::Window::new(
        winx, winy, width, height, "About System",
    );

    let mut frame = frame::Frame::default()
        .with_size(logo.width(), logo.height());
    frame.draw(move |f| {
        logo.draw(f.x(), f.y(), logo.width(), logo.height());
    });
    let info_msg = get_uname().unwrap();
    let mut info = frame::Frame::default()
        .with_pos(0, frame.y() + frame.height())
        .with_size(width, 30)
        .with_label(info_msg.as_str());
    info.set_label_size(12);

    win.end();
    win.show();

    app.run().unwrap();
}


fn get_logo() -> Result<image::RgbImage, Box<dyn error::Error>> {
    let logo = include_bytes!("resources/logo.png");

    let img = load_from_memory_with_format(
        logo,
        ::image::ImageFormat::Png,
    )?;

    let imgw = img.width() as i32;
    let imgh = img.height() as i32;
    let rawimg = img.as_bytes();

    let res = image::RgbImage::new(rawimg, imgw, imgh, enums::ColorDepth::Rgba8)?;
    Ok(res)
}

fn get_uname() -> Result<String, Box<dyn error::Error>> {
    let info = uname::uname()?;
    Ok(
        [
            info.sysname,
            info.nodename,
            info.release,
            info.version,
            info.machine,
        ]
        .join(" ")
    )
}
