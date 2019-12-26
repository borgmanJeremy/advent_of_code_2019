use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn write_to_ppm(path: &str, width: usize, height: usize, image: &Vec<u8>) {
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't open  file: {}", why.description()),
        Ok(file) => file,
    };

    let ppm_header = format!("P3\n{} {}\n255\n", width, height);
    match file.write(ppm_header.as_bytes()) {
        Err(why) => panic!("Could not write to file because: {}", why.description()),
        Ok(_) => {}
    }

    let mut image_str = String::from("");
    for pixel in 0..image.len() {
        match image[pixel] {
            0 => {
                image_str = format!("{} 0 0 0\n", image_str);
            }
            1 => {
                image_str = format!("{} 255 255 255\n", image_str);
            }
            2 => {
                image_str = format!("{} 255 0 0\n", image_str);
            }
            _ => panic!("value not ok"),
        }
    }
    match file.write(image_str.as_bytes()) {
        Err(why) => panic!("Could not write to file because: {}", why.description()),
        Ok(_) => {}
    }
}

fn squash_layers(input_layers: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut combined = input_layers[0].clone();

    for layer in input_layers {
        for pixel in 0..layer.len() {
            if combined[pixel] == 2 {
                combined[pixel] = layer[pixel];
            }
        }
    }
    return combined;
}

fn main() {
    let path = Path::new("/home/jeremy/advent_of_code/day_8/input.txt");
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open  file: {}", why.description()),
        Ok(file) => file,
    };

    let buff = BufReader::new(file);

    let mut char_array = Vec::new();
    for line in buff.lines() {
        let unwrap = line.unwrap();
        char_array = unwrap.as_bytes().to_vec();
    }

    // convert to num
    for i in 0..char_array.len() {
        char_array[i] = char_array[i] - 0x30;
    }

    let image_width: usize = 25;
    let image_height: usize = 6;
    let image_size = image_width * image_height;

    assert_eq!(char_array.len() % (image_width * image_height), 0);
    let mut layer_list = Vec::new();
    let offset = char_array.len() / image_size;
    for idx in 0..offset {
        let mut temp_layer = Vec::new();
        for pixel in 0..image_size {
            temp_layer.push(char_array[pixel + (idx * image_size)]);
        }
        layer_list.push(temp_layer.clone());
    }

    let mut min_0 = 999999999;
    let mut layer_idx = 0;
    for layer in 0..layer_list.len() {
        let mut count = 0;
        for pixel in &layer_list[layer] {
            if *pixel == 0 {
                count += 1;
            }
        }
        if count < min_0 {
            min_0 = count;
            layer_idx = layer;
        }
    }

    let mut count_1 = 0;
    let mut count_2 = 0;
    for pixel in &layer_list[layer_idx] {
        if *pixel == 1 {
            count_1 += 1;
        } else if *pixel == 2 {
            count_2 += 1;
        }
    }

    println!("Min Zero on layer: {}", layer_idx);
    println!(
        "Num 1: {} Num 2: {} Num 1*2: {}",
        count_1,
        count_2,
        count_1 * count_2
    );

    let squashed_image = squash_layers(&layer_list);
    write_to_ppm(
        "/home/jeremy/advent_of_code/day_8/output.ppm",
        image_width,
        image_height,
        &squashed_image,
    );
}
