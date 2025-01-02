// --- Day 6: Guard Gallivant ---

use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
enum MazeField {
    Empty,
    Obstacle,
    GuardUp,
    GuardDown,
    GuardLeft,
    GuardRight,
}

#[derive(Debug, Clone, PartialEq)]
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

fn turn_right(field: MazeField) -> MazeField {
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

fn print_maze(maze: &Vec<Vec<MazeField>>) {
    for line in maze {
        for field in line {
            match field {
                MazeField::Empty => print!("."),
                MazeField::Obstacle => print!("#"),
                MazeField::GuardUp => print!("^"),
                MazeField::GuardDown => print!("v"),
                MazeField::GuardRight => print!(">"),
                MazeField::GuardLeft => print!("<"),
            }
        }
        println!("")
    }
    println!("")
}

fn part_one(mut guard_position: GuardPosition, mut maze: Vec<Vec<MazeField>>) {
    println!("Part one: {:?}", guard_position);

    let maze_height = maze.len();
    let maze_width = maze[0].len();
    let mut distinct_positions_count = 0;
    let mut tmp_x: usize = guard_position.x.try_into().unwrap();
    let mut tmp_y: usize = guard_position.y.try_into().unwrap();
    let mut guard_direction = maze[tmp_y][tmp_x].clone();
    // print_maze(&maze);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let next_position = get_next_position(&guard_direction, &guard_position);

        if (next_position.x < 0 || next_position.x >= maze_width.try_into().unwrap()) ||
            (next_position.y < 0 || next_position.y >= maze_height.try_into().unwrap()) {
            break;
        }

        let next_x: usize = next_position.x.try_into().unwrap();
        let next_y: usize = next_position.y.try_into().unwrap();
        let next_field = &maze[next_y][next_x];

        tmp_x = guard_position.x.try_into().unwrap();
        tmp_y = guard_position.y.try_into().unwrap();

        // CASE 1: in front of the obstacle -> turn right
        if *next_field == MazeField::Obstacle {
            guard_direction = turn_right(guard_direction);
            maze[tmp_y][tmp_x] = guard_direction.clone();
        }
        // CASE 2: not in front of the obstacle -> walk further
        else if *next_field == MazeField::Empty {
            maze[tmp_y][tmp_x] = MazeField::Empty;
            guard_position = next_position;
            maze[next_y][next_x] = guard_direction.clone();
            visited.insert((next_y, next_x));
            // print_maze(&maze);
        }
    }

    println!("Part one {:?}", visited.len())
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

fn main() {
    let lines: Vec<&str> = include_str!("../input/06.in").lines().collect();

    let (maze, guard_position) = create_maze(lines.clone());

    part_one(maze, guard_position);
    // part_two(lines.clone());
}
