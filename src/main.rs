fn check_cell(x: f64, y: f64) -> u8 {
    let c = num_complex::Complex::new(x, y);
    let mut z = c;
    for i in 0..255 {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    255
}

fn error_and_abort(s: &str) -> ! {
    eprintln!("{}", s);
    std::process::exit(1);
}

fn main() {
    use palette::Shade;
    use palette::Pixel;
    use std::io::Write;

    let divs: u32 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| error_and_abort("Divs required"))
        .parse()
        .unwrap_or_else(|_| error_and_abort("Failed to parse divs"));
    use rayon::prelude::*;
    let delta = 4.0 / divs as f64;
    let pixels: Vec<_> = (0..(3 * divs / 4))
        .into_par_iter()
        .map(|i| {
            (0..divs).into_par_iter().map(move |j| {
                let y = i as f64 * delta - 1.5;
                let x = j as f64 * delta - 2.5;
                255 - check_cell(x, y)
            })
        })
        .flatten()
        .collect();

    let file = std::fs::File::create("mandelbrot.png").unwrap();
    let w = std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, divs as u32, 3 * divs / 4 as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap().into_stream_writer();

    for v in pixels {
        let c: [u8; 3] = palette::LinSrgb::new(255 as u8, 0, 0).into_format().lighten(v as f64).into_format().into_raw();
        writer.write_all(&c).unwrap();
    }
}
