use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;

mod arg_parser;
pub use arg_parser::*;
mod act_man;
pub use act_man::*;
mod ghost;
pub use ghost::*;
mod gamestate;
pub use gamestate::*;

//Box
mod id_dfs;
pub use id_dfs::*;
mod gbfs;
pub use gbfs::*;
mod astar;
pub use astar::*;

const GOAL_SCORE: usize = 24;

fn main() {
    //PARSE ARGS
    let mut game_inputs = arg_parse();
    //POINTERS AND STUFF TO PRINT
    //MAIN GAME LOOP
    //CREATE ACTORS
    let act_man = spawn_act_man(&game_inputs);
    let punky = spawn_punky(&game_inputs);
    let bunky = spawn_bunky(&game_inputs);
    let dunky = spawn_dunky(&game_inputs);
    let runky = spawn_runky(&game_inputs);

    /*INIT GAMEBOARD*/
    let mut game_board = create_board(&mut game_inputs,act_man,vec![punky,bunky,dunky,runky]);
    let result = act_man_astar_gs(game_board.clone());
    //let result = "UUURRRDDUULLLLLLDDDRRRRR".chars().collect();
    println!("{:?}",result);
    let mut final_board = transition_function(&mut game_board,&result);
    print_map(game_inputs.width, game_inputs.height, &mut final_board.map);
    println!("SCORE: {}",final_board.score);
    let args: Vec<String> = env::args().collect();
    let mut out = File::create(args[2].clone()).expect("FAILED TO CREATE");
    let moves: String = result.into_iter().collect();
    let map = write_map(game_inputs.width,game_inputs.height, &mut final_board.map);
    writeln!(&mut out, "{}", moves).unwrap();
    writeln!(&mut out, "{}", final_board.score).unwrap();
    write!(&mut out, "{}", map).unwrap();
}

fn calc_move_set(origin: (i32, i32)) -> Vec<(i32, i32)> {
    let up = (origin.0 - 1, origin.1);
    let down = (origin.0 + 1, origin.1);
    let left = (origin.0, origin.1 - 1);
    let right = (origin.0, origin.1 + 1);
    let move_set: Vec<(i32, i32)> = vec![up, down, left, right];
    return move_set;
}

fn print_map(x: i32, y: i32, map: &mut HashMap<(i32, i32), char>) {
    for i in 0..x {
        for j in 0..y {
            print!("{}", map.get(&(i, j)).unwrap());
        }
        println!();
    }
}
fn write_map(x: i32, y: i32, map: &mut HashMap<(i32, i32), char>) -> String {
    let mut ret_var = String::new();
    for i in 0..x {
        for j in 0..y {
            ret_var.push(*map.get(&(i, j)).unwrap());
        }
        ret_var.push('\n');
    }
    ret_var
}

fn is_not_wall(coord: (i32, i32), map: &mut HashMap<(i32, i32), char>) -> bool {
    if map.get(&coord) == Some(&'#') {
        return false;
    }
    true
}
