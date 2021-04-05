use crate::*;
pub use ghost::*;

#[allow(non_snake_case)]
#[derive(Debug,Clone)]
pub struct GameBoard {
    //Cord, Char
    pub map: HashMap<(i32,i32), char>,
    // usinged int
    pub score: usize,
    pub runky_pointer: usize,
    pub dunky_pointer: usize,
    // vec of coords
    pub dunky_patrol_list: Vec<(i32, i32)>,
    // vec of dirs
    pub runky_move_list: Vec<char>,
    pub ActMan: ActMan,
    pub Ghosts: Vec<Ghost>,
    pub moves_so_far: Vec<char>,
    pub loot: usize,
    pub collected: usize
    /*
    pub a_pos: (i32, i32),-> 0
    pub p_pos: (i32, i32),-> 1
    pub b_pos: (i32, i32),-> 2
    pub d_pos: (i32, i32),-> 3
    pub r_pos: (i32, i32),-> 4
    */
}

pub fn goal(game_board: &mut GameBoard) -> bool{
   if game_board.collected == 15 {
        return true
   }
   false 
}


pub fn create_board(game_inputs: &mut Inputs, act_man: ActMan, pbdr: Vec<Ghost>) -> GameBoard {
    GameBoard {
        map: game_inputs.map.clone(),
        score: 0,
        runky_pointer: 0,
        dunky_pointer: 0,
        dunky_patrol_list: game_inputs.dunky_patrol_list.clone(),
        runky_move_list: game_inputs.runky_move_list.clone(),
        ActMan: act_man,
        Ghosts: pbdr,
        moves_so_far: vec![],
        loot: game_inputs.loot,
        collected: 0
    } 
}

pub fn act_man_move(game_board: &mut GameBoard, dir: char) {
    if game_board.ActMan.alive == true {
        game_board.map.insert((game_board.ActMan.x,game_board.ActMan.y), ' ');
        let valid_moves = valid_moves(game_board);
    //    println!("CHECKING VALID MOVES{:?}",valid_moves);
        if valid_moves.iter().any(|&x| x == dir) {

            match dir {
               'U' => {game_board.ActMan.x -= 1;
                       game_board.ActMan.dir = 'U';
                       game_board.moves_so_far.push('U')
                      },
               'D' => {game_board.ActMan.x += 1;
                       game_board.ActMan.dir = 'D';
                       game_board.moves_so_far.push('D')
                      },
               'L' => {game_board.ActMan.y -= 1;
                       game_board.ActMan.dir = 'L';
                       game_board.moves_so_far.push('L')
                      },
               'R' => {game_board.ActMan.y += 1;
                       game_board.ActMan.dir = 'R';
                       game_board.moves_so_far.push('R')
                      },
                _ => {},
            };

            match game_board.map.get(&(game_board.ActMan.x,game_board.ActMan.y)) {
                Some('.') => {game_board.score +=  1;game_board.collected += 1},
                Some('$') => {game_board.score +=  5;game_board.collected += 1},
                Some('*') => {game_board.score += 10;game_board.collected += 1},
                 _ => {},
            }

            game_board.map.insert((game_board.ActMan.x,game_board.ActMan.y), 'A');
        }
    }

}

pub fn on_ghost(game_board: &mut GameBoard) -> bool {
    let a_pos : (i32,i32) = (game_board.ActMan.x,game_board.ActMan.y);
    let p_pos : (i32,i32) = (game_board.Ghosts[0].x,game_board.Ghosts[0].y);
    let b_pos : (i32,i32) = (game_board.Ghosts[1].x,game_board.Ghosts[1].y);
    let d_pos : (i32,i32) = (game_board.Ghosts[2].x,game_board.Ghosts[2].y);
    let r_pos : (i32,i32) = (game_board.Ghosts[3].x,game_board.Ghosts[3].y);
    //if the vector of ghost pos contains act mans post
    if vec![p_pos,b_pos,d_pos,r_pos].contains(&a_pos) {
        game_board.map.insert((game_board.ActMan.x,game_board.ActMan.y), 'X');
        game_board.ActMan.alive = false;
        return true;
    }//else
    false
}
pub fn valid_moves(game_board: &mut GameBoard) -> Vec<char> {
    if game_board.ActMan.alive == false {
        return vec![];
    }
    let move_set = calc_move_set((game_board.ActMan.x,game_board.ActMan.y));
    let mut valid_moves: Vec<char> = vec![];
/*    println!("############valid_moves################");
    println!("Pos {:?}", (game_board.ActMan.x,game_board.ActMan.y));
    println!("move_set:->{:?}",move_set);*/
    for (i,a_move) in move_set.into_iter().enumerate() {
        let m = game_board.map.get(&a_move).unwrap();
        if m != &'#' && m != &'P' && m != &'B' && m != &'D' && m != &'R' {
            match i {
                0 => valid_moves.push('U'),
                1 => valid_moves.push('D'),
                2 => valid_moves.push('L'),
                3 => valid_moves.push('R'),
                _ => {},
            }
        }
    }
    valid_moves
}

//TODO: add ghost move to tranisiton
pub fn transition_function(game_board: &mut GameBoard,sequence: &Vec<char>) -> GameBoard {
        let mut punky = game_board.Ghosts[0].clone();
        let mut bunky = game_board.Ghosts[1].clone();
        let mut dunky = game_board.Ghosts[2].clone();
        let mut runky = game_board.Ghosts[3].clone();

        for dir in sequence {
            act_man_move(game_board,*dir);
            if on_ghost(game_board) {break}
            ghost_move(game_board,&mut punky);
            game_board.Ghosts[0] = punky.clone();
            if on_ghost(game_board) {break}
            ghost_move(game_board,&mut bunky);
            game_board.Ghosts[1] = bunky.clone();
            if on_ghost(game_board) {break}
            ghost_move(game_board,&mut dunky);
            game_board.Ghosts[2] = dunky.clone();
            if game_board.dunky_patrol_list[game_board.dunky_pointer as usize] == (dunky.x, dunky.y) {
                game_board.dunky_pointer += 1;
            } if game_board.dunky_pointer == (game_board.dunky_patrol_list.len()) {
                game_board.dunky_pointer = 0;
            }
            if on_ghost(game_board) {break}
            ghost_move(game_board,&mut runky);
            game_board.Ghosts[3] = runky.clone();
            if on_ghost(game_board) {break}
            //GHOST MOVES
            //
        }
        game_board.clone()
    }


