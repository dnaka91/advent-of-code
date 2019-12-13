//! # Day 8: Space Image Format
//!
//! The Elves' spirits are lifted when they realize you have an opportunity to reboot one of their
//! Mars rovers, and so they are curious if you would spend a brief sojourn on Mars. You land your
//! ship near the rover.
//!
//! When you reach the rover, you discover that it's already in the process of rebooting! It's just
//! waiting for someone to enter a [BIOS] password. The Elf responsible for the rover takes a
//! picture of the password (your puzzle input) and sends it to you via the Digital Sending Network.
//!
//! Unfortunately, images sent via the Digital Sending Network aren't encoded with any normal
//! encoding; instead, they're encoded in a special Space Image Format. None of the Elves seem to
//! remember why this is the case. They send you the instructions to decode it.
//!
//! Images are sent as a series of digits that each represent the color of a single pixel. The
//! digits fill each row of the image left-to-right, then move downward to the next row, filling
//! rows top-to-bottom until every pixel of the image is filled.
//!
//! Each image actually consists of a series of identically-sized **layers** that are filled in this
//! way. So, the first digit corresponds to the top-left pixel of the first layer, the second digit
//! corresponds to the pixel to the right of that on the same layer, and so on until the last digit,
//! which corresponds to the bottom-right pixel of the last layer.
//!
//! For example, given an image `3` pixels wide and `2` pixels tall, the image data `123456789012`
//! corresponds to the following image layers:
//!
//! ```txt
//! Layer 1: 123
//!          456
//!
//! Layer 2: 789
//!          012
//! ```
//!
//! The image you received is **`25` pixels wide and `6` pixels tall**.
//!
//! To make sure the image wasn't corrupted during transmission, the Elves would like you to find
//! the layer that contains the **fewest `0` digits**. On that layer, what is **the number of `1`
//! digits multiplied by the number of `2` digits?**
//!
//! [BIOS]: https://en.wikipedia.org/wiki/BIOS
//!
//! ## Part Two
//!
//! Now you're ready to decode the image. The image is rendered by stacking the layers and aligning
//! the pixels with the same positions in each layer. The digits indicate the color of the
//! corresponding pixel: `0` is black, `1` is white, and `2` is transparent.
//!
//! The layers are rendered with the first layer in front and the last layer in back. So, if a given
//! position has a transparent pixel in the first and second layers, a black pixel in the third
//! layer, and a white pixel in the fourth layer, the final image would have a **black** pixel at
//! that position.
//!
//! For example, given an image `2` pixels wide and `2` pixels tall, the image data
//! `0222112222120000` corresponds to the following image layers:
//!
//! ```txt
//! Layer 1: 02
//!          22
//!
//! Layer 2: 11
//!          22
//!
//! Layer 3: 22
//!          12
//!
//! Layer 4: 00
//!          00
//! ```
//!
//! Then, the full image can be found by determining the top visible pixel in each position:
//!
//! - The top-left pixel is **black** because the top layer is `0`.
//! - The top-right pixel is **white** because the top layer is `2` (transparent), but the second
//!   layer is `1`.
//! - The bottom-left pixel is **white** because the top two layers are `2`, but the third layer is
//!   `1`.
//! - The bottom-right pixel is **black** because the only visible pixel in that position is `0`
//!   (from layer 4).
//!
//! So, the final image looks like this:
//!
//! ```txt
//! 01
//! 10
//! ```
//!
//! **What message is produced after decoding your image?**

use anyhow::Result;

pub const INPUT: &str = include_str!("d08.txt");

const ROW_SIZE: usize = 25;
const LAYER_SIZE: usize = ROW_SIZE * 6;

pub fn solve_part_one(input: &str) -> Result<u16> {
    let input = input.lines().next().unwrap();
    let mut pos = 0;
    let mut min_layer = "";
    let mut min_count = usize::max_value();

    while pos < input.len() {
        let layer = &input[pos..pos + LAYER_SIZE];
        let count = layer.bytes().filter(|b| *b == b'0').count();

        if count < min_count {
            min_layer = layer;
            min_count = count;
        }

        pos += LAYER_SIZE;
    }

    let mut count1 = 0;
    let mut count2 = 0;

    for byte in min_layer.bytes() {
        match byte {
            b'1' => count1 += 1,
            b'2' => count2 += 1,
            _ => (),
        }
    }

    // Highest possible value is (150 / 2) ^ 2 = 5625, so u16 is enough.
    Ok(count1 * count2)
}

const WHITE: char = '#';
const BLACK: char = '.';

pub fn solve_part_two(input: &str) -> Result<String> {
    let input = input.lines().next().unwrap();
    let mut pos = input.len();
    let mut image = [' '; LAYER_SIZE];

    while pos > 0 {
        let layer = &input[pos - LAYER_SIZE..pos];

        for (i, b) in layer.bytes().enumerate() {
            image[i] = match b {
                b'0' => BLACK,
                b'1' => WHITE,
                _ => image[i],
            }
        }

        pos -= LAYER_SIZE;
    }

    let mut image = image.iter().collect::<String>();
    for i in (0..image.len()).rev() {
        if i % ROW_SIZE == 0 {
            image.insert(i, '\n');
        }
    }

    Ok(image)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {}

    #[test]
    fn part_two() {}
}
