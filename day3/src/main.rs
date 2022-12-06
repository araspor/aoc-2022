use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    
    /* Rucksack has 2 compartments, each item goes into exactly one compartment (based on category).
        The elf failed to follow rule to put each item type per exactly one compartment.
        PuzzleInput: list of all the items currently in each rucksack
        Task: find the errors
        Item types are identified by uppercase or lowercase letters (a and A refer to different item types)
        List of items for each rucksack is a single line of characters. The first half is in compartment 1, the second is in compartment 2.
        
        Example:
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw

            Rucksack 1 has items: vJrwpWtwJgWrhcsFMMfFFhFp
                compartment 1: vJrwpWtwJgWr
                compartment 2: hcsFMMfFFhFp
            The only item type that is in BOTH compartments is lowercase 'p'
            
            The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
            The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
            The fourth rucksack's compartments only share item type v.
            The fifth rucksack's compartments only share item type t.
            The sixth rucksack's compartments only share item type s.

        To help prioritize item rearrangement, every item type can be converted to a priority:
            Lowercase item types a through z have priorities 1 through 26.
            Uppercase item types A through Z have priorities 27 through 52.

        In the above example, the priority of the item type that appears in both compartments of each rucksack is 
            16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); 
        the sum of these is 157.

        Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
    */

    // task0: read input :^)
    let file = "resources/input.txt";
    let contents = fs::read_to_string(file)
        .expect("Couldn't read file.");

    let rucksacks: Vec<&str> = contents.lines().collect();

    let mut priorities = HashMap::new();
    for (c, i) in (('a' ..= 'z').chain('A' ..= 'Z')).zip(1 ..= 52) {
        println!("i: {i}, c: {c}");
        priorities.insert(c, i);
    }
    
    let mut sum_priorities: u32 = rucksacks.into_iter()
        // task1: split string into two halves
        .map(|rucksack| get_compartments(rucksack)) // split each rucksack into compartments
        // task2: find matching letters in each compartment
        .map(|(comp1, comp2)| get_matching_items((comp1, comp2))) // for each compartment find matching characters
        .flatten()
        // task3: calculate item type priority (a..z 1..26; A..Z 27..52)
        .map(|priority_char| priorities.get(&priority_char).copied().unwrap_or(0)) // get priorites for each character
        .sum();

    println!("SUM: {:?}", sum_priorities);
}

/* Split each rucksack into compartments */
fn get_compartments(input: &str) -> (&str, &str) {
    input.split_at(input.chars().count() / 2)
}

/* Match two strings and find matching string (case sensitive!) */
fn get_matching_items((comp1, comp2): (&str, &str)) -> HashSet<char> {
    comp1.chars()
        .filter(|c1| comp2.contains(*c1))
        .collect()
}