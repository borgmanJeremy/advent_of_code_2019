use day_13::intcode::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::error::Error;
use std::io;
use std::time::Duration;

// Not doing the proper error handling here
fn load_instructions(path: &str) -> Result<Vec<i128>, Box<dyn Error>> {
    let mut ret_vec = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        for elem in record.iter() {
            ret_vec.push(elem.parse().unwrap());
        }
    }
    Ok(ret_vec)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SpriteType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: i128,
    pub y: i128,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Sprite {
    pub pos: Point,
    pub id: SpriteType,
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            pos: Point { x: 0, y: 0 },
            id: SpriteType::Empty,
        }
    }
}

fn find_index(sprite_list: &Vec<Sprite>, point: &Point) -> Option<usize> {
    for sprite in 0..sprite_list.len() {
        if sprite_list[sprite].pos == *point {
            return Some(sprite);
        }
    }
    return None;
}

fn find_ball(sprite_list: &Vec<Sprite>) -> Option<usize> {
    for sprite in 0..sprite_list.len() {
        if sprite_list[sprite].id == SpriteType::Ball {
            return Some(sprite);
        }
    }
    return None;
}

fn find_paddle(sprite_list: &Vec<Sprite>) -> Option<usize> {
    for sprite in 0..sprite_list.len() {
        if sprite_list[sprite].id == SpriteType::Paddle {
            return Some(sprite);
        }
    }
    return None;
}

fn update_sprite(sprite_list: &mut Vec<Sprite>, new_sprite: &Sprite) {
    let mut inserted = false;
    for sprite in 0..sprite_list.len() {
        if sprite_list[sprite].pos == new_sprite.pos {
            sprite_list[sprite].id = new_sprite.id.clone();
            inserted = true;
            break;
        }
    }

    if inserted == false {
        sprite_list.push(new_sprite.clone());
    }
}

fn draw_sprites(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    sprite_list: &Vec<Sprite>,
    scale: u32,
) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    for sprite in 0..sprite_list.len() {
        if sprite_list[sprite].id != SpriteType::Empty {
            canvas
                .draw_rect(Rect::new(
                    (sprite_list[sprite].pos.x as i32) * scale as i32 - scale as i32 / 2,
                    (sprite_list[sprite].pos.y as i32) * scale as i32 - scale as i32 / 2,
                    scale,
                    scale,
                ))
                .unwrap();
        }
    }
    canvas.present();
}
fn main() {
    let memory_initial_state =
        load_instructions("/home/jeremy/advent_of_code/day_13/input.txt").unwrap();
    let mut cpu = Cpu::new();
    cpu.input_mode = day_13::intcode::IOMode::STACK;
    cpu.output_mode = day_13::intcode::IOMode::STACK;
    cpu.memory.resize(10 * 1024 * 1024, 0);

    for idx in 0..memory_initial_state.len() {
        cpu.memory[idx] = memory_initial_state[idx];
    }
    let mut sprite_vec = Vec::new();
    //loop {
    //    cpu.exec();
    //    if cpu.output_stack.len() == 3 {
    //        let mut new_sprite = Sprite::new();
    //        let sprite_id = match cpu.output_stack[2] {
    //            0 => SpriteType::Empty,
    //            1 => SpriteType::Wall,
    //            2 => SpriteType::Block,
    //            3 => SpriteType::Paddle,
    //            4 => SpriteType::Ball,
    //            _ => panic!("Invalid sprite type"),
    //        };
    //        new_sprite.id = sprite_id;
    //        new_sprite.pos.y = cpu.output_stack[1];
    //        new_sprite.pos.x = cpu.output_stack[0];
    //        update_sprite(&mut sprite_vec, &new_sprite);
    //        cpu.output_stack.clear();
    //    }
    //    match cpu.state {
    //        CpuState::PendingInput => {
    //            print!("waiting for input");
    //            break;
    //        }
    //        CpuState::Halt => {
    //            println!("program ended");
    //            break;
    //        }
    //        CpuState::Run => {}
    //    }
    //}
    //let mut min_x = sprite_vec[0].pos.x;
    //let mut max_x = sprite_vec[0].pos.x;
    //let mut min_y = sprite_vec[0].pos.y;
    //let mut max_y = sprite_vec[0].pos.y;

    //for idx in 0..sprite_vec.len() {
    //    if sprite_vec[idx].pos.x < min_x {
    //        min_x = sprite_vec[idx].pos.x;
    //    }
    //    if sprite_vec[idx].pos.x > max_x {
    //        max_x = sprite_vec[idx].pos.x;
    //    }
    //    if sprite_vec[idx].pos.y < min_y {
    //        min_y = sprite_vec[idx].pos.y;
    //    }
    //    if sprite_vec[idx].pos.y > max_y {
    //        max_y = sprite_vec[idx].pos.y;
    //    }
    //}

    //println!("min x: {} max x: {}", min_x, max_x);
    //println!("min y: {} max y: {}", min_y, max_y);

    //for y in min_y..=max_y {
    //    for x in min_x..=max_x {
    //        match find_index(&sprite_vec, &Point { x: x, y: y }) {
    //            Some(idx) => match &sprite_vec[idx].id {
    //                SpriteType::Empty => print!(" "),
    //                SpriteType::Wall => print!("|"),
    //                SpriteType::Block => print!("U"),
    //                SpriteType::Paddle => print!("_"),
    //                SpriteType::Ball => print!("o"),
    //            },
    //            None => panic!("index not found"),
    //        }
    //    }
    //    println!("");
    //}
    // Part 2
    let width = 45;
    let height = 26;
    let scale = 10;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo", width * scale, height * scale)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut input = String::new();
    cpu.memory[0] = 2;
    loop {
        cpu.exec();
        if cpu.output_stack.len() == 3 {
            if cpu.output_stack[0] == -1 {
                println!("Score: {}", cpu.output_stack[2]);
            } else {
                let mut new_sprite = Sprite::new();
                let sprite_id = match cpu.output_stack[2] {
                    0 => SpriteType::Empty,
                    1 => SpriteType::Wall,
                    2 => SpriteType::Block,
                    3 => SpriteType::Paddle,
                    4 => SpriteType::Ball,
                    _ => panic!("Invalid sprite type"),
                };
                new_sprite.id = sprite_id;
                new_sprite.pos.y = cpu.output_stack[1];
                new_sprite.pos.x = cpu.output_stack[0];
                update_sprite(&mut sprite_vec, &new_sprite);
            }

            cpu.output_stack.clear();
            draw_sprites(&mut canvas, &sprite_vec, scale);
        }
        match cpu.state {
            CpuState::PendingInput => {
                //print!("waiting for input");
                let ball_pos = sprite_vec[find_ball(&sprite_vec).unwrap()].pos.x;
                let paddle_pos = sprite_vec[find_paddle(&sprite_vec).unwrap()].pos.x;
                if paddle_pos < ball_pos {
                    cpu.input_stack.push(1);
                } else if paddle_pos > ball_pos {
                    cpu.input_stack.push(-1)
                } else {
                    cpu.input_stack.push(0)
                }

                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
            CpuState::Halt => {
                println!("program ended");
                break;
            }
            CpuState::Run => {}
        }
    }
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Couldn't read line");
}
