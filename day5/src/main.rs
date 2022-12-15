use std::fs;

#[derive(Debug, Default)]
struct Ship {
    stacks: Vec<Stack>, // each index == y, Stack has x coord
    steps: Vec<Step>,   // vec[from] -> vec[to]; vec.pop(crates_to_move)
}

#[derive(Debug)]
struct Step {
    from: usize,           // from y=1
    to: usize,             // to y=3
    crates_to_move: usize, // move 3 crates
}

#[derive(Debug)]
struct Stack {
    crates: Vec<char>,
}

impl Ship {
    fn apply_steps(&mut self) {
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
}

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Could not read file.");

    let step: Step = Step {
        from: 2,
        to: 1,
        crates_to_move: 3,
    };

    let stack_1: Stack = Stack {
        crates: vec!['A', 'B', 'C', 'D', 'E'],
    };

    println!("{:?}", stack_1);

    let stack_2: Stack = Stack {
        crates: vec!['F', 'G', 'H', 'J', 'Z', 'W'],
    };

    println!("{:?}", stack_2);

    let mut ship: Ship = Ship {
        stacks: vec![stack_1, stack_2],
        steps: vec![step],
    };

    ship.apply_steps();

    println!("{:?}", ship);
}
