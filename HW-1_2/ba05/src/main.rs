pub mod pixels {
    pub fn parse_bitmap_8x8(lines: [&str; 8]) -> [u8; 8] {
        let mut result: [u8; 8] = [0; 8];
        for (line_index, line) in lines.iter().enumerate() {
            let mut acc: u32 = 0;

            for (index, symbol) in line.chars().rev().enumerate() {
                if symbol == '#' {
                    acc += 2u32.pow(index as u32)
                }
            }
            result[line_index] = acc as u8;
        }

        result
    }

    pub fn render_bitmap_8x8(bytes: [u8; 8]) -> [String; 8] {
        let mut result = [
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ];
        for (line_index, byte) in bytes.iter().enumerate() {
            let mut output_string = String::from("");
            for bit_index in 0..8 {
                let is_set = byte & (1 << bit_index) != 0;
                if is_set == false {
                    output_string = String::from(".") + &output_string;
                } else {
                    output_string = String::from("#") + &output_string;
                }
            }

            result[line_index] = output_string;
        }

        result
    }

    pub fn invert_bitmap_8x8(bytes: [u8; 8]) -> [u8; 8] {
        let mut result: [u8; 8] = [0; 8];
        for (byte_index, byte) in bytes.iter().enumerate() {

            let inverted = !byte;
            result[byte_index] = inverted;

        }

        result

    }
}

use pixels::{parse_bitmap_8x8, render_bitmap_8x8, invert_bitmap_8x8};

fn main() {
    println!("Hello, world!");

    let image = [
        "..####..", ".#....#.", "#.#..#.#", "#..##..#", "#......#", "#.#..#.#", ".#....#.",
        "..####..",
    ];

    let bytes = parse_bitmap_8x8(image);

    println!("Bytes:");
    for byte in bytes {
        println!("{byte:08b} 0x{byte:02X}");
    }
    println!();
    println!("Rendered:");
    for line in render_bitmap_8x8(bytes) {
        println!("{line}");
    }

    println!();
    println!("Inverted:");
    for line in render_bitmap_8x8(invert_bitmap_8x8(bytes)) {
        println!("{line}");
    }

}
