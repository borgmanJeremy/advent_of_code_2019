use num::Integer;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Fraction {
    pub num: i128,
    pub den: i128,
}

#[derive(Debug)]
pub struct Point {
    pub x: i128,
    pub y: i128,
}

pub fn calculate_angle(origin: &Point, dst: &Point) -> Fraction {
    Fraction {
        num: dst.y - origin.y,
        den: dst.x - origin.x,
    }
}

pub fn calc_lcm(input: &Vec<Fraction>) -> i128 {
    let mut lcm = 1;
    for i in 0..input.len() {
        if input[i].den != 0 {
            lcm = lcm.lcm(&input[i].den);
        }
    }
    return lcm;
}

pub fn normalize_angles_to_lcm(input: &mut Vec<Fraction>) {
    let lcm = calc_lcm(&input);
    for i in 0..input.len() {
        if input[i].den != 0 {
            let multiple = (lcm / input[i].den).abs();
            input[i].num *= multiple;
            input[i].den *= multiple;
        } else {
            if input[i].num > 0 {
                input[i].num = 1;
            } else if input[i].num < 0 {
                input[i].num = -1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tet() {
        let mut map = Vec::new();
        for i in 0..5 {
            for j in 0..5 {
                map.push(Point { x: i, y: j });
            }
        }
        let origin = Point { x: 2, y: 2 };
        let mut angle_list = Vec::new();
        for point in &map {
            let angle = calculate_angle(&origin, point);
            angle_list.push(angle);
        }
        normalize_angles_to_lcm(&mut angle_list);
        let set: HashSet<Fraction> = angle_list.into_iter().collect();
        assert_eq!(set.len(), 10);
    }
}
