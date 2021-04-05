use crate::*;
use linked_hash_map::LinkedHashMap;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

//STRUCT FOR INPUTS/GAME STATE
//hashmap of : (x,y), ' ' 'A' 'P' 'B' 'D' 'R' '.' '*' '$'
//starting direction of ghost
//dunky patrol list and runky move list
//current positions of A,P,B,D,R
//runky and dunky pointer
//BFS bath: COMING SOON
#[derive(Debug)]
pub struct Inputs {
    pub height: i32,
    pub width: i32,
    pub map: HashMap<(i32, i32), char>,
    pub ghost_start_dir: Vec<char>,
    pub dunky_patrol_list: Vec<(i32, i32)>,
    pub runky_move_list: Vec<char>,
    pub a_pos: (i32, i32),
    pub p_pos: (i32, i32),
    pub b_pos: (i32, i32),
    pub d_pos: (i32, i32),
    pub r_pos: (i32, i32),
    pub runky_pointer: i32,
    pub dunky_pointer: i32,
    pub loot: usize,
}

pub fn arg_parse() -> Inputs {
    let args: Vec<String> = env::args().collect();
    //OPEN SAMPLE INPUT
    let file = File::open(args[1].clone()).unwrap();
    let mut buffer = BufReader::new(file);
    let mut contents = String::new();
    buffer.read_to_string(&mut contents).unwrap();
    //SPLIT ON NEW LINE COLLECT INTO VEC OF REF STR
    let mut inputs_as_vec = contents.split("\n").collect::<Vec<&str>>();
    //GET WIDTH AND HEIGHT
    let dimensions = inputs_as_vec[0].split(" ").collect::<Vec<&str>>();
    let x: usize = dimensions[0].parse().unwrap();
    let y: usize = dimensions[1].parse().unwrap();
    inputs_as_vec.remove(0);
    //REMOVE EXTRA NEWLINES
    inputs_as_vec.pop();
    inputs_as_vec.pop();
    //CREATE VARS FOR OBJECT
    let mut map: HashMap<(i32, i32), char> = HashMap::with_capacity(x * y);
    let mut direction: Vec<char> = Vec::new();
    let mut dunkey_patrol_list: Vec<(i32, i32)> = Vec::new();
    let mut runkey_move_list: Vec<char> = Vec::new();
    let mut act_man_pos: (i32, i32) = (0, 0);
   let mut p_pos: (i32, i32) = (0, 0);
    let mut b_pos: (i32, i32) = (0, 0);
    let mut d_pos: (i32, i32) = (0, 0);
    let mut r_pos: (i32, i32) = (0, 0);
    //gimme 
    let mut da_loot: usize = 0;
    let loot_types = ['$','.','*'];
    //FOR THE INDEX AND THE LINE OF THE VECTOR INPUT
    for (i, line) in inputs_as_vec.iter().enumerate() {
        //IF THE INPUT IS PART OF THE MAP
        if line.len() == y {
            //FOR THE INDEX AND A CHAR IN EACH LINE
            for (j, c) in line.chars().enumerate() {
                //ALWAYS INSERT INTO MAP
                map.insert((i as i32, j as i32), c);
                //IF THE CHAR IS A,P,B,D,R SAVE INDEX
                if c == 'A' {
                    act_man_pos = (i as i32, j as i32);
                } else if c == 'P' {
                    p_pos = (i as i32, j as i32);
                } else if c == 'B' {
                    b_pos = (i as i32, j as i32);
                } else if c == 'D' {
                    d_pos = (i as i32, j as i32);
                } else if c == 'R' {
                    r_pos = (i as i32, j as i32);
                } else if loot_types.iter().any(|&i| i == c) {
                    // c = * $ . 
                    da_loot += 1;
                }
            }
        //IF LEN IS 4, SAVE GHOST DIR
        } else if line.len() == 4 {
            direction = line.chars().collect::<Vec<char>>();
        //IF MOVE LIST
        } else if i == inputs_as_vec.len() - 1 {
            runkey_move_list = line.chars().collect::<Vec<char>>();
        // ELSE PAIRS
        }  if i == x+1  {
            let mut split_line = line.split(' ').collect::<VecDeque<&str>>();
            let number_of_pairs = split_line[0].parse::<i32>().unwrap();
            split_line.remove(0);
            let mut pairs_found = 0;

            while number_of_pairs != pairs_found {
                let x_cord = split_line[0].parse::<i32>().unwrap();
                let y_cord = split_line[1].parse::<i32>().unwrap();
                let pair: (i32, i32) = (x_cord, y_cord);
                dunkey_patrol_list.push(pair);
                split_line.pop_front();
                split_line.pop_front();
                pairs_found += 1;
            }
        }
    }
    //CONSTRUCT ALL VALID PATHS IN ORDER
    //let mut graph = construct_graph_from_act_man(act_man_pos, &mut map);
    //let bfs = find_best_path(&mut graph, &mut map);
   let input = Inputs {
        width: x as i32,
        height: y as i32,
        map: map,
        ghost_start_dir: direction,
        dunky_patrol_list: dunkey_patrol_list,
        runky_move_list: runkey_move_list,
        a_pos: act_man_pos,
        p_pos: p_pos,
        b_pos: b_pos,
        d_pos: d_pos,
        r_pos: r_pos,
        runky_pointer: 0,
        dunky_pointer: 0,
        loot: da_loot
    };
   //println!("{:?}",input);

   input
}

//CONSTRUCT HASHSET
pub fn construct_graph_from_act_man(
    a_pos: (i32, i32),
    map: &mut HashMap<(i32, i32), char>,
) -> LinkedHashMap<(i32, i32), Vec<(i32, i32)>> {
    /*   let mut moves: Vec<(i32,i32)> = vec![a_pos];
    let mut valid_moves = only_valid_moves(a_pos,map);
    println!("moves {:?}",moves);
    println!("vm {:?}",valid_moves);
    let mut i = 0;
    while i < 10 {
        let mut new_moves: Vec<(i32,i32)> = Vec::new();
        for vm in &valid_moves {
             new_moves.append(&mut valid_moves_no_repeat(*vm,map,&moves));
         }
         println!("{:?}",new_moves);
         valid_moves.append(&mut new_moves);
         new_moves.clear();
         i += 1; }*/
    //FIND MOVES THAT ARE NOT WALLS
    //CREATE LINKED SET
    let mut count: usize = 0;
    for index in map.into_iter() {
        if index.1 != &'#' {
            count += 1;
        }
    }
    let mut act_man_graph: LinkedHashMap<(i32, i32), Vec<(i32, i32)>> = LinkedHashMap::new();
    act_man_graph.insert(a_pos, only_valid_moves(a_pos, map));

    //NEW MOVES FOR LOOP
    let mut valid_moves = only_valid_moves(a_pos, map);
    let mut new_moves: Vec<(i32, i32)> = Vec::new();
    let mut visited: Vec<(i32, i32)> = vec![a_pos];
    //INF LOOP
    loop {
        //FOR valid moves in vector
        for vm in &valid_moves {
            //FIND MOVES AROUND VALID MOVE
            //            new_moves.append(&mut valid_moves_no_repeat(*vm,map,&act_man_graph));
            new_moves.append(&mut only_valid_moves(*vm, map));
            //            ^^^ TODO: Cyclical
            //INSERT VALID MOVE TO SET
            //
     //           act_man_graph.insert(*vm,valid_moves_no_repeat(*vm,map,&act_man_graph));
            if !visited.contains(vm) {
                act_man_graph.insert(*vm, only_valid_moves(*vm, map));
                visited.push(*vm);
            }
        }
        //IF NO MORE NEW MOVES
        if visited.len() == count {
            //EXIT LOOP
            break;
        } else {
            //REMOVE OLD MOVES
            valid_moves.clear();
            //AND CHECK NEW MOVES
            valid_moves.append(&mut new_moves);
        }
    }
    act_man_graph
}

pub fn find_best_path(
    graph: &mut LinkedHashMap<(i32, i32), Vec<(i32, i32)>>,
    map: &mut HashMap<(i32, i32), char>,
) -> Vec<char> {
    #[derive(Debug, Clone, Copy)]
    pub struct Node {
        coord: (i32, i32),
        score: i32,
        path: Option<(i32, i32)>,
        //        path_2: Option<Vec<Box<Node>>>
    }
    let mut deque: VecDeque<Node> = VecDeque::new();
    deque.push_back(Node {
        coord: *graph.keys().next().unwrap(),
        score: 0,
        path: None,
        //       path_2: None
    });
    let mut path: Vec<Node> = Vec::new();
    let mut visited: Vec<(i32, i32)> = Vec::new();
    while !deque.is_empty() {
        if deque.front().unwrap().score >= 20 {
            let mut curr: Node = *deque.front().unwrap();
            let mut sol_p: Vec<(i32, i32)> = vec![curr.coord];
            let mut to_sub;
            while curr.score != 0 {
                match map.get(&curr.coord).unwrap() {
                    '.' => to_sub = 1,
                    '*' => to_sub = 10,
                    '$' => to_sub = 5,
                    _ => to_sub = 0,
                }
                for n in &path {
                    /*                   println!("curr.score -> {:?}", curr.score);
                    println!("n.score -> {:?}", n.score);
                    println!("curr.coord -> {:?}", curr.coord);
                    println!("n.coord -> {:?}", n.coord);
                    println!("to_sub -> {:?}", to_sub);
                    println!("WOO");*/
                    // Infinite loops because to_sub is always 0
                    if n.score == curr.score - to_sub {
                        let move_set = calc_move_set(curr.coord);
                        if move_set.iter().any(|&i| i == n.coord) {
                            sol_p.push(n.coord);
                            curr = *n;
                            break;
                        }
                    }
                }
            }
            return coord_to_dirs(&mut sol_p);
        }
        let edges = graph.get(&deque.front().unwrap().coord).unwrap();
        path.push(deque.pop_front().unwrap());
        for e in edges {
            if !visited.contains(e) {
                match map.get(e).unwrap() {
                    '.' => deque.push_back(Node {
                        coord: *e,
                        score: (path[path.len() - 1].score + 1),
                        path: if path.len() == 1 {
                            Some(path[0].coord)
                        } else {
                            Some(path[path.len() - 1].coord)
                        },
                    }),
                    '$' => deque.push_back(Node {
                        coord: *e,
                        score: (path[path.len() - 1].score + 5),
                        path: if path.len() == 1 {
                            Some(path[0].coord)
                        } else {
                            Some(path[path.len() - 1].coord)
                        },
                    }),

                    '*' => deque.push_back(Node {
                        coord: *e,
                        score: (path[path.len() - 1].score + 10),
                        path: if path.len() == 1 {
                            Some(path[0].coord)
                        } else {
                            Some(path[path.len() - 1].coord)
                        },
                    }),
                    _ => deque.push_back(Node {
                        coord: *e,

                        score: (path[path.len() - 1].score),
                        path: if path.len() == 1 {
                            Some(path[0].coord)
                        } else {
                            Some(path[path.len() - 1].coord)
                        },
                    }),
                }
                visited.push(*e);
            } else {
                deque.push_back(Node {
                    coord: *e,
                    score: path[path.len() - 1].score,
                    path: Some(path[path.len() - 1].coord),
                });
            }
        }
    }
    vec![]
}

pub fn coord_to_dirs(coords: &mut Vec<(i32, i32)>) -> Vec<char> {
    let mut origin = coords[coords.len() - 1];
    coords.pop();
    coords.reverse();
    let mut dir: Vec<char> = Vec::new();
    for coord in coords {
        let move_set: Vec<(i32, i32)> = calc_move_set(origin);
        if *coord == move_set[0] {
            dir.push('U');
            origin = *coord;
        } else if *coord == move_set[1] {
            dir.push('D');
            origin = *coord;
        } else if *coord == move_set[2] {
            dir.push('L');
            origin = *coord;
        } else if *coord == move_set[3] {
            dir.push('R');
            origin = *coord;
        }
    }
    dir
}

pub fn valid_moves_no_repeat(
    origin: (i32, i32),
    map: &mut HashMap<(i32, i32), char>,
    visited: &LinkedHashMap<(i32, i32), Vec<(i32, i32)>>,
) -> Vec<(i32, i32)> {
    let move_set = calc_move_set(origin);
    let find_valid: Vec<bool> = move_set.iter().map(|&dir| is_not_wall(dir, map)).collect();
    let mut valid_moves: Vec<(i32, i32)> = Vec::new();

    for (line, is_true) in find_valid.iter().enumerate() {
        if *is_true && !visited.contains_key(&move_set[line]) {
            valid_moves.push(move_set[line]);
        }
    }
    valid_moves
}

//FIND MOVES THAR ARE NOT A WALL
pub fn only_valid_moves(
    origin: (i32, i32),
    map: &mut HashMap<(i32, i32), char>,
) -> Vec<(i32, i32)> {
    //STARTING POINT
    let move_set = calc_move_set(origin);
    let find_valid: Vec<bool> = move_set.iter().map(|&dir| is_not_wall(dir, map)).collect();
    let mut valid_moves: Vec<(i32, i32)> = Vec::new();

    for (line, is_true) in find_valid.iter().enumerate() {
        if *is_true {
            valid_moves.push(move_set[line]);
        }
    }
    valid_moves
}
