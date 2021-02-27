use crate::{args::Png2RexArgs, png_reader::PixelBuffer};
use bracket_lib::prelude::XpFile;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn make_rex(params: &Png2RexArgs, buf: &PixelBuffer) -> Result<(), Box<dyn Error>> {
    let w = buf.width as usize;
    let h = buf.height as usize;
    let mut rex = XpFile::new(w, h);
    buf.pixels.iter().enumerate().for_each(|(i, p)| {
        let cell = rex.layers[0].get_mut(i % w, i / w).unwrap();
        cell.ch = 219;
        cell.fg = *p;
        cell.bg = *p;
    });

    let mut outfile = File::create(Path::new(&params.output))?;
    rex.write(&mut outfile)?;

    Ok(())
}
