use crate::*;


#[derive(Debug,Clone)]
pub struct ActMan {
    pub x: i32,
    pub y: i32,
    pub dir: char,
    pub alive: bool
}

pub fn spawn_act_man(game_inputs: &Inputs) -> ActMan {
    ActMan {
        x: game_inputs.a_pos.0,
        y: game_inputs.a_pos.1,
        dir: 'U',
        alive: true
    }
}
