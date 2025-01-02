// --- Day 8: Resonant Collinearity ---

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
    let lines: Vec<&str> = include_str!("../input/08.in").lines().collect();

    let (maze, guard_position) = create_maze(lines.clone());
}
