use crate::*;
use std::collections::VecDeque;

pub fn act_man_greedy_bfs(game_board: GameBoard) -> Vec<char> {
    let mut frontier: VecDeque<Vec<char>> = VecDeque::new();
    frontier.push_back(vec![]);
    
    while !frontier.is_empty() {
        let mut p = frontier.pop_front().unwrap();
        let mut sk = transition_function(&mut game_board.clone(), &mut p);
        if goal(&mut sk) && sk.ActMan.alive == true {
            return p;
        }
        for a_move in valid_moves(&mut sk) {
            let mut next_path = sk.moves_so_far.clone();
            next_path.extend(&vec![a_move]);
            let sx = transition_function(&mut game_board.clone(),&mut next_path);
            if frontier.front() != None {
                if h(sx.clone()) < h(transition_function(&mut game_board.clone(), &mut frontier.front().unwrap())) {
                    frontier.push_front(next_path);
                }else{
                    for (i,path) in frontier.clone().into_iter().enumerate() {
                        if h(sx.clone()) <= h(transition_function(&mut game_board.clone(), &path)) {
                            frontier.insert(i,next_path.clone());
							break;
                        }
                    }
                }
            }else {
                frontier.push_back(next_path);
            }
 
        }
    }

    vec![]
}

pub fn h(game_board: GameBoard) -> usize {
    if game_board.ActMan.alive == false {
        return 9999;
    }else if game_board.score >= GOAL_SCORE {
        return 0;
	}
	24 - game_board.score
}


