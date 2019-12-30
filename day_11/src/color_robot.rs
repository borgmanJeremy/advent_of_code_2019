use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
//use std::fs::File;
//use std::io::prelude::*;
//use std::io::BufReader;
//use std::path::Path;

#[derive(Debug, Clone)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug)]
pub enum QueryType {
    ColorQuery,
    PrintMap,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
pub enum Turn {
    Right,
    Left,
}

#[derive(Debug, Clone)]
pub struct MoveCommand {
    pub dir: Turn,
    pub color: Color,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: i128,
    pub y: i128,
}

pub struct HullInfo {
    pub pos: Point,
    pub color: Color,
}
pub fn print_path_to_terminal(hull_info: &Vec<HullInfo>){
    let mut min_x = hull_info[0].pos.x;
    let mut max_x = hull_info[0].pos.x;
    let mut min_y = hull_info[0].pos.y;
    let mut max_y = hull_info[0].pos.y;
    
    for elem in hull_info{
        if elem.pos.x < min_x{
            min_x = elem.pos.x;
        }
        if elem.pos.x > max_x{
            max_x = elem.pos.x;
        }
        if elem.pos.y < min_y{
            min_y = elem.pos.y;
        }
        if elem.pos.y > max_y{
            max_y = elem.pos.y;
        }
    }

    for x in min_x..=max_x{
        for y in min_y..=max_y{
            let color = match find_pos(&Point{x: x, y:y}, &hull_info){
                Some(idx)=> {hull_info[idx].color.clone()}
                None => {Color::Black}
            };
            match color {
                Color::Black => print!(" "),
                Color::White => print!("#")
            }
        }
        println!("");
    }

}
//pub fn write_path_to_ppm_file(path &str){
//
//}

pub fn command_robot(handle: &ColorRobotHandle, command: MoveCommand) {
    handle.tx_main_move_command.send(command).unwrap();
    // println!("{:?}", handle.rx_main_move_command.recv().unwrap().unwrap());
}

pub fn robot_color(handle: &ColorRobotHandle) -> Color {
    handle
        .tx_main_color_query
        .send(QueryType::ColorQuery)
        .unwrap();
    let msg = handle.rx_main_color_query.recv().unwrap();
    // println!("{:?}", msg);
    return msg;
}
pub fn find_pos(hull_query: &Point, hull_info: &Vec<HullInfo>) -> Option<usize> {
    for idx in 0..hull_info.len() {
        if hull_info[idx].pos == *hull_query {
            return Some(idx);
        }
    }
    None
}

pub fn move_robot(starting_pos: &Point, dir: &Direction) -> Point {
    match dir {
        Direction::Up => Point {
            x: starting_pos.x,
            y: starting_pos.y + 1,
        },
        Direction::Down => Point {
            x: starting_pos.x,
            y: starting_pos.y - 1,
        },
        Direction::Left => Point {
            x: starting_pos.x - 1,
            y: starting_pos.y,
        },
        Direction::Right => Point {
            x: starting_pos.x + 1,
            y: starting_pos.y,
        },
    }
}

pub fn rotate_robot(starting_dir: &Direction, command: Turn) -> Direction {
    match command {
        Turn::Left => match starting_dir {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        },
        Turn::Right => match starting_dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        },
    }
}

pub struct ColorRobotHandle {
    pub tx_main_color_query: Sender<QueryType>,
    pub rx_main_color_query: Receiver<Color>,
    pub tx_main_move_command: Sender<MoveCommand>,
    pub rx_main_move_command: Receiver<Result<Point, ()>>,
}

pub fn spawn_color_robot() -> ColorRobotHandle {
    let (tx_thread_color_query, rx_main_color_query): (Sender<Color>, Receiver<Color>) =
        mpsc::channel();
    let (tx_main_color_query, rx_thread_color_query): (Sender<QueryType>, Receiver<QueryType>) =
        mpsc::channel();
    let (tx_main_move_command, rx_thread_move_command): (
        Sender<MoveCommand>,
        Receiver<MoveCommand>,
    ) = mpsc::channel();
    let (tx_thread_move_command, rx_main_move_command): (
        Sender<Result<Point, ()>>,
        Receiver<Result<Point, ()>>,
    ) = mpsc::channel();

    let _child = thread::spawn(move || {
        // Init Camera position and hull color
        let mut hull_color = Vec::new();
        hull_color.push(HullInfo {
            pos: Point { x: 0, y: 0 },
            color: Color::White,
        });
        let mut robot_pos = Point { x: 0, y: 0 };
        let mut robot_dir = Direction::Up;

        loop {
            // Handle color query
            match rx_thread_color_query.try_recv() {
                Ok(hull_query) => {
                    let _rx_status = match hull_query {
                        QueryType::ColorQuery => match find_pos(&robot_pos, &hull_color) {
                            Some(idx) => tx_thread_color_query.send(hull_color[idx].color.clone()),
                            None => tx_thread_color_query.send(Color::White),
                        },
                        QueryType::PrintMap => {
                            print_path_to_terminal(&hull_color);
                            tx_thread_color_query.send(Color::White)
                        }
                    };
                }
                Err(_err) => {}
            }
            //Handle Move request
            match rx_thread_move_command.try_recv() {
                Ok(move_command) => {
                    // paint current position
                    let idx = find_pos(&robot_pos, &hull_color).unwrap();
                    hull_color[idx].color = move_command.color;
                    // move robot
                    robot_dir = rotate_robot(&robot_dir, move_command.dir);
                    robot_pos = move_robot(&robot_pos, &robot_dir);
                    match find_pos(&robot_pos, &hull_color) {
                        Some(_idx) => {}
                        None => hull_color.push(HullInfo {
                            pos: robot_pos.clone(),
                            color: Color::Black,
                        }),
                    }
                    tx_thread_move_command.send(Ok(robot_pos.clone())).unwrap();
 
        
                }
                Err(_err) => {
                    //tx_thread_move_command.send(Err(())).unwrap();
                }
            }
        }
    });
    ColorRobotHandle {
        rx_main_color_query: rx_main_color_query,
        tx_main_color_query: tx_main_color_query,
        rx_main_move_command: rx_main_move_command,
        tx_main_move_command: tx_main_move_command,
    }
}