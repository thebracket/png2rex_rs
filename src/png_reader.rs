use crate::args::Png2RexArgs;
use bracket_lib::prelude::XpColor;
use image::io::Reader as ImageReader;
use std::error::Error;

pub struct PixelBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<XpColor>,
}

pub fn read_png(args: &Png2RexArgs) -> Result<PixelBuffer, Box<dyn Error>> {
    let mut img = ImageReader::open(&args.input)?.decode()?;
    if args.flip_v {
        img = img.flipv();
    }
    if args.flip_h {
        img = img.fliph();
    }
    let rgb = img.to_rgb8();
    let width = rgb.width();
    let height = rgb.height();
    let raw = rgb.into_raw();
    let mut pixels = Vec::with_capacity(width as usize * height as usize);
    let len = raw.len() / 3;
    for i in 0..len {
        let j = i * 3;
        pixels.push(XpColor::new(raw[j], raw[j + 1], raw[j + 2]));
    }

    Ok(PixelBuffer {
        width,
        height,
        pixels,
    })
}

#[cfg(test)]
mod test {
    use crate::args::Png2RexArgs;

    use super::read_png;

    #[test]
    fn test_kitty() {
        let buf = read_png(&Png2RexArgs {
            input: "resources/kitty.png".to_string(),
            output: String::new(),
            flip_v: false,
            flip_h: false,
        })
        .unwrap();
        assert_eq!(buf.width, 32);
        assert_eq!(buf.height, 32);
    }
}
