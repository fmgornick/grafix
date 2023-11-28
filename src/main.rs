use grafix::utils::color::Color;
use std::fs::File;
use std::io::prelude::*;

const HEIGHT: u32 = 256;
const WIDTH: u32 = 256;

fn main() -> std::io::Result<()> {
    let mut f = File::options()
        .append(true)
        .create(true)
        .open("image.ppm")?;
    writeln!(&mut f, "P3")?;
    writeln!(&mut f, "{} {}", HEIGHT, WIDTH)?;
    writeln!(&mut f, "255")?;

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            Color::from_u32(i, j, 0).write(&mut f)?;
        }
    }

    Ok(())
}
