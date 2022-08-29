use std::fs::File;
use std::io::Write;
use clap::Parser;

/// Packs a PNG image into Xbox Packed Resource (XPR) format for loading on Xbox
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CLIArgs {
    /// Input image
    #[clap(short, long, value_parser)]
    input: String,

    /// Output file
    #[clap(short, long, value_parser)]
    output: String,
}

fn main() {
    let args = CLIArgs::parse();

    let input_file = match File::open(args.input) {
        Ok(file) => file,
        Err(e) => {
            println!("Could not open input file: {}", e.to_string());
            return;
        }
    };

    let decoder = png::Decoder::new(input_file);

    let mut reader = match decoder.read_info() {
        Ok(info_reader) => info_reader,
        Err(e) => {
            println!("Could not decode PNG: {}", e.to_string());
            return;
        }
    };

    let mut img_buf = vec![0; reader.output_buffer_size()];

    let img_info = match reader.next_frame(&mut img_buf) {
        Ok(out_info) => out_info,
        Err(e) => {
            println!("Could not decode PNG pixel data: {}", e.to_string());
            return;
        }
    };

    assert_eq!(img_info.width, 64, "Image width not 64px, found {}px", img_info.width);
    assert_eq!(img_info.height, 64, "Image height not 64px, found {}px", img_info.height);
    assert_eq!(img_info.color_type, png::ColorType::Rgba, "PNG image not in RGBA format");

    let mut out_img_buf = vec![0; reader.output_buffer_size()];

    xpr_swizzle::swizzle_box(&mut img_buf, img_info.width, img_info.height, 1, &mut out_img_buf, img_info.width);

    let header = match xpr_swizzle::create_header(&out_img_buf) {
        Ok(header) => header,
        Err(e) => {
            println!("Error creating header: {}", e.to_string());
            return;
        }
    };


    let mut out_file = match File::create(args.output.clone()) {
        Ok(file) => file,
        Err(e) => {
            println!("Could not open output file for writing: {}", e.to_string());
            return;
        }
    };

    out_file.write(&header).unwrap();
    out_file.write(&out_img_buf).unwrap();

    println!("Created new file {}!", args.output);
}
