use super::*;

use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;


//THIS WAS AN UTTER FAIL, how can I do graph algos without having recursive
//inderirection.
//ex:
//struct Node<T> { A node struct templated at type T
//  coords: T
//  weight: T
//  parent: Option<Node<T>>
//  children: Option<Vec<Node<T>>>
//  
//recursive type `Node<T>` has infinite size: recursive without indirection
//would I use a Box::o
//In Rust, all values are allocated in stack by default. So the compiler needs to know the size of each. The size of a struct is the sum of all its fields size.
//
//}
//
//Better Implementation
//struct NewNode<T> {
//    coord: T,
//    children: Option<Vec<Box<NewNode<T>>>>,
//    parent: Option<Box<NewNode<T>>>,
//}
pub fn id_dfs(
    graph: &mut LinkedHashMap<(i32, i32), Vec<(i32, i32)>>,
    map: &mut HashMap<(i32, i32), char>,
) {
    let mut new_g = remake_graph(graph, map);
    let mut depth = 0;
    let mut paths: Vec<Vec<(Position, i32)>> = Vec::new();
    while depth < 21 {
        let mut frontier: Vec<Position> = Vec::new();
        find_paths(&mut new_g, depth, &mut paths, &mut frontier);
        depth += 1;
    }

    for p in paths {
        println!("{:?}",p);
    }
}

pub fn find_paths(
    graph: &mut Vec<Node<Position>>,
    depth: i32,
    paths: &mut Vec<Vec<(Position, i32)>>,
    frontier: &mut Vec<Position>
) {
    let root = graph[depth as usize].clone();
    let children = &root.children;
    for (_i, c) in children.clone().unwrap().into_iter().enumerate() {
        //println!("child:->{:?}",c);
        let node = find_node(c, graph, root.weight.y,frontier);
        let _score = root.weight.y + node.weight.x;
//`        println!("old {} new{}",root.weight.y,node.weight.x);
  //      println!("SCORE:->{:?}",score);
  //    
         let current_path = vec![(root.coord, root.weight.x), (node.coord, node.weight.x)];
         paths.push(current_path);

    } 

    //    println!("\n");

    //println!("{:?}",paths[0].pop().unwrap().1);
}

pub fn find_node(pos: Position, graph: &mut Vec<Node<Position>>, _score: i32,frontier: &mut Vec<Position> ) -> Node<Position> {
    let mut to_return: Node<Position> = Node {
        coord: Position { x: 9999, y: 9999 },
        weight: Position { x: -9999, y: -9999 },
        children: None,
        parent: None,
    };
    for x in graph {
        if (x.coord.x, x.coord.y) == (pos.x, pos.y) {
            if !frontier.contains(&x.coord) {
                frontier.push(x.coord);
                x.weight.y = x.weight.x;
                to_return = x.clone();
                return to_return;
            }
            x.weight.y = 0;
            to_return = x.clone();
            return to_return;
        }
    }
    to_return
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    coord: T,  //coords x,t
    weight: T, //(value,score)
    children: Option<Vec<T>>,
    parent: Option<Vec<T>>,
}

#[derive(Debug, Clone, Copy,PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
}

pub fn remake_graph(
    graph: &mut LinkedHashMap<(i32, i32), Vec<(i32, i32)>>,
    map: &mut HashMap<(i32, i32), char>,
) -> Vec<Node<Position>> {
    let mut new_graph: Vec<Node<Position>> = vec![Node {
        coord: Position {
            x: graph.front().unwrap().0 .0,
            y: graph.front().unwrap().0 .1,
        },
        weight: Position { x: 0, y: 0 },
        children: Some(only_valid_moves_v3(*graph.front().unwrap().0, map)),
        parent: None,
    }];
    for (i, x) in graph.into_iter().enumerate() {
        if i != 0 {//Skip root
            new_graph.push(Node {
                coord: Position {
                    x: x.0 .0,
                    y: x.0 .1,
                },
                weight: Position {
                    x: {
                        match map.get(&x.0).unwrap() {
                            '.' => 1,
                            '$' => 5,
                            '*' => 10,
                            _ => 0,
                        }
                    },
                    y: 0,
                },
                children: Some(only_valid_moves_v3(*x.0, map)),
                parent: {
                    let mut parents: Vec<Position> = Vec::new();
                    for y in &new_graph {
                        if only_valid_moves((y.coord.x, y.coord.y), map).contains(&x.0) {
                            parents.push(y.coord);
                        }
                    }
                    Some(parents)
                },
            });
        }
    }
    new_graph
}

pub fn only_valid_moves_v3(
    origin: (i32, i32),
    map: &mut HashMap<(i32, i32), char>,
) -> Vec<Position> {
    //STARTING POINT
    let move_set = calc_move_set(origin);
    let find_valid: Vec<bool> = move_set.iter().map(|&dir| is_not_wall(dir, map)).collect();
    let mut valid_moves: Vec<Position> = Vec::new();
    for (line, is_true) in find_valid.iter().enumerate() {
        if *is_true {
            valid_moves.push(Position {x: move_set[line].0,
                                 y: move_set[line].1});
        }
    }
    valid_moves
}
