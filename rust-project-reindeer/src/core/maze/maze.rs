use std::{cmp::Ordering, collections::{HashMap, HashSet}, fmt::Debug};

use godot::prelude::*;
use strum::{EnumCount, IntoEnumIterator};

use crate::core::{common::{communicator::Communicator, coordinate::{Coordinate, IHasCoordinates}, direction::Direction}, maze::{maze_find_paths_communicator::MazeFindPathsCommunicator, maze_solver_info::MazeSolverInfo, maze_tile_state::MazeTileState, path_info::PathInfo}};


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
    pub fn try_new_gd_from_str(s : &str) -> Result<Gd<Self>, NewMazeError> {
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

            let y = NewMazeError::parse_integer(dim_y)?; 
            dim_y += 1;

            let mut local_dim_x = 0;
            for c in line.chars() {
                let x = NewMazeError::parse_integer(local_dim_x)?;
                local_dim_x += 1;

                let line_and_column_index_opt = (|| {
                    let x = i32::try_from(x).ok()?;
                    let y = i32::try_from(y).ok()?;

                    Some((x, y))
                })();

                let parsed_tile_info = ParsedTile::try_from(&c).map_err(|_| {
                    NewMazeError {
                        error:format!("Unrecognized token '{}'!",c),
                        line_and_column_index_opt
                    }
                })?;

                let tile = parsed_tile_info.tile;
                let is_wall = tile == Tile::Wall;
                if is_wall {
                    n_walls += 1;
                }

                array.push(tile);

                if parsed_tile_info.is_start_coordinate {
                    if start_coordinate_opt.is_some() {
                        return Err(
                            NewMazeError {
                                error : "Found multiple start coordinates!".to_string(),
                                line_and_column_index_opt
                            }
                        );
                    }
                    start_coordinate_opt = Some(Coordinate::new(x, y));
                }
                if parsed_tile_info.is_end_coordinate {
                    if end_coordinate_opt.is_some() {
                        return Err(
                            NewMazeError {
                                error : "Found multiple end coordinates!".to_string(),
                                line_and_column_index_opt
                            }
                        );
                    }
                    end_coordinate_opt = Some(Coordinate::new(x, y));
                }

            }

            if dim_x == 0 {
                dim_x = local_dim_x;

            } else if local_dim_x != dim_x {
                return Err(
                    NewMazeError {
                        error : format!("Misalignment error! Line {} has {} chars, but previous one(s) have {}!", y + 1, local_dim_x, dim_x),
                        line_and_column_index_opt : None,
                    }
                );
            }
        }

        let start_coordinate = start_coordinate_opt.ok_or_else(|| {
            NewMazeError {
                error : "Found no start coordinate!".to_string(),
                line_and_column_index_opt : None
            }
        })?;
        let end_coordinate = end_coordinate_opt.ok_or_else(|| {
            NewMazeError {
                error : "Found no end coordinate!".to_string(),
                line_and_column_index_opt : None
            }
        })?;

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

        Ok(gd)
    }


    #[func]
    pub fn find_paths(&self, maze_solver_info : Gd<MazeSolverInfo>, cost_per_rotation : u32) -> Gd<MazeFindPathsCommunicator> {
        const REMEMBER_BEST_PATH : bool = true;
        const IMPLIED_UNVISITED_COST : usize = usize::MAX;

        let map_dim_x = self.dim_x;
        let map_dim_y = self.dim_y;

        let gd = self.to_gd();

        let reindeer = self.reindeer.clone();
        let start_coordinate = self.start_coordinate.clone();
        let end_coordinate = self.end_coordinate.clone();

        let waiting_flag_bits = maze_solver_info.bind().wait_on_state;

        let communicator = MazeFindPathsCommunicator::new_gd();
        let interface = communicator.clone();

        let rotations_are_free = cost_per_rotation == 0;

        godot::task::spawn(async move {
            // AWAIT
            let _await = communicator.signals().start().to_fallible_future().await;
            // AWAIT

            let start_direction = reindeer.direction;

            let start_state = CoordinateAndDirecton {
                coordinate : start_coordinate.clone(),
                direction : start_direction.clone(),
            };

            let start_idx = start_state.coordinate.try_to_index(map_dim_x, map_dim_y).unwrap();

            // AWAIT
            let _await = Self::wait_for_update_idx(
                communicator.clone(),
                start_idx,
                start_direction,
                MazeTileState::Committed,
                waiting_flag_bits
            ).await;
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
                // AWAIT
                let _await = Self::wait_for_update_idx(
                    communicator.clone(),
                    current_idx,
                    current_direction,
                    MazeTileState::Active,
                    waiting_flag_bits
                ).await;
                // AWAIT

                // Unwrapping this as if this is an option then I have done something wrong.
                let cost_to_get_here = *coordinate_state_to_cost.get(&current).unwrap();

                if current_coordinate == end_coordinate {
                    let previous_score = score_opt.unwrap_or(usize::MAX);
                    if cost_to_get_here < previous_score {
                        score_opt = Some(cost_to_get_here);
                    }
                }

                // Valid candidates and costs depend on rotation cost - see below:
                let valid_candidates_and_costs = if rotations_are_free {
                    // Special scenario - rotation cost is 0.
                    // Tracking predecessors when rotation cost is 0 will result in an infinite loop under normal circumstances. 
                    // There will always be a "free" action of moving from rotation A to B.
                    // Fixing this by skipping rotations and dealing with all neighbors instead regardless of orientation.

                    let results = Direction::iter().filter_map(|direction| {
                        let move_target = current_coordinate.clone() + direction.to_vector();

                        let bound = gd.bind();
                        let (_, tile) = bound.get_index_and_content_by_coordinate(&move_target)?;
                        if tile != &Tile::Ground {
                            return None;
                        }

                        let target = CoordinateAndDirecton {
                            coordinate : move_target,
                            direction
                        };

                        let target_cost = 1;

                        Some((target, target_cost))

                    }).collect::<Vec<_>>();

                    results
                    
                } else {
                    // Generic scenario (rotations are not free) - we can either continue in our current direction or rotate.

                    let mut results = Vec::new();

                    // Move options:
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

                        let target_cost = Self::rotation_cost(rotation_steps, cost_per_rotation);

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

                    // Notify touched.
                    // AWAIT
                    let _await = Self::wait_for_update_idx(
                        communicator.clone(),
                        candidate_index,
                        current_direction,
                        MazeTileState::Touched,
                        waiting_flag_bits
                    ).await;
                    // AWAIT

                    let cost_to_move_here_from_current = cost_to_get_here + (usize::try_from(additional_cost).unwrap());
                    let cost_to_move_here_otherwise = coordinate_state_to_cost.get(&candidate).unwrap_or(&IMPLIED_UNVISITED_COST);
        
                    let cost_to_move_cmp = cost_to_move_here_from_current.cmp(&cost_to_move_here_otherwise);
                    
                    if cost_to_move_cmp == Ordering::Equal && additional_cost != 0 {                       
                        Self::add_predecessor(candidate.clone(), current.clone(), false, &mut key_got_here_from_value_map);
                    }
        
                    let skip_overshooting_cost = REMEMBER_BEST_PATH && cost_to_move_here_from_current > score_opt.unwrap_or(IMPLIED_UNVISITED_COST);
                    if cost_to_move_cmp != Ordering::Less || skip_overshooting_cost {
                        // Notify non-committed by reverting to default state.

                        // AWAIT
                        let _await = Self::wait_for_update_idx(
                            communicator.clone(),
                            candidate_index,
                            candidate_direction.clone(),
                            MazeTileState::Normal,
                            waiting_flag_bits
                        ).await;
                        // AWAIT

                        continue;
                    }
        
                    // Else, we should have found new optimal path.

                    // Notify candidate committed.
                    // AWAIT
                    let _await = Self::wait_for_update_idx(
                        communicator.clone(),
                        candidate_index,
                        candidate_direction.clone(),
                        MazeTileState::Committed,
                        waiting_flag_bits
                    ).await;
                    // AWAIT

                    coordinate_state_to_cost.insert(candidate.clone(), cost_to_move_here_from_current);
        
                    to_visit.push(candidate);
        
                    Self::add_predecessor(candidate, current, true, &mut key_got_here_from_value_map);

                    any_new_coordinate_added = true;
                }

                // If any coordinates were added, sort them by heurestic (lower is closer).
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

                // Finally, notify and mark current as normal.
                // AWAIT
                let _await = Self::wait_for_update_idx(
                    communicator.clone(),
                    current_idx,
                    current_direction,
                    MazeTileState::Normal,
                    waiting_flag_bits
                ).await;
                // AWAIT
            }

            // Either path is found or all coordinates have been exhausted.
            let path_info_opt = score_opt.map(|score| {
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
                    let mut coordinates = Vec::with_capacity(coordinate_and_directions.len());
                    for coordinate_and_direction in coordinate_and_directions {
                        let coordinate = coordinate_and_direction.coordinate;

                        let exists = coordinates.last().map_or(
                            false,
                            |last| {
                                &coordinate == last
                            }
                        );
                        
                        if !exists {
                            coordinates.push(coordinate);
                        }
                    }
                    
                    coordinates.reverse();

                    coordinates
                }).collect::<Vec<_>>();

                let path_info = PathInfo::new_gd(paths, score);
                path_info
            });

            communicator.signals().commit_finished().emit(path_info_opt.as_ref());
        });

        interface
    }


    fn reconstruct_path_from_end_reversed(&self, end_state : CoordinateAndDirecton, key_got_here_from_value_map : &HashMap<CoordinateAndDirecton, HashSet<CoordinateAndDirecton>>) -> Vec<Vec<CoordinateAndDirecton>> {
        let mut path = vec![end_state.clone()];
        let mut current = end_state;

        while let Some(current_predecessors) = key_got_here_from_value_map.get(&current) {
            if current_predecessors.len() == 1 {
                // Single predecessor:
                let predecessor = current_predecessors.iter().next().unwrap();

                path.push(predecessor.clone());
                current = predecessor.clone();

            } else {
                // Multiple predecessors:
                let mut all_paths = Vec::with_capacity(current_predecessors.len());
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


    pub fn rust_get_reindeer_start_coordinate(&self) -> &Coordinate {
        &self.start_coordinate
    }


    pub fn rust_get_end_coordinate(&self) -> &Coordinate {
        &self.end_coordinate
    }


    fn rotation_cost(rotations : usize, cost_per_rotation : u32) -> u32 {
        let clamped = rotations % Direction::COUNT;
        match clamped {
            0 => 0,

            1 | 3 => cost_per_rotation,

            2 => 2 * cost_per_rotation,

            // Should never happen.
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


    async fn wait_for_update_idx<T>(
        communicator : Gd<MazeFindPathsCommunicator>,
        idx : T,
        direction : Direction,
        maze_tile_state : MazeTileState,
        waiting_flag_bits : u32,
    )
    where
    T : TryInto<i32>,
    <T as TryInto<i32>>::Error : Debug,
    {
        let paused = communicator.bind().get_paused();
        if paused {
            // AWAIT
            let _await = communicator.signals().unpaused().to_fallible_future().await;
            // AWAIT
        }

        let acknowledger = Communicator::new_gd();
        communicator.signals().update_idx().emit(
            idx.try_into().unwrap(),
            maze_tile_state,
            direction,
            &acknowledger
        );
        if maze_tile_state.is_set_flag_in_bits(waiting_flag_bits) {
            // AWAIT
            let future_opt = acknowledger.bind().get_done_future();
            if let Some(future) = future_opt {
                future.await;
            }
            // AWAIT
        }
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

// ^- Not to be confused with the public Godot Reindeer.
 
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct CoordinateAndDirecton {
    coordinate : Coordinate,
    direction : Direction,
}


// NewMazeError

pub struct NewMazeError {
    pub error : String,
    pub line_and_column_index_opt : Option<(i32, i32)>,
}


impl NewMazeError {
    fn parse_integer<T>(to_parse : T) -> Result<isize, Self>
    where isize : TryFrom<T>
    {
        isize::try_from(to_parse).map_err(|_| {
            Self {
                error : "Integer error: Maze is too large??".to_string(),
                line_and_column_index_opt : None
            }
        })
    }
}
