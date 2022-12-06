use std::fs;

fn main() {

    /* 
        This list represents the Calories of the food carried by five Elves:

        The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
        The second Elf is carrying one food item with 4000 Calories.
        The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
        The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
        The fifth Elf is carrying one food item with 10000 Calories.
        In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).

        Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
    */

    // separate input by new lines
    // each empty space is new elf
    let file_path = "resources/input.txt";
    let contents  = fs::read_to_string(file_path)
        .expect("Could not read file.");
    
    /* PART 2 
    By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.

    To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.
    
    In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). The sum of the Calories carried by these three elves is 45000.
    
    Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
    */
    let mut calories_by_elf_vector: Vec<u64> = vec![];

    for number in contents.split("\n\n") {
        let sum_per_elf: u64 = number.lines().map(|line| line.parse::<u64>().unwrap()).sum();

        println!("Number.lines: {:?}", sum_per_elf);
        calories_by_elf_vector.push(sum_per_elf);
    }
    
    let highest_sum_elf = calories_by_elf_vector.iter().max().unwrap();
    println!("Highest sum: {highest_sum_elf}");

    calories_by_elf_vector.sort();
    let ans: u64 = calories_by_elf_vector.iter().rev().take(3).sum();
    println!("Top 3: {:?}", calories_by_elf_vector.iter().rev().take(3));
    
    println!("Sum of the sums: {:?}", ans);
    //println!("Sum of the sums mine: {:?}", sum_of_sums);

}