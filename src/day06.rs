// --- Day 6: Guard Gallivant ---

use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MazeField {
    Empty,
    Obstacle,
    GuardUp,
    GuardDown,
    GuardLeft,
    GuardRight,
}

impl MazeField {
    fn as_str(&self) -> &'static str {
        match self {
            MazeField::Empty => ".",
            MazeField::Obstacle => "#",
            MazeField::GuardUp => "^",
            MazeField::GuardDown => "v",
            MazeField::GuardRight => ">",
            MazeField::GuardLeft => "<",
        }
    }

}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct GuardPosition {
    x: i32,
    y: i32,
}

impl GuardPosition {
    fn empty() -> GuardPosition {
        return GuardPosition { x: i32::MAX, y: i32::MAX };
    }
}

fn is_guard_maze_field(field: MazeField) -> bool {
    return field == MazeField::GuardDown || field == MazeField::GuardUp || field == MazeField::GuardLeft || field == MazeField::GuardRight;
}

fn turn_right(field: &MazeField) -> MazeField {
    return match field {
        MazeField::GuardLeft => MazeField::GuardUp,
        MazeField::GuardUp => MazeField::GuardRight,
        MazeField::GuardRight => MazeField::GuardDown,
        MazeField::GuardDown => MazeField::GuardLeft,
        _ => todo!()
    };
}

fn get_next_position(guard_direction: &MazeField, guard_position: &GuardPosition) -> GuardPosition {
    let mut x_diff = 0;
    let mut y_diff = 0;
    match *guard_direction {
        MazeField::GuardUp => { y_diff = -1;},
        MazeField::GuardDown => { y_diff = 1;},
        MazeField::GuardLeft => { x_diff = -1;},
        MazeField::GuardRight => { x_diff = 1;},
        _ => todo!()
    }
    return GuardPosition {x: guard_position.x + x_diff, y: guard_position.y + y_diff}
}

fn print_maze(maze: &Vec<Vec<MazeField>>, guard_visited_position_fields: &HashSet<(GuardPosition, MazeField)>, guard_position: &GuardPosition, guard_direction: &MazeField) {
    let guard_visited_fields: HashSet<GuardPosition> =
        guard_visited_position_fields
            .iter()
            .map(|(position, _)| position.clone())
            .collect();

    for (y, line) in maze.iter().enumerate() {
        for (x, field) in line.iter().enumerate() {
            // print maze field where the guard is currently located
            // print maze field which has been visited by the guard
            if guard_visited_fields.contains(&GuardPosition { x: x.try_into().unwrap(), y: y.try_into().unwrap() }) {
                // print | if the MazeField is visited up or down only
                // print - if the MazeField is visited left or right only
                // print + if the MazeField is visited in both directions
                let visited_fields = guard_visited_position_fields.iter().filter(|(position, _)| position.x == x.try_into().unwrap() && position.y == y.try_into().unwrap());

                let mut visited_up = false;
                let mut visited_down = false;
                let mut visited_left = false;
                let mut visited_right = false;
                for (_, field) in visited_fields {
                    match field {
                        MazeField::GuardUp => visited_up = true,
                        MazeField::GuardDown => visited_down = true,
                        MazeField::GuardLeft => visited_left = true,
                        MazeField::GuardRight => visited_right = true,
                        _ => todo!()
                    }
                }

                if (visited_up || visited_down) && (visited_left || visited_right) {
                    print!("+");
                } else if visited_up || visited_down {
                    print!("|");
                } else if visited_left || visited_right {
                    print!("-");
                }

            } else {
                print!("{}", field.as_str());
            }
        }
        println!("")
    }
    println!("")

}

fn create_maze(lines: Vec<&str>) -> (GuardPosition, Vec<Vec<MazeField>>) {
    let mut maze: Vec<Vec<MazeField>> = Vec::new();
    let mut guard_position: GuardPosition = GuardPosition::empty();
    for (i, &line) in lines.iter().enumerate() {
        let mut maze_line: Vec<MazeField> = Vec::new();
        for (j, maze_field_str) in line.chars().enumerate() {
            let maze_field = match maze_field_str {
                '.' => MazeField::Empty,
                '#' => MazeField::Obstacle,
                '^' => MazeField::GuardUp,
                'v' => MazeField::GuardDown,
                '>' => MazeField::GuardRight,
                '<' => MazeField::GuardLeft,
                _ => todo!(),
            };
            if is_guard_maze_field(maze_field.clone()) {
                guard_position = GuardPosition { x: j.try_into().unwrap(), y: i.try_into().unwrap() };
            }
            maze_line.push(maze_field);
        }
        maze.push(maze_line.clone())
    }
    return (guard_position, maze);
}

fn move_guard_to_next_position(
    guard_position: &mut GuardPosition,
    guard_direction: &mut MazeField,
    guard_visited_fields: &mut HashSet<GuardPosition>,
    maze: &mut Vec<Vec<MazeField>>,
    maze_width: usize,
    maze_height: usize,
) -> bool {
    let next_position = get_next_position(&guard_direction, &guard_position);

    if (next_position.x < 0 || next_position.x >= maze_width.try_into().unwrap()) ||
        (next_position.y < 0 || next_position.y >= maze_height.try_into().unwrap()) {
        *guard_position = GuardPosition{y: next_position.y, x: next_position.x };
        return false;
    }

    let guard_next_x: usize = next_position.x.try_into().unwrap();
    let guard_next_y: usize = next_position.y.try_into().unwrap();
    let next_field = &maze[guard_next_y][guard_next_x];

    let guard_x: usize = guard_position.x.try_into().unwrap();
    let guard_y: usize = guard_position.y.try_into().unwrap();
    // CASE 1: in front of the obstacle -> turn right
    if *next_field == MazeField::Obstacle {
        *guard_direction = turn_right(guard_direction);
        maze[guard_y][guard_x] = guard_direction.clone();
        return true;
    }
    // CASE 2: not in front of the obstacle -> walk further
    else if *next_field == MazeField::Empty {
        maze[guard_y][guard_x] = MazeField::Empty;
        guard_position.x = next_position.x;
        guard_position.y = next_position.y;
        maze[guard_next_y][guard_next_x] = guard_direction.clone();
        guard_visited_fields.insert(GuardPosition{y: next_position.y, x: next_position.x });
        // print_maze(&maze, &guard_visited_fields);
        return true;
    } else {
        // raise exception - unhandled case
        panic!("Unhandled case");
    }
}

fn part_one(mut guard_position: GuardPosition, mut maze: Vec<Vec<MazeField>>) -> HashSet<GuardPosition> {
    let maze_height = maze.len();
    let maze_width = maze[0].len();
    let mut distinct_positions_count = 0;
    let guard_x: usize = guard_position.x.try_into().unwrap();
    let guard_y: usize = guard_position.y.try_into().unwrap();
    let mut guard_direction = maze[guard_y][guard_x].clone();
    let mut guard_visited_fields: HashSet<GuardPosition> = HashSet::new();
    loop {
        let is_guard_in_bounds = move_guard_to_next_position(
            &mut guard_position,
            &mut guard_direction,
            &mut guard_visited_fields,
            &mut maze,
            maze_width,
            maze_height
        );
        if !is_guard_in_bounds {
            break;
        }
    }

    println!("Part one {:?}", guard_visited_fields.len());

    return guard_visited_fields;
}

fn part_two(guard_position: GuardPosition, mut maze: Vec<Vec<MazeField>>, guard_visited_fields: HashSet<GuardPosition>) {
    let maze_height = maze.len();
    let maze_width = maze[0].len();
    let mut distinct_positions_count = 0;
    let guard_x: usize = guard_position.x.try_into().unwrap();
    let guard_y: usize = guard_position.y.try_into().unwrap();
    let mut guard_direction = maze[guard_y][guard_x].clone();
    let mut time_paradox_obstruction_count = 0;

    for guard_path in guard_visited_fields {
        let path_x: usize = guard_path.x.try_into().unwrap();
        let path_y: usize = guard_path.y.try_into().unwrap();
        // skip the guard position
        if path_x == guard_x && path_y == guard_y {
            continue;
        }

        // println!("placing obstacle at {:?}", guard_path);

        let mut tmp_maze = maze.clone();
        let mut tmp_guard_position = guard_position.clone();
        let mut tmp_guard_direction = guard_direction.clone();
        let mut tmp_guard_visited_fields: HashSet<GuardPosition> = HashSet::new();
        let mut tmp_guard_visited_fields_directions: HashSet<(GuardPosition, MazeField)> = HashSet::new();

        // print_maze(&tmp_maze, &tmp_guard_visited_fields_directions, &tmp_guard_position, &tmp_guard_direction);
        tmp_maze[path_y][path_x] = MazeField::Obstacle;
        // print_maze(&tmp_maze, &tmp_guard_visited_fields_directions, &tmp_guard_position, &tmp_guard_direction);
        tmp_guard_visited_fields_directions.insert((tmp_guard_position.clone(), tmp_guard_direction.clone()));

        loop {
            let is_guard_in_bounds = move_guard_to_next_position(
                &mut tmp_guard_position,
                &mut tmp_guard_direction,
                &mut tmp_guard_visited_fields,
                &mut tmp_maze,
                maze_width,
                maze_height
            );
            // print_maze(&tmp_maze, &tmp_guard_visited_fields_directions, &tmp_guard_position, &tmp_guard_direction);
            let current_guard_field_direction: (GuardPosition, MazeField)  = (tmp_guard_position.clone(), tmp_guard_direction.clone());
            if tmp_guard_visited_fields_directions.contains(&current_guard_field_direction) {
                time_paradox_obstruction_count += 1;
                break;
            }
            tmp_guard_visited_fields_directions.insert(current_guard_field_direction);
            if !is_guard_in_bounds {
                break;
            }
        }
    }

    println!("Part two {:?}", time_paradox_obstruction_count);
}

fn main() {
    let lines: Vec<&str> = include_str!("../input/06.in").lines().collect();

    let (maze, guard_position) = create_maze(lines.clone());

    let guard_visited_fields = part_one(maze.clone(), guard_position.clone());
    part_two(maze.clone(), guard_position.clone(), guard_visited_fields.clone());
}
