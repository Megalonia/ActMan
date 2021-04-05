use crate::*;




//TODO: Find Bug with frontier and path
//
pub fn act_man_bounded_dfs(game_board: GameBoard, depth_limit: usize) -> (bool,Option<Vec<char>>){
    let mut frontier: Vec<Vec<char>> = Vec::new(); 
    let mut limit_hit: bool = false;
    frontier.push(vec![]);
    while !frontier.is_empty() {
       let mut p = frontier.pop().unwrap();

        if &p.len() == &depth_limit {
            let mut sk = transition_function(&mut game_board.clone(),&mut p);
            if sk.score >= 20 {
                return (limit_hit,Some(p));
            }
            if valid_moves(&mut sk) == vec![] {
                limit_hit = true;
            }
        }else {
            let mut sk = transition_function(&mut game_board.clone(),&mut p);
         //   println!("IN ELSE {:?}",sk.ActMan);
            for a_move in valid_moves(&mut sk) {
                let mut new_set = sk.moves_so_far.clone();
                new_set.extend(&vec![a_move]);
          //      println!("ADDING {:?} TO FRONTIER",new_set);

                frontier.push(new_set);
            }

        }
//        println!("#######################################");
    }
    (limit_hit,None)
}

pub fn act_man_id_dfs(game_board: GameBoard) -> Vec<char> {
    let mut depth: usize = 0;
    while depth < 21 {
        let result: (bool,Option<Vec<char>>) = act_man_bounded_dfs(game_board.clone(),depth);
        if result.1 != None  {
            return result.1.unwrap();
        }
        depth += 1;
    }
    vec![]
}

