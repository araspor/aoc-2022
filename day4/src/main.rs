use std::fs;

#[derive(Debug)]
struct Range {
    start_index: i64,
    end_index: i64
}

fn main() {
    let file_path = "resources/input.txt";
    let contents  = fs::read_to_string(file_path)
        .expect("Could not read file.");

    /* DAY 4 
        So we have pairs of ranges 1-5,4-8 and we need to find ranges where one fully overlaps the other.
        For example: 2-8,3-7 => 3-7 is fully contained in 2-8
            .2345678.  2-8
            ..34567..  3-7
    
    ========= PART 1 =========
    Logic:
      (start_index1 >= start_index2 AND end_index1 <= end_index2) 
       OR
      (start_index2 >= start_index1 AND end_index2 <= end_index1)
      
      if that is true then range1 is contained withing range2 or range2 is contained within range1 -> counter + 1
    */
    
    // for each pair get start_index1, end_index1, start_index2 and end_index2
    // my goal is to get something like this: (Range1(x, y), Range2(x, y))
    // then filter out those meeting conditions
    // and count()
    let count_pairs: usize = contents.lines()
    .map(|line| line.split_once(",").unwrap())
    .map(|(range1, range2)| (
        range1.split_once("-").map(|(s, e)| Range {start_index: s.parse::<i64>().unwrap(), end_index: e.parse::<i64>().unwrap()}).unwrap(),
        range2.split_once("-").map(|(s, e)| Range {start_index: s.parse::<i64>().unwrap(), end_index: e.parse::<i64>().unwrap()}).unwrap()
    ))
    .filter(|(range1, range2)| (range1.start_index >= range2.start_index && range1.end_index <= range2.end_index) || (range2.start_index >= range1.start_index && range2.end_index <= range1.end_index))
    .count();

    println!("Pairs: {:?}", count_pairs);

    /* ========= PART 2 ========= */
    /* Logic: 
        start_index2 <= end_index1 AND (end_index2 >= end_index1 || end_index2 >= start_index1)

        if that is true then they overlap
        .23. 2-3
        12.. 1-2

        ..3. 3-3
        1... 1-2

        .23. 2-3
        1234 1-4
    */
    let count_overlaps: usize = contents.lines()
    .map(|line| line.split_once(",").unwrap())
    .map(|(range1, range2)| (
        range1.split_once("-").map(|(s, e)| Range {start_index: s.parse::<i64>().unwrap(), end_index: e.parse::<i64>().unwrap()}).unwrap(),
        range2.split_once("-").map(|(s, e)| Range {start_index: s.parse::<i64>().unwrap(), end_index: e.parse::<i64>().unwrap()}).unwrap()
    ))
    .filter(|(range1, range2)| range2.start_index <= range1.end_index && (range2.end_index >= range1.end_index || range2.end_index >= range1.start_index))
    .count();

    println!("Overlaps: {:?}", count_overlaps);
}
