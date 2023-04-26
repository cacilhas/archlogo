extern crate anyhow;
extern crate eframe;
extern crate image;

use eframe::egui;
use image::Pixel;
use std::{error, io};

#[cfg(target_os = "linux")]
fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 100.0)),
        centered: true,
        resizable: false,
        run_and_return: false,
        ..Default::default()
    };
    let font = include_bytes!("resources/firacode.ttf");
    let font = egui::FontData::from_static(font);
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("mono".into(), font);
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "mono".into());
    eframe::run_native(
        "About System",
        options,
        Box::new(|cc| {
            let visuals = egui::Visuals {
                override_text_color: Some(egui::Color32::BLACK),
                panel_fill: egui::Color32::LIGHT_GRAY,
                ..Default::default()
            };
            cc.egui_ctx.set_visuals(visuals);
            cc.egui_ctx.set_fonts(fonts);
            Box::new(App::default())
        }),
    )
    .unwrap();
}

struct App {
    logo: egui::ColorImage,
    uname: String,
    texture: Option<egui::TextureHandle>,
    width: f32,
    height: f32,
}

impl App {
    pub fn pressed_keys(ctx: &egui::Context) -> PressKeys {
        let mut escape = false;
        let events = ctx.input(|input| input.events.to_owned());
        for event in events.iter() {
            match event {
                egui::Event::Key {
                    key,
                    pressed,
                    repeat: _,
                    modifiers: _,
                } if *key == egui::Key::Escape => escape = *pressed,
                _ => (),
            }
        }
        PressKeys { escape }
    }
}

impl Default for App {
    fn default() -> Self {
        let logo = get_logo().unwrap();
        let size = [logo.width() as usize, logo.height() as usize];
        let mut pixels = Vec::with_capacity(size[0] * size[1]);
        for y in 0..size[1] {
            for x in 0..size[0] {
                let pixel = logo.get_pixel(x as u32, y as u32);
                let pixel = pixel.to_rgba().0;
                pixels.push(egui::Color32::from_rgba_premultiplied(
                    pixel[0], pixel[1], pixel[2], pixel[3],
                ));
            }
        }

        Self {
            logo: egui::ColorImage { size, pixels },
            uname: get_uname().unwrap(),
            texture: Default::default(),
            width: size[0] as f32,
            height: size[1] as f32 + 36.0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        frame.set_centered();
        frame.set_always_on_top(true);
        frame.set_window_size(egui::vec2(self.width, self.height));
        if Self::pressed_keys(ctx).escape {
            return frame.close();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let texture = self.texture.get_or_insert_with(|| {
                ui.ctx().load_texture(
                    "logo",
                    egui::ImageData::Color(self.logo.to_owned()),
                    Default::default(),
                )
            });
            let size = texture.size_vec2();
            ui.image(texture, size);
            ui.centered_and_justified(|ui| ui.strong(&self.uname));
        });
    }
}

struct PressKeys {
    escape: bool,
}

fn get_logo() -> anyhow::Result<image::RgbaImage> {
    let logo = include_bytes!("resources/logo.png");

    let img = image::load_from_memory_with_format(logo, image::ImageFormat::Png)?.to_rgba8();

    let (width, height) = img.dimensions();
    let res = image::RgbaImage::from_raw(width, height, img.to_vec())
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
    Ok(res)
}

fn get_uname() -> Result<String, Box<dyn error::Error>> {
    let info = uname::uname()?;
    Ok([
        info.sysname,
        info.nodename,
        info.release,
        info.version,
        info.machine,
    ]
    .join(" "))
}
