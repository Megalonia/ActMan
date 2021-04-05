use crate::*;
use std::collections::VecDeque;

pub fn act_man_astar_gs(game_board: GameBoard) -> Vec<char> {
    let mut frontier: VecDeque<Vec<char>> = VecDeque::new();
    frontier.push_back(vec![]);
    while !frontier.is_empty() {

        let mut p = frontier.pop_front().unwrap();
        let mut sk = transition_function(&mut game_board.clone(), &mut p);

        if goal(&mut sk) {
            return p;
        }

        for a_move in valid_moves(&mut sk) {
            let mut px = sk.moves_so_far.clone();
            px.extend(&vec![a_move]);
            let sx = transition_function(&mut game_board.clone(), &px);
           
            if frontier.is_empty() {
                frontier.push_back(px);
            }else {
                let front = transition_function(&mut game_board.clone(), &frontier.front().unwrap());
                let front_cost =  g_star(game_board.clone(),front.clone()) + h_star(front.clone());
                let cost =  g_star(game_board.clone(),sx.clone()) + h_star(sx.clone());

                if  cost <  front_cost {
                   frontier.push_front(px.clone()); 
                }else{
                    for (i,path) in frontier.clone().into_iter().enumerate() {
                        let path_board = transition_function(&mut game_board.clone(), &path);
                        let path_cost = g_star(game_board.clone(),path_board.clone()) + h_star(path_board.clone());
                        if cost == path_cost && px.len() == path_board.moves_so_far.len() {
                            frontier.insert(i,px.clone());
                        }
                    }
                }
 
            }
           
            


            
        }
    }

    vec![]
}



pub fn h_star(game_board: GameBoard) -> usize {
    if game_board.ActMan.alive != true {
        return 99999999;
    }
    game_board.loot
}

pub fn g_star(init_board: GameBoard, s: GameBoard) -> usize {
    if s.ActMan.alive != true {
        return 99999999;
    }
    init_board.loot - s.collected
}



