// --- Day 7: Bridge Repair ---

#[derive(Clone)]
struct Equation {
    result: i64,
    sequence: Vec<i64>
}

impl Equation {
    pub fn is_sound(&mut self, operators: Vec<&str>) -> bool {
        // get all possible math equations
        for equation in self.generate_all_equations(&operators) {
            let mut equation_res = equation[0].parse::<i64>().unwrap();
            for i in (1..equation.len()).step_by(2) {
                let operator = equation[i].as_str();
                let b = equation[i+1].parse::<i64>().unwrap();
                equation_res = match operator {
                    "+" => equation_res + b,
                    "*" => equation_res * b,
                    "|" => format!("{}{}", equation_res, b).parse::<i64>().unwrap(),
                    _ => todo!()
                };
                if equation_res > self.result {
                    break;
                }
            }
            if equation_res == self.result {
                return true
            }
        }
        return false
    }

    fn generate_all_equations(&mut self, operators: &Vec<&str>) -> Vec<Vec<String>> {
        let mut equations: Vec<Vec<String>> = vec![vec![self.sequence[0].to_string()]];
        for i in 1..self.sequence.len() {
            let mut equations_updated: Vec<Vec<String>> = Vec::new();
            for j in 0..equations.len() {
                for &operator in operators {
                    let mut tmp_str_vec1 = equations[j].clone();
                    tmp_str_vec1.push(operator.to_string());
                    equations_updated.push(tmp_str_vec1);
                }
            }
            equations = equations_updated;
            for j in 0..equations.len() {
                equations[j].push(self.sequence[i].to_string())

            }
        }
        return equations;
    }
}

fn part_one(equations: Vec<Equation>) {
    let mut result_sum = 0;
    for mut equation in equations {
        if equation.is_sound(vec!["+", "*"]) {
            result_sum += equation.result;
        }
    }
    println!("Part one: {:?}", result_sum)
}

fn part_two(equations: Vec<Equation>) {
    let mut result_sum = 0;
    for mut equation in equations {
        if equation.is_sound(vec!["+", "*", "|"]) {
            result_sum += equation.result;
        }
    }
    println!("Part rw0tw0: {:?}", result_sum)
}


fn main() {
    let lines: Vec<Equation> = include_str!("../input/07.in")
        .lines()
        .map(|x| {
            let mut equation_str = x.split(':');
            let result: i64 = equation_str
                .next()
                .clone()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let sequence: Vec<i64> = equation_str
                .next()
                .clone()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            Equation { result, sequence }
        }).collect();


    part_one(lines.clone());
    part_two(lines.clone());
}
