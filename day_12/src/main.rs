use day_12::body::*;
use num::Integer;
use std::collections::HashSet;

fn calc_period(body_list_in: Vec<Point1D<i64>>) -> usize {
    let mut body_list = body_list_in.clone();
    let mut x_list = HashSet::new();
    x_list.insert(body_list.clone());

    loop {
        let mut updated_list = body_list.clone();
        for origin_body in 0..body_list.len() {
            for subject_body in 0..body_list.len() {
                updated_list[origin_body].velocity += calc_velocity(
                    body_list[origin_body].position,
                    body_list[subject_body].position,
                );
            }
        }

        for body in 0..body_list.len() {
            updated_list[body].position =
                calc_pos(body_list[body].position, updated_list[body].velocity);
        }
        if x_list.contains(&updated_list) {
            return x_list.len();
        }
        body_list = updated_list.clone();
        x_list.insert(updated_list);
    }
}

fn main() {
    let mut moons = Vec::<Body<i64>>::new();
    moons.push(Body {
        vel: Velocity { x: 0, y: 0, z: 0 },
        pos: Position { x: 17, y: -9, z: 4 },
    });
    moons.push(Body {
        vel: Velocity { x: 0, y: 0, z: 0 },
        pos: Position {
            x: 2,
            y: 2,
            z: -13,
        },
    });
    moons.push(Body {
        vel: Velocity { x: 0, y: 0, z: 0 },
        pos: Position { x: -1, y: 5, z: -1 },
    });
    moons.push(Body {
        vel: Velocity { x: 0, y: 0, z: 0 },
        pos: Position { x: 4, y: 7, z: -7 },
    });

/*    moons.push(Body {
        vel: Velocity { x: 0, y: 0, z: 0 },
        pos: Position { x: -8, y: -10, z: 0 },
    });
    moons.push(Body {
        vel: Velocity { x: 0, y: 0, z: 0 },
        pos: Position {
            x: 5,
            y: 5,
            z: 10,
        },
    });
    moons.push(Body {
        vel: Velocity { x: 0, y: 0, z: 0 },
        pos: Position { x: 2, y: -7, z: 3 },
    });
    moons.push(Body {
        vel: Velocity { x: 0, y: 0, z: 0 },
        pos: Position { x: 9, y: -8, z: -3 },
    });
*/

    // Part 1
    for _count in 0..1000 {
        moons = tick_velocity(&moons);
        moons = tick_pos(&moons);
    }
    //println!("{:?}", moons);
    println!("{}", total_energy(&moons));

    // Part 2
    let mut body_list_x = Vec::<Point1D<i64>>::new();
    for body in &moons {
        body_list_x.push(Point1D {
            velocity: body.vel.x,
            position: body.pos.x,
        });
    }
   
    let mut body_list_y = Vec::<Point1D<i64>>::new();
    for body in &moons {
        body_list_y.push(Point1D {
            velocity: body.vel.y,
            position: body.pos.y,
        });
    }
  
    let mut body_list_z = Vec::<Point1D<i64>>::new();
    for body in &moons {
        body_list_z.push(Point1D {
            velocity: body.vel.z,
            position: body.pos.z,
        });
    }
    let x_period = calc_period(body_list_x);
    println!("x period: {}", x_period);
    
    let y_period = calc_period(body_list_y);
    println!("y period: {}", y_period);
    
    let z_period = calc_period(body_list_z);
    println!("z period: {}", z_period);
    
    println!("total period: {}", x_period.lcm(&y_period).lcm(&z_period));
}
