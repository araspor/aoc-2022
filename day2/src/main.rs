use std::fs;

pub trait CalcPoints {
    fn calc_points(self, opponent: HandShape) -> u64;
}

#[derive(PartialEq, Debug)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
    Unknown
}

impl CalcPoints for HandShape {
    fn calc_points(self, opponent: HandShape) -> u64 {
        if self == HandShape::Rock {
            if opponent == HandShape::Rock {
                1 + 3
            }
            else if opponent == HandShape::Paper {
                1 + 0
            }
            else if opponent == HandShape::Scissors {
                1 + 6
            }
            else {
                0
            }
        }
        else if self == HandShape::Paper {
            if opponent == HandShape::Rock {
                2 + 6
            }
            else if opponent == HandShape::Paper {
                2 + 3
            }
            else if opponent == HandShape::Scissors {
                2 + 0
            }
            else {
                0
            }
        }
        else if self == HandShape::Scissors {
            if opponent == HandShape::Rock {
                3 + 0
            }
            else if opponent == HandShape::Paper {
                3 + 6
            }
            else if opponent == HandShape::Scissors {
                3 + 3
            }
            else
            {
                0
            }
        }
        else {
            0
        }
    }
}

pub fn get_shape(code: &str) -> HandShape {
    if code == "A" || code == "X" {
        HandShape::Rock
    }
    else if code == "B" || code == "Y" {
        HandShape::Paper
    }
    else if code == "C" || code == "Z" {
        HandShape::Scissors
    }
    else {
        HandShape::Unknown
    }
}

/*
    X - lose
    Y - draw
    Z - win
*/
pub fn get_shape_for_result(opponent: HandShape, code: &str) -> HandShape {
    if code == "X" {
        if opponent == HandShape::Rock {
            HandShape::Scissors
        }
        else if opponent == HandShape::Paper {
            HandShape::Rock
        }
        else if opponent == HandShape::Scissors {
            HandShape::Paper
        }
        else {
            HandShape::Unknown    
        } 
    }
    else if code == "Y" {
        if opponent == HandShape::Rock {
            HandShape::Rock
        }
        else if opponent == HandShape::Paper {
            HandShape::Paper
        }
        else if opponent == HandShape::Scissors {
            HandShape::Scissors
        }
        else {
            HandShape::Unknown    
        } 
    }
    else if code == "Z" {
        if opponent == HandShape::Rock {
            HandShape::Paper
        }
        else if opponent == HandShape::Paper {
            HandShape::Scissors
        }
        else if opponent == HandShape::Scissors {
            HandShape::Rock
        }
        else {
            HandShape::Unknown    
        } 
    }
    else {
        HandShape::Unknown
    }
}


fn main() {
    let file_path = "resources/input.txt";
    let contents  = fs::read_to_string(file_path)
        .expect("Could not read file.");

    let test = get_shape("Z").calc_points(get_shape("C"));
    println!("points: {test}");

    /*let test2: u64 = contents.split("\n")
        .map(|line| line.split_whitespace().collect().iter()
            .get_shape(line[0]).calc_points(line[1])
        ).sum();*/

    let mut sum_points_part1 = 0;
    let mut sum_points_part2 = 0;
    for line in contents.split("\n") {
        let lines: Vec<&str> = line.split_whitespace().collect();    
        
        let points_part1 = get_shape(lines[1]).calc_points(get_shape(lines[0]));
        let points_part2 = get_shape_for_result(get_shape(lines[0]), lines[1]).calc_points(get_shape(lines[0]));
        sum_points_part1 += points_part1;
        sum_points_part2 += points_part2;
        //let shapes: Vec<&str> = lines.iter().map(|line| line.split_whitespace()).collect();

        
    }
    
    println!("points : {:?}", sum_points_part1);
    println!("points : {:?}", sum_points_part2);
       //let shapes_split = shapes.split_whitespace();
        //let points = get_shape(shapes_split[0]).calc_points(get_shape(shapes_split[1]));

    /*
        --- Part Two ---
        The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

        The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

        In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
        In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
        In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
        Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

        Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
    */



}
