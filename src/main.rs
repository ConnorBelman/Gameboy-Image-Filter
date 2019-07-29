#[macro_use]
extern crate clap;
extern crate image;

use clap::{Arg, App};

mod filter;

fn main() {
	let matches = App::new("gb_filter")
        .version(crate_version!())
        .author("Connor Belman")
        .about("Image filter for the Game Boy color palette")
        .arg(Arg::with_name("input_file")
                .required(true)
                .index(1)
                .help("input file to be filtered"))
        .arg(Arg::with_name("output_file")
        	.required(true)
        	.index(2)
        	.help("path to output file"))
        .arg(Arg::with_name("dithering")
        	.short("d")
        	.long("dither")
        	.takes_value(true)
        	.help("dithering level: none, low, mid, high (default: mid)"))
        .get_matches();
 
	let output = match matches.value_of("dithering").unwrap_or("mid") {
		"none" => filter::filter_none(matches.value_of("input_file").unwrap().to_string()),
		"low" => filter::filter_low(matches.value_of("input_file").unwrap().to_string()),
		"mid" => filter::filter_mid(matches.value_of("input_file").unwrap().to_string()),
		"high" => filter::filter_high(matches.value_of("input_file").unwrap().to_string()),
		_ => {
			println!("invalid dithering level");
			let img = image::open(matches.value_of("input_file").unwrap()).unwrap();
			img.to_rgb()
		}
	};
    output.save(matches.value_of("output_file").unwrap()).unwrap();
    println!("Done! Image saved to {}", matches.value_of("output_file").unwrap());
}
