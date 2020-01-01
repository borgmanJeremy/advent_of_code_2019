pub mod body {
    use num::{PrimInt, Signed};
    #[derive(Debug, Clone, PartialEq,Eq,Hash)]
    pub struct Point1D<T> {
        pub velocity: T,
        pub position: T,
    }

    #[derive(Debug, Clone)]
    pub struct Velocity<T> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    #[derive(Debug, Clone)]
    pub struct Position<T> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    #[derive(Debug, Clone)]
    pub struct Body<T> {
        pub pos: Position<T>,
        pub vel: Velocity<T>,
    }

    pub fn calc_velocity<T>(origin: T, subject: T) -> T
    where
        T: PrimInt + Signed,
    {
        if origin > subject {
            -T::one()
        } else if origin < subject {
            T::one()
        } else {
            T::zero()
        }
    }
    pub fn calc_pos<T>(pos: T, vel: T) -> T
    where
        T: PrimInt + Signed,
    {
        pos + vel
    }
    pub fn tick_pos<T>(body_list: &Vec<Body<T>>) -> Vec<Body<T>>
    where
        T: PartialOrd + Clone + PrimInt + Signed + std::ops::AddAssign,
    {
        let mut updated_list = body_list.clone();
        for body in 0..body_list.len() {
            updated_list[body].pos.x = calc_pos(body_list[body].pos.x, body_list[body].vel.x);
            updated_list[body].pos.y = calc_pos(body_list[body].pos.y, body_list[body].vel.y);
            updated_list[body].pos.z = calc_pos(body_list[body].pos.z, body_list[body].vel.z);
        }
        updated_list
    }

    pub fn tick_velocity<T>(body_list: &Vec<Body<T>>) -> Vec<Body<T>>
    where
        T: PartialOrd + Clone + PrimInt + Signed + std::ops::AddAssign,
    {
        let mut updated_list = body_list.clone();

        for origin_body in 0..body_list.len() {
            for subject_body in 0..body_list.len() {
                updated_list[origin_body].vel.x +=
                    calc_velocity(body_list[origin_body].pos.x, body_list[subject_body].pos.x);
                updated_list[origin_body].vel.y +=
                    calc_velocity(body_list[origin_body].pos.y, body_list[subject_body].pos.y);
                updated_list[origin_body].vel.z +=
                    calc_velocity(body_list[origin_body].pos.z, body_list[subject_body].pos.z);
            }
        }
        return updated_list;
    }

    pub fn potential_energy<T>(body: &Body<T>) -> T
    where
        T: PartialOrd + Clone + PrimInt + Signed + std::ops::AddAssign,
    {
        body.pos.x.abs() + body.pos.y.abs() + body.pos.z.abs()
    }

    pub fn kinetic_energy<T>(body: &Body<T>) -> T
    where
        T: PartialOrd + Clone + PrimInt + Signed + std::ops::AddAssign,
    {
        body.vel.x.abs() + body.vel.y.abs() + body.vel.z.abs()
    }

    pub fn total_energy<T>(body_list: &Vec<Body<T>>) -> T
    where
        T: PartialOrd + Clone + PrimInt + Signed + std::ops::AddAssign,
    {
        let mut energy = T::zero();
        for idx in 0..body_list.len() {
            energy += potential_energy(&body_list[idx]) * kinetic_energy(&body_list[idx]);
        }

        return energy;
    }
}
