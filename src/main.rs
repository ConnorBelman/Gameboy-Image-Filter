extern crate image;

use std::env;
use image::Pixel;
use image::GenericImageView;

const DARKEST_GREEN: [u8; 3] = [0x0F, 0x38, 0x0F];
const DARK_GREEN: [u8; 3] = [0x30, 0x62, 0x30];
const LIGHT_GREEN: [u8; 3] = [0x8B, 0xAC, 0x0F];
const LIGHTEST_GREEN: [u8; 3] = [0x9B, 0xBC, 0x0F];

fn pattern_even(x: u32, y: u32, color1: [u8; 3], color2: [u8; 3]) -> [u8; 3] {
	if (x % 2 == 0 && y % 2 == 0) || (x % 2 == 1 && y % 2 == 1) {
		color1
	}
	else {
		color2
	}
}

fn pattern_uneven(x: u32, y: u32, color1: [u8; 3], color2: [u8; 3]) -> [u8; 3] {
	if x % 4 == 0 {
		if y % 4 != 3 {
			color1
		}
		else {
			color2
		}
	}
	else if x % 4 == 1 || x % 4 == 3 {
		if y % 2 == 0 {
			color2
		}
		else {
			color1
		}
	}
	else {
		if y % 4 != 1 {
			color1
		}
		else {
			color2
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let input_file = &args[1];
	let output_file = &args[2]; 
	let img = image::open(input_file).unwrap();
	let mut output = img.to_rgb();
	for (x, y, pixel) in img.pixels() {
		let brightness = pixel.to_luma();
		let shade: [u8; 3] = match brightness[0] / 21 {
			0 => DARKEST_GREEN,
			1 => pattern_uneven(x, y, DARKEST_GREEN, DARK_GREEN),
			2 => pattern_even(x, y, DARKEST_GREEN, DARK_GREEN),
			3 => pattern_uneven(x, y, DARK_GREEN, DARKEST_GREEN),
			4 => DARK_GREEN,
			5 => pattern_uneven(x, y, DARK_GREEN, LIGHT_GREEN),
			6 => pattern_even(x, y, DARK_GREEN, LIGHT_GREEN),
			7 => pattern_uneven(x, y, LIGHT_GREEN, DARK_GREEN),
			8 => LIGHT_GREEN,
			9 => pattern_uneven(x, y, LIGHT_GREEN, LIGHTEST_GREEN),
			10 => pattern_even(x, y, LIGHT_GREEN, LIGHTEST_GREEN),
			11 => pattern_uneven(x, y, LIGHTEST_GREEN, LIGHT_GREEN),
			12 => LIGHTEST_GREEN,
			_ => [0, 0, 0]
		};
		output.put_pixel(x, y, image::Rgb(shade));
	}
    output.save(output_file).unwrap();
    println!("Done! Image saved to {}", output_file);
}
