use std::{cmp::Ordering, collections::{HashMap, HashSet}, fmt::{Display, Write}};

use godot::prelude::*;
use strum::{EnumCount, IntoEnumIterator};

use crate::core::{common::{acknowledger::Acknowledger, coordinate::{Coordinate, IHasCoordinates}, direction::Direction}, maze::{maze_find_paths_communicator::MazeFindPathsCommunicator, maze_tile_state::MazeTileState, path_info::PathInfo}};


const ROTATION_COST : usize = 1000;


#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct Maze {
    array : Vec<Tile>,

    dim_x : usize,

    dim_y : usize,

    n_walls : usize,

    start_coordinate : Coordinate,

    end_coordinate : Coordinate,

    reindeer : Reindeer,

    base : Base<RefCounted>,
}


impl IHasCoordinates for Maze {
    type Item = Tile;
    
    fn get_index_and_content_by_coordinate(&self, coordinate : &Coordinate) -> Option<(usize, &Self::Item)> {
        let index = coordinate.try_to_index(self.dim_x, self.dim_y)?;
        let tile = self.array.get(index)?;
        Some((index, tile))
    }
}


#[godot_api]
impl Maze {
    pub fn try_new_gd_from_str(s : &str) -> Option<Gd<Self>> {
        let mut array = Vec::new();

        let mut dim_x = 0;
        let mut dim_y = 0;

        let mut n_walls = 0;

        let mut start_coordinate_opt = None;
        let mut end_coordinate_opt = None;

        let reindeer = Reindeer::default();

        let lines = s.lines();
        for line in lines {
            if line.is_empty() {
                continue;
            }

            let y = isize::try_from(dim_y).ok()?;
            dim_y += 1;

            let mut local_dim_x = 0;
            for c in line.chars() {
                let x = isize::try_from(local_dim_x).ok()?;
                local_dim_x += 1;

                let parsed_tile_info = ParsedTile::try_from(&c).ok()?;

                let tile = parsed_tile_info.tile;
                let is_wall = tile == Tile::Wall;
                if is_wall {
                    n_walls += 1;
                }

                array.push(tile);

                if parsed_tile_info.is_start_coordinate {
                    start_coordinate_opt = Some(Coordinate::new(x, y));

                }
                if parsed_tile_info.is_end_coordinate {
                    end_coordinate_opt = Some(Coordinate::new(x, y));
                }
            }

            if dim_x == 0 {
                dim_x = local_dim_x;

            } else if local_dim_x != dim_x {
                return None;
            }
        }

        let start_coordinate = start_coordinate_opt?;
        let end_coordinate = end_coordinate_opt?;

        let gd = Gd::from_init_fn(|base| {
            Self {
                array,
                dim_x,
                dim_y,
                n_walls,
                start_coordinate,
                end_coordinate,
                reindeer,
                base,
            }
        });

        Some(gd)
    }


    fn find_paths(&self) -> Gd<MazeFindPathsCommunicator> {
        const REMEMBER_BEST_PATH : bool = true;
        const IMPLIED_UNVISITED_COST : usize = usize::MAX;

        let map_dim_x = self.dim_x;
        let map_dim_y = self.dim_y;

        let gd = self.to_gd();

        let reindeer = self.reindeer.clone();
        let start_coordinate = self.start_coordinate.clone();
        let end_coordinate = self.end_coordinate.clone();

        let communicator = MazeFindPathsCommunicator::new_gd();
        let interface = communicator.clone();

        godot::task::spawn(async move {

            let start_direction = reindeer.direction;

            let start_state = CoordinateAndDirecton {
                coordinate : start_coordinate.clone(),
                direction : start_direction.clone(),
            };

            let start_idx = start_state.coordinate.try_to_index(map_dim_x, map_dim_y).unwrap();

            let acknowledger = Acknowledger::new_gd();
            communicator.signals().update_idx().emit(
                start_idx as i32,
                MazeTileState::Committed,
                start_direction,
                &acknowledger
            );
            // AWAIT
            let _await = acknowledger.signals().ok().to_fallible_future().await;
            // AWAIT

            let mut to_visit = vec![start_state.clone()];

            let mut coordinate_state_to_cost = HashMap::from([(start_state, 0usize)]);

            let mut key_got_here_from_value_map : HashMap<CoordinateAndDirecton, HashSet<CoordinateAndDirecton>> = HashMap::new();

            let mut score_opt = None;

            while let Some(current) = to_visit.pop()  {
                let CoordinateAndDirecton {
                    coordinate : current_coordinate,
                    direction : current_direction,
                } = current;

                let current_idx = current_coordinate.try_to_index(map_dim_x, map_dim_y).unwrap();

                // Notify active and sync
                let acknowledger = Acknowledger::new_gd();
                communicator.signals().update_idx().emit(
                    current_idx as i32,
                    MazeTileState::Active,
                    current_direction,
                    &acknowledger
                );
                // AWAIT
                let _await = acknowledger.signals().ok().to_fallible_future().await;
                // AWAIT


                // Unwrapping this as if this is an option then I have done something wrong
                let cost_to_get_here = *coordinate_state_to_cost.get(&current).unwrap();

                if current_coordinate == end_coordinate {
                    let previous_score = score_opt.unwrap_or(usize::MAX);
                    if cost_to_get_here < previous_score {
                        score_opt = Some(cost_to_get_here);
                    }
                }

                let valid_candidates_and_costs = {
                    let mut results = Vec::new();

                    // Move options
                    let move_target = current_coordinate.clone() + current_direction.to_vector();
                    if let Some((_, tile)) = gd.bind().get_index_and_content_by_coordinate(&move_target) {
                        if tile == &Tile::Ground {
                            let target_cost = 1;
                            let target = CoordinateAndDirecton {
                                coordinate : move_target,
                                direction : current_direction.clone(),
                            };

                            let candidate_and_cost = (target, target_cost);

                            results.push(candidate_and_cost);

                        }
                    }

                    // Rotation options:
                    for rotation_steps in 1..Direction::COUNT {
                        let new_target_rotation = current_direction.rotate_clockwise_by_steps(rotation_steps);
                        let target = CoordinateAndDirecton {
                            coordinate: current_coordinate.clone(),
                            direction: new_target_rotation,
                        };

                        let target_cost = rotation_cost(rotation_steps);

                        let candidate_and_cost = (target, target_cost);

                        results.push(candidate_and_cost);
                    }

                    results
                };

                let mut any_new_coordinate_added = false;
                
                for (candidate, additional_cost) in valid_candidates_and_costs {
                    let CoordinateAndDirecton {
                        coordinate : candidate_coordinate,
                        direction : candidate_direction,
                    } = &candidate;

                    let candidate_index = candidate_coordinate.try_to_index(map_dim_x, map_dim_y).unwrap();

                    // Notify touched
                    let acknowledger = Acknowledger::new_gd();
                    communicator.signals().update_idx().emit(
                        candidate_index as i32,
                        MazeTileState::Touched,
                        candidate_direction.clone(),
                        &acknowledger
                    );
                    // AWAIT
                    let _await = acknowledger.signals().ok().to_fallible_future().await;
                    // AWAIT


                    let cost_to_move_here_from_current = cost_to_get_here + additional_cost;
                    let cost_to_move_here_otherwise = coordinate_state_to_cost.get(&candidate).unwrap_or(&IMPLIED_UNVISITED_COST);
        
                    let cost_to_move_cmp = cost_to_move_here_from_current.cmp(&cost_to_move_here_otherwise);
                    
                    if cost_to_move_cmp == Ordering::Equal {                       
                        add_predecessor(candidate.clone(), current.clone(), false, &mut key_got_here_from_value_map);
                    }
        
                    let skip_overshooting_cost = REMEMBER_BEST_PATH && cost_to_move_here_from_current > score_opt.unwrap_or(IMPLIED_UNVISITED_COST);
                    if cost_to_move_cmp != Ordering::Less || skip_overshooting_cost {
                        // Notify non-committed by reverting to default state
                        let acknowledger = Acknowledger::new_gd();
                        communicator.signals().update_idx().emit(
                            candidate_index as i32,
                            MazeTileState::Normal,
                            candidate_direction.clone(),
                            &acknowledger
                        );
                        // AWAIT
                        let _await = acknowledger.signals().ok().to_fallible_future().await;
                        // AWAIT


                        continue;
                    }
        
                    // Else, we should have found new optimal path

                    // Notify candidate committed
                    let acknowledger = Acknowledger::new_gd();
                    communicator.signals().update_idx().emit(
                        candidate_index as i32,
                        MazeTileState::Committed,
                        candidate_direction.clone(),
                        &acknowledger
                    );
                    // AWAIT
                    let _await = acknowledger.signals().ok().to_fallible_future().await;
                    // AWAIT

                    coordinate_state_to_cost.insert(candidate.clone(), cost_to_move_here_from_current);
        
                    to_visit.push(candidate);
        
                    add_predecessor(candidate, current, true, &mut key_got_here_from_value_map);

                    any_new_coordinate_added = true;
                }

                // At the end, sort by heurestic (lower is closer)
                if any_new_coordinate_added {
                    to_visit.sort_unstable_by(|a, b| {
                        let a_cost = coordinate_state_to_cost.get(a).unwrap_or(&IMPLIED_UNVISITED_COST);
                        let b_cost = coordinate_state_to_cost.get(b).unwrap_or(&IMPLIED_UNVISITED_COST);
                        
                        let mut ord = b_cost.cmp(&a_cost);

                        if ord == Ordering::Equal {
                            let bound = gd.bind();
                            let a_h = bound.heurestic_value(&a.coordinate);
                            let b_h = bound.heurestic_value(&b.coordinate);
                            drop(bound);

                            ord = b_h.cmp(&a_h)
                        }

                        ord
                    });
                }

                // Finally, notify and mark current as normal
                let acknowledger = Acknowledger::new_gd();
                communicator.signals().update_idx().emit(
                    current_idx as i32,
                    MazeTileState::Normal,
                    current_direction,
                    &acknowledger
                );
                // AWAIT
                let _await = acknowledger.signals().ok().to_fallible_future().await;
                // AWAIT
            }

            if let Some(score) = score_opt {
                let end_state = Direction::iter().map(|direction| {
                    let state = CoordinateAndDirecton {
                        coordinate : end_coordinate.clone(),
                        direction,
                    };

                    state

                }).min_by(|a, b| {
                    let a_cost = coordinate_state_to_cost.get(a).unwrap_or(&IMPLIED_UNVISITED_COST);
                    let b_cost = coordinate_state_to_cost.get(b).unwrap_or(&IMPLIED_UNVISITED_COST);

                    a_cost.cmp(&b_cost)
                }).unwrap();
                
                let paths = gd.bind().reconstruct_path_from_end_reversed(
                    end_state,
                    &key_got_here_from_value_map

                ).into_iter().map(|coordinate_and_directions| {
                    let mut coordinates = coordinate_and_directions.into_iter().map(|coordinate_and_direction| {
                        coordinate_and_direction.coordinate
                    }).collect::<Vec<_>>();

                    coordinates.reverse();

                    coordinates
                }).collect::<Vec<_>>();

                let path_info = PathInfo::new_gd(paths, score);

                communicator.signals().commit_found_path().emit(&path_info);
            };
        });

        interface
    }


    fn reconstruct_path_from_end_reversed(&self, end_state : CoordinateAndDirecton, key_got_here_from_value_map : &HashMap<CoordinateAndDirecton, HashSet<CoordinateAndDirecton>>) -> Vec<Vec<CoordinateAndDirecton>> {
        //println!("Backtracking from {}", end_coordinate.to_display_shifted());

        let mut path = vec![end_state.clone()];
        let mut current = end_state;

        while let Some(current_predecessors) = key_got_here_from_value_map.get(&current) {
            //println!("Predecessors of {} : {:#?}", current.to_display_shifted(), current_predecessors.iter().map(|orig| orig.to_display_shifted()).collect::<Vec<_>>());
            if current_predecessors.len() == 1 {
                // Single predecessor
                let predecessor = current_predecessors.iter().next().unwrap();

                path.push(predecessor.clone());
                current = predecessor.clone();

            } else {
                // Multiple predecessors
                let mut all_paths = Vec::with_capacity(current_predecessors.len());
                path.pop();
                for predecessor in current_predecessors {
                    let sub_paths = self.reconstruct_path_from_end_reversed(predecessor.clone(), key_got_here_from_value_map);
                    for sub_path in sub_paths {
                        let mut full_path = path.clone();
                        full_path.extend(sub_path.into_iter());
                        all_paths.push(full_path);
                    }
                }

                return all_paths;
            }
        }

        return vec![path];
    }


    fn heurestic_value(&self, coordinate : &Coordinate) -> usize {
        let vector = self.end_coordinate.clone() - coordinate.clone();
        let distance = vector.manhattan_distance();
        distance
    }


    pub fn rust_get_array(&self) -> & Vec<Tile> {
        &self.array
    }


    pub fn rust_get_dim_x(&self) -> usize {
        self.dim_x
    }


    pub fn rust_get_dim_y(&self) -> usize {
        self.dim_y
    }


    pub fn rust_get_n_walls(&self) -> usize {
        self.n_walls
    }
}


fn rotation_cost(rotations : usize) -> usize {
    let clamped = rotations % Direction::COUNT;
    match clamped {
        0 => 0,

        1 | 3 => ROTATION_COST,

        2 => 2 * ROTATION_COST,

        // Should never happen
        _ => {
            panic!("Got {} rotations - should not have been possible!", &clamped);
        }
    }
}


fn add_predecessor(target : CoordinateAndDirecton, new_predecessor : CoordinateAndDirecton, strictly_better_cost : bool, key_got_here_from_value_map : &mut HashMap<CoordinateAndDirecton, HashSet<CoordinateAndDirecton>>) {
    if target == new_predecessor {
        return;
    }

    if strictly_better_cost {
        key_got_here_from_value_map.insert(target, HashSet::from([new_predecessor]));

    } else {
        key_got_here_from_value_map.entry(target).or_insert(HashSet::new()).insert(new_predecessor);

    }
}


// ParsedTile

struct ParsedTile {
    tile : Tile,
    is_start_coordinate : bool,
    is_end_coordinate : bool,
}


impl TryFrom<&char> for ParsedTile {
    type Error = ();

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        let mut is_start_coordinate = false;
        let mut is_end_coordinate = false;

        let value = *value;        
        let tile = match value {
            '.' => {
                Tile::Ground
            },

            '#' => {
                Tile::Wall
            },

            'S' => {
                is_start_coordinate = true;
                Tile::Ground
            },

            'E' => {
                is_end_coordinate = true;
                Tile::Ground
            }

            _ => {
                return Err(());
            }            
        };

        let parsed = Self {
            tile,
            is_start_coordinate,
            is_end_coordinate,
        };

        Ok(parsed)
    }
}



// Tile

#[derive(Clone, PartialEq, Eq)]
pub enum Tile {
    Ground,
    Wall,
}


// From<&Tile> -> char

impl From<&Tile> for char {
    fn from(value : &Tile) -> Self {
        match &value {
            Tile::Ground => '.',
            Tile::Wall => '#',
        }
    }
}


// Reindeer

#[derive(Clone)]
struct Reindeer {
    direction : Direction,
}

impl Default for Reindeer {
    fn default() -> Self {
        let direction = Direction::East;
        Self {
            direction
        }
    }
}


// CoordinateAndDirection

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct CoordinateAndDirecton {
    coordinate : Coordinate,
    direction : Direction,
}


struct MazeDisplayer<'a> {
    maze : &'a Maze,
    path_coordinates : &'a HashSet<Coordinate>,
}


impl Display for MazeDisplayer<'_> {
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maze = &self.maze;

        let last_y = maze.dim_y - 1;

        for y in 0..maze.dim_y {
            for x in 0..maze.dim_x {
                let coordinate = Coordinate::new(x as isize, y as isize);

                let c = if self.path_coordinates.contains(&coordinate) {
                    'O'
                } else if let Some((_, tile)) = maze.get_index_and_content_by_coordinate(&coordinate) {
                    char::from(tile)
                } else {
                    '?'
                };

                f.write_char(c)?;
            }

            if y != last_y {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}
