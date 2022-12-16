use std::fs;

#[derive(Debug, Clone)]
struct Ship {
    stacks: Vec<Stack>, // each index == y, Stack has x coord
    steps: Vec<Step>,   // vec[from] -> vec[to]; vec.pop(crates_to_move)
}

#[derive(Debug, Clone)]
struct Step {
    from: usize,           // from y=1
    to: usize,             // to y=3
    crates_to_move: usize, // move 3 crates
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn populate_stacks(crates_str: Vec<&str>) -> Vec<Stack> {
        let mut stacks_vec: Vec<Stack> = vec![Stack { crates: Vec::new() }; 9];

        // populate stacks vector
        crates_str.iter().for_each(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, char)| *char != ' ')
                .for_each(|(index, char)| stacks_vec[index].crates.push(char))
        });

        stacks_vec
    }
}

impl Step {
    fn from_str(str: &str) -> Result<Step, &str> {
        let str = str.trim();
        let mut words = str.split(' ');
        if words.next().unwrap() != "move" {
            return Err("No MOVE command found.");
        }
        let crates_to_move = words.next().unwrap().parse().unwrap();

        if words.next().unwrap() != "from" {
            return Err("No FROM command found.");
        }
        let from = words.next().unwrap().parse().unwrap();

        if words.next().unwrap() != "to" {
            return Err("No TO command found.");
        }
        let to = words.next().unwrap().parse().unwrap();

        Ok(Self {
            from,
            to,
            crates_to_move,
        })
    }
}

impl Ship {
    fn apply_steps_part1(&mut self) {
        for Step {
            from,
            to,
            crates_to_move,
        } in self.steps.iter()
        {
            // let mut crates_from: &mut Vec<char> = &mut self.stacks[*from - 1].crates;
            let src_stack: &mut Vec<char> = &mut self.stacks[*from - 1].crates;
            let mut stack_to_move: Vec<char> = src_stack
                .drain((src_stack.len() - crates_to_move)..)
                .rev()
                .collect();
            self.stacks[*to - 1].crates.append(&mut stack_to_move);
        }
    }

    fn apply_steps_part2(&mut self) {
        for Step {
            from,
            to,
            crates_to_move,
        } in self.steps.iter()
        {
            // let mut crates_from: &mut Vec<char> = &mut self.stacks[*from - 1].crates;
            let src_stack: &mut Vec<char> = &mut self.stacks[*from - 1].crates;
            let mut stack_to_move: Vec<char> = src_stack
                .drain((src_stack.len() - crates_to_move)..)
                .collect();
            self.stacks[*to - 1].crates.append(&mut stack_to_move);
        }
    }

    fn get_top_crates(self) -> Vec<char> {
        self.stacks
            .iter()
            .map(|stack| stack.crates.last().unwrap())
            .map(|char| *char)
            .collect()
    }
}

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Could not read file.");

    let crate_string: Vec<&str> = contents
        .lines()
        .filter(|line| line.contains('['))
        .rev()
        .collect();

    let steps_string: Vec<&str> = contents
        .lines()
        .filter(|line| line.contains("move"))
        .collect();

    let steps_vec: Vec<Step> = steps_string
        .iter()
        .map(|step| Step::from_str(step).unwrap())
        .collect();

    let mut ship_part1: Ship = Ship {
        stacks: Stack::populate_stacks(crate_string),
        steps: steps_vec,
    };

    let mut ship_part2: Ship = ship_part1.clone();

    ship_part1.apply_steps_part1();
    println!(":{:?}", ship_part1.get_top_crates());

    ship_part2.apply_steps_part2();
    println!(":{:?}", ship_part2.get_top_crates());
}

#[cfg(test)]
mod tests {
    use crate::{Ship, Stack, Step};

    #[test]
    fn test_apply_steps() {
        let step: Step = Step {
            from: 2,
            to: 1,
            crates_to_move: 3,
        };

        let stack_1: Stack = Stack {
            crates: vec!['A', 'B', 'C', 'D', 'E'],
        };

        let stack_2: Stack = Stack {
            crates: vec!['F', 'G', 'H', 'J', 'Z', 'W'],
        };

        let mut ship: Ship = Ship {
            stacks: vec![stack_1, stack_2],
            steps: vec![step],
        };

        ship.apply_steps_part1();
        assert_eq!(
            ship.stacks[0],
            Stack {
                crates: vec!['A', 'B', 'C', 'D', 'E', 'W', 'Z', 'J']
            }
        );
    }

    #[test]
    fn test_apply_steps_aoc_example() {
        let step_1: Step = Step {
            from: 2,
            to: 1,
            crates_to_move: 1,
        };

        let step_2: Step = Step {
            from: 1,
            to: 3,
            crates_to_move: 3,
        };

        let step_3: Step = Step {
            from: 2,
            to: 1,
            crates_to_move: 2,
        };

        let step_4: Step = Step {
            from: 1,
            to: 2,
            crates_to_move: 1,
        };

        let stack_1: Stack = Stack {
            crates: vec!['Z', 'N'],
        };

        let stack_2: Stack = Stack {
            crates: vec!['M', 'C', 'D'],
        };

        let stack_3: Stack = Stack { crates: vec!['P'] };

        let mut ship: Ship = Ship {
            stacks: vec![stack_1, stack_2, stack_3],
            steps: vec![step_1, step_2, step_3, step_4],
        };

        ship.apply_steps_part1();
        assert_eq!(
            ship.stacks,
            vec![
                Stack { crates: vec!['C'] },
                Stack { crates: vec!['M'] },
                Stack {
                    crates: vec!['P', 'D', 'N', 'Z']
                }
            ]
        );
        assert_eq!(ship.get_top_crates(), vec!['C', 'M', 'Z']);
    }

    #[test]
    fn test_steps_from_string_apply_aoc_example() {
        let step_1: Step = Step::from_str("move 1 from 2 to 1").unwrap();

        let step_2: Step = Step::from_str("move 3 from 1 to 3").unwrap();

        let step_3: Step = Step::from_str("move 2 from 2 to 1").unwrap();

        let step_4: Step = Step::from_str("move 1 from 1 to 2").unwrap();

        let stack_1: Stack = Stack {
            crates: vec!['Z', 'N'],
        };

        let stack_2: Stack = Stack {
            crates: vec!['M', 'C', 'D'],
        };

        let stack_3: Stack = Stack { crates: vec!['P'] };

        let mut ship: Ship = Ship {
            stacks: vec![stack_1, stack_2, stack_3],
            steps: vec![step_1, step_2, step_3, step_4],
        };

        ship.apply_steps_part1();
        assert_eq!(
            ship.stacks,
            vec![
                Stack { crates: vec!['C'] },
                Stack { crates: vec!['M'] },
                Stack {
                    crates: vec!['P', 'D', 'N', 'Z']
                }
            ]
        );
        assert_eq!(ship.get_top_crates(), vec!['C', 'M', 'Z']);
    }
}
