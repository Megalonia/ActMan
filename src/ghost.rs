use crate::*;
//CREATE GHOST OBJECT
//WITH X,Y COORDS
//DIR: CHAR EITHER U,D,L,R
//NAME:: P,B,D,R
//WHAT ARE THEY ON TOP OF; ' ' 'A' 'P' 'B' 'D' 'R' '.' '*' '$'
//RENDER ORDER FOR FIXING MAP
#[derive(Debug,Clone)]
pub struct Ghost {
    pub x: i32,
    pub y: i32,
    pub dir: char,
    pub name: char,
    pub on_top_of: char,
    pub render_order: i32,
}

//SPAWNERS
pub fn spawn_punky(game_input: &Inputs) -> Ghost {
    Ghost {
        x: game_input.p_pos.0,
        y: game_input.p_pos.1,
        dir: game_input.ghost_start_dir[0],
        name: 'P',
        on_top_of: ' ',
        render_order: 0,
    }
}

pub fn spawn_bunky(game_input: &Inputs) -> Ghost {
    Ghost {
        x: game_input.b_pos.0,
        y: game_input.b_pos.1,
        dir: game_input.ghost_start_dir[1],
        name: 'B',
        on_top_of: ' ',
        render_order: 0,
    }
}

pub fn spawn_dunky(game_input: &Inputs) -> Ghost {
    Ghost {
        x: game_input.d_pos.0,
        y: game_input.d_pos.1,
        dir: game_input.ghost_start_dir[2],
        name: 'D',
        on_top_of: ' ',
        render_order: 0,
    }
}

pub fn spawn_runky(game_input: &Inputs) -> Ghost {
    Ghost {
        x: game_input.r_pos.0,
        y: game_input.r_pos.1,
        dir: game_input.ghost_start_dir[3],
        name: 'R',
        on_top_of: ' ',
        render_order: 0,
    }
}
//NEW STUFF
/*TODO:TO IMPL WITH GAMEBOARD INSTEAD OF INPUT*/
pub fn ghost_move(game_board :&mut GameBoard, ghost: &mut Ghost) {
    //find default move
    let default_move = process_move(ghost.dir, &(ghost.x, ghost.y));
    //find valid moves
    let move_set = calc_move_set((ghost.x, ghost.y));
    let find_valid: Vec<bool> = move_set
        .iter()
        .map(|&dir| is_not_wall(dir, &mut game_board.map))
        .collect();
    let count = find_valid.iter().filter(|&n| *n == true).count();
    if count == 1 {
        let index = find_valid.iter().position(|&r| r == true).unwrap();
        let new_char = *game_board.map.get(&move_set[index]).unwrap();
        game_board.map.insert(move_set[index], ghost.name);
        if on_top_of_helper(ghost.on_top_of) {
            game_board.map.insert((ghost.x, ghost.y), ghost.on_top_of);
        }
        ghost.x = move_set[index].0;
        ghost.y = move_set[index].1;
        ghost.on_top_of = new_char;
        match index {
            0 => ghost.dir = 'U',
            1 => ghost.dir = 'D',
            2 => ghost.dir = 'L',
            3 => ghost.dir = 'R',
            _ => {}
        };
    } else if count == 2 {
        if game_board.map.get(&default_move).unwrap() == &'#' {
            two_spots(&mut game_board.map, ghost, move_set, find_valid);
        } else {
            let new_char = *game_board.map.get(&default_move).unwrap();
            game_board.map.insert(default_move, ghost.name);
            if on_top_of_helper(ghost.on_top_of) {
                game_board.map.insert((ghost.x, ghost.y), ghost.on_top_of);
            }
            ghost.x = default_move.0;
            ghost.y = default_move.1;
            ghost.on_top_of = new_char;
        }
    } else {
        //DO SPECIAL MOVE
        match ghost.name {
            'P' => punky(game_board,ghost,move_set,find_valid),
            'B' => bunky(game_board,ghost,move_set,find_valid),
            'D' => dunky(game_board,ghost,move_set,find_valid),
            'R' => runky(game_board,ghost,move_set),
              _ => {}
        };
    }

}
pub fn punky(game_board: &mut GameBoard, ghost: &mut Ghost, move_set: Vec<(i32,i32)>, find_valid: Vec<bool>) {
    let dist_vec = find_dist((game_board.ActMan.x,game_board.ActMan.y), &move_set);
    let mut valid_dist = Vec::new();

    for (idx, valid) in find_valid.iter().enumerate() {
        if *valid {
            valid_dist.insert(idx, (idx, dist_vec[idx]));
        } else {
            valid_dist.insert(idx, (idx, 9999.9999));
        }
    }
    valid_dist.sort_by(|(_a, c), (_b, d)| c.partial_cmp(d).unwrap());
    let mut min_pair = valid_dist[0];
    if valid_dist
        .iter()
        .any(|&(a, b)| b == min_pair.1 && a != min_pair.0)
        && min_pair.0 != 0
    {
        for index in valid_dist.iter() {
            if min_pair.0 == 3 && index.0 == 0 {
                min_pair.0 = 0;
                break;
            } else if min_pair.0 == 2 && (index.0 == 0 || index.0 == 3 || index.0 == 1) {
                match index.0 {
                    0 => min_pair.0 = 0,
                    1 => min_pair.0 = 1,
                    3 => min_pair.0 = 3,
                    _ => {}
                };
                break;
            } else if min_pair.0 == 1 && (index.0 == 0 || index.0 == 3) {
                match index.0 {
                    0 => min_pair.0 = 0,
                    3 => min_pair.0 = 3,
                    _ => {}
                };
                break;
            }
        }
    }

    match min_pair.0 {
        0 => ghost.dir = 'U',
        1 => ghost.dir = 'D',
        2 => ghost.dir = 'L',
        3 => ghost.dir = 'R',
        _ => {}
    }

    let new_char = *game_board.map.get(&move_set[min_pair.0]).unwrap();
    game_board.map.insert(move_set[min_pair.0], ghost.name);
    if on_top_of_helper(ghost.on_top_of) {
        game_board.map.insert((ghost.x, ghost.y), ghost.on_top_of);
    }
    ghost.x = move_set[min_pair.0].0;
    ghost.y = move_set[min_pair.0].1;
    ghost.on_top_of = new_char;

}

pub fn bunky(game_board: &mut GameBoard, ghost: &mut Ghost, move_set: Vec<(i32,i32)>, find_valid: Vec<bool>){

    let dist_vec;
    match game_board.ActMan.dir {
        'U' => dist_vec = find_dist((game_board.ActMan.x-4, game_board.ActMan.y), &move_set),
        'D' => dist_vec = find_dist((game_board.ActMan.x+4, game_board.ActMan.y + 4), &move_set),
        'L' => dist_vec = find_dist((game_board.ActMan.x, game_board.ActMan.y - 4), &move_set),
        'R' => dist_vec = find_dist((game_board.ActMan.x, game_board.ActMan.y + 4), &move_set),
        _ => dist_vec = find_dist((game_board.ActMan.x, game_board.ActMan.y + 4), &move_set),
    };
    let mut valid_dist = Vec::new();

    for (idx, valid) in find_valid.iter().enumerate() {
        if *valid {
            valid_dist.insert(idx, (idx, dist_vec[idx]));
        } else {
            valid_dist.insert(idx, (idx, 9999.9999));
        }
    }
    valid_dist.sort_by(|(_a, c), (_b, d)| c.partial_cmp(d).unwrap());
    let mut min_pair = valid_dist[0];
    if valid_dist
        .iter()
        .any(|&(a, b)| b == min_pair.1 && a != min_pair.0)
        && min_pair.0 != 0
    {
        for index in valid_dist.iter() {
            if min_pair.0 == 3 && index.0 == 0 {
                min_pair.0 = 0;
                break;
            } else if min_pair.0 == 2 && (index.0 == 0 || index.0 == 3 || index.0 == 1) {
                match index.0 {
                    0 => min_pair.0 = 0,
                    1 => min_pair.0 = 1,
                    3 => min_pair.0 = 3,
                    _ => {}
                };
                break;
            } else if min_pair.0 == 1 && (index.0 == 0 || index.0 == 3) {
                match index.0 {
                    0 => min_pair.0 = 0,
                    1 => min_pair.0 = 1,
                    3 => min_pair.0 = 3,
                    _ => {}
                };
                break;
            }
        }
    }
    match min_pair.0 {
        0 => ghost.dir = 'U',
        1 => ghost.dir = 'D',
        2 => ghost.dir = 'L',
        3 => ghost.dir = 'R',
        _ => {}
    }

    let new_char = *game_board.map.get(&move_set[min_pair.0]).unwrap();
    game_board.map.insert(move_set[min_pair.0], ghost.name);
    if on_top_of_helper(ghost.on_top_of) {
        game_board.map.insert((ghost.x, ghost.y), ghost.on_top_of);
    }
    ghost.x = move_set[min_pair.0].0;
    ghost.y = move_set[min_pair.0].1;
    ghost.on_top_of = new_char;

}
pub fn dunky(game_board: &mut GameBoard, ghost: &mut Ghost, move_set: Vec<(i32,i32)>, find_valid: Vec<bool>) {
    
    let dist_vec = find_dist(
        game_board.dunky_patrol_list[game_board.dunky_pointer as usize],
        &move_set,
    );
    let mut valid_dist = Vec::new();

    for (idx, valid) in find_valid.iter().enumerate() {
        if *valid {
            valid_dist.insert(idx, (idx, dist_vec[idx]));
        } else {
            valid_dist.insert(idx, (idx, 9999.9999));
        }
    }
    valid_dist.sort_by(|(_a, c), (_b, d)| c.partial_cmp(d).unwrap());
    let mut min_pair = valid_dist[0];
    if valid_dist
        .iter()
        .any(|&(a, b)| b == min_pair.1 && a != min_pair.0)
        && min_pair.0 != 0
    {
        for index in valid_dist.iter() {
            if (min_pair.0 == 3 && index.0 == 0)  && min_pair.1 == index.1{
                min_pair.0 = 0;
                break;
            } else if (min_pair.0 == 2 && (index.0 == 0 || index.0 == 3 || index.0 == 1)) && min_pair.1 == index.1{
                match index.0 {
                    0 => min_pair.0 = 0,
                    1 => min_pair.0 = 1,
                    3 => min_pair.0 = 3,
                    _ => {}
                };
                break;
            } else if (min_pair.0 == 1 && (index.0 == 0 || index.0 == 3)) && min_pair.1 == index.1 {
                match index.0 {
                    0 => min_pair.0 = 0,
                    3 => min_pair.0 = 3,
                    _ => {}
                };
                break;
            }
        }
    }

    match min_pair.0 {
        0 => ghost.dir = 'U',
        1 => ghost.dir = 'D',
        2 => ghost.dir = 'L',
        3 => ghost.dir = 'R',
        _ => {}
    }

    let new_char = *game_board.map.get(&move_set[min_pair.0]).unwrap();
    game_board.map.insert(move_set[min_pair.0], ghost.name);
    if on_top_of_helper(ghost.on_top_of) {
        game_board.map.insert((ghost.x, ghost.y), ghost.on_top_of);
    }
    ghost.x = move_set[min_pair.0].0;
    ghost.y = move_set[min_pair.0].1;
    ghost.on_top_of = new_char;
}

pub fn runky(game_board: &mut GameBoard, ghost: &mut Ghost, move_set: Vec<(i32,i32)>) {
    let mut r_spot = (0, 0);
    loop {
        if game_board.runky_move_list.len() == game_board.runky_pointer as usize {
            game_board.runky_pointer = 0;
        }
        match game_board.runky_move_list[game_board.runky_pointer as usize] {
            'U' => {
                r_spot = move_set[0];
                ghost.dir = 'U'
            }
            'D' => {
                r_spot = move_set[1];
                ghost.dir = 'D'
            }
            'L' => {
                r_spot = move_set[2];
                ghost.dir = 'L'
            }
            'R' => {
                r_spot = move_set[3];
                ghost.dir = 'R'
            }
            _ => {}
        }
        if is_not_wall(r_spot, &mut game_board.map) {
            game_board.runky_pointer += 1;
            break;
        }
        game_board.runky_pointer += 1;
    }
    let new_char = *game_board.map.get(&r_spot).unwrap();
    game_board.map.insert(r_spot, ghost.name);
    if on_top_of_helper(ghost.on_top_of) {
        game_board.map.insert((ghost.x, ghost.y), ghost.on_top_of);
    }
    ghost.x = r_spot.0;
    ghost.y = r_spot.1;
    ghost.on_top_of = new_char;

}
//OLD BUT STILL USEABLE
pub fn find_dist(a_pos: (i32, i32), move_set: &Vec<(i32, i32)>) -> Vec<f32> {
    let up = distance_formula(
        a_pos.0 as f32,
        a_pos.1 as f32,
        move_set[0].0 as f32,
        move_set[0].1 as f32,
    );
    let down = distance_formula(
        a_pos.0 as f32,
        a_pos.1 as f32,
        move_set[1].0 as f32,
        move_set[1].1 as f32,
    );
    let left = distance_formula(
        a_pos.0 as f32,
        a_pos.1 as f32,
        move_set[2].0 as f32,
        move_set[2].1 as f32,
    );
    let right = distance_formula(
        a_pos.0 as f32,
        a_pos.1 as f32,
        move_set[3].0 as f32,
        move_set[3].1 as f32,
    );

    vec![up, down, left, right]
}

pub fn two_spots(
    map: &mut HashMap<(i32, i32), char>,
    ghost: &mut Ghost,
    move_set: Vec<(i32, i32)>,
    find_valid: Vec<bool>,
) {
    match ghost.dir {
        'U' | 'D' => {
            if find_valid[2] == true {
                let new_char = *map.get(&move_set[2]).unwrap();
                map.insert(move_set[2], ghost.name);
                if on_top_of_helper(ghost.on_top_of) {
                    map.insert((ghost.x, ghost.y), ghost.on_top_of);
                }
                ghost.x = move_set[2].0;
                ghost.y = move_set[2].1;
                ghost.dir = 'L';
                ghost.on_top_of = new_char;
            } else if find_valid[3] == true {
                let new_char = *map.get(&move_set[3]).unwrap();
                map.insert(move_set[3], ghost.name);
                if on_top_of_helper(ghost.on_top_of) {
                    map.insert((ghost.x, ghost.y), ghost.on_top_of);
                }
                ghost.x = move_set[3].0;
                ghost.y = move_set[3].1;
                ghost.dir = 'R';
                ghost.on_top_of = new_char;
            }
        }
        'L' | 'R' => {
            if find_valid[0] == true {
                let new_char = *map.get(&move_set[0]).unwrap();
                map.insert(move_set[0], ghost.name);
                if on_top_of_helper(ghost.on_top_of) {
                    map.insert((ghost.x, ghost.y), ghost.on_top_of);
                }
                ghost.x = move_set[0].0;
                ghost.y = move_set[0].1;
                ghost.dir = 'U';
                ghost.on_top_of = new_char;
            } else if find_valid[1] == true {
                let new_char = *map.get(&move_set[1]).unwrap();
                map.insert(move_set[1], ghost.name);
                if on_top_of_helper(ghost.on_top_of) {
                    map.insert((ghost.x, ghost.y), ghost.on_top_of);
                }
                ghost.x = move_set[1].0;
                ghost.y = move_set[1].1;
                ghost.dir = 'D';
                ghost.on_top_of = new_char;
            }
        }

        _ => {}
    };
}

pub fn on_top_of_helper(char_atop: char) -> bool {
    match char_atop {
        'P' => false,
        'B' => false,
        'R' => false,
        'D' => false,
        _ => true,
    }
}

pub fn process_move(key_move: char, origin: &(i32, i32)) -> (i32, i32) {
    match key_move {
        'U' => (origin.0 - 1, origin.1),
        'D' => (origin.0 + 1, origin.1),
        'L' => (origin.0, origin.1 - 1),
        'R' => (origin.0, origin.1 + 1),
        _ => (0, 0),
    }
}

pub fn distance_formula(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    return (x2 - x1).powf(2.0) + (y2 - y1).powf(2.0);
}
