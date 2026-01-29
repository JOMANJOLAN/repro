use std::env;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io::BufWriter;
use std::io::Write as IoWrite;

pub fn generate() {
    let scale = [16, 9];
    let fps = 60;
    let cell = fps;
    let w = scale[0] * cell;
    let h = scale[1] * cell;
    let rgb_max = 0xFF;
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_dir = root + "/output/board";
    if !fs::exists(&output_dir).unwrap() {
        fs::create_dir_all(&output_dir).unwrap();
    }
    let mut buf = String::new();
    let header = format!("P6\n{} {}\n{}\n", w, h, rgb_max);
    for i in 0..fps {
        FmtWrite::write_fmt(&mut buf, format_args!("{}/frame-{:02}.ppm", output_dir, i)).unwrap();
        let output_path = &buf;
        let file = fs::File::create(output_path).unwrap();
        let mut file = BufWriter::new(file);
        IoWrite::write_all(&mut file, header.as_bytes()).unwrap();
        for y in 0..h {
            for x in 0..w {
                let mut buf = [0x00, 0x00, 0x00];
                if ((x + i) / cell + (y + i) / cell) % 2 == 1 {
                    buf[0] = 0xFF;
                }
                IoWrite::write_all(&mut file, &buf).unwrap();
            }
        }
        println!("Generated {}", output_path);
        buf.clear();
    }
}
