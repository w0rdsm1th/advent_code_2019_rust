

use std::collections::{HashMap, HashSet};
use crate::utils;

use std::cmp::Ordering;


fn index_min(nets: &Vec<i32>) -> usize {
    // https://stackoverflow.com/a/53908709/3596968
    let index_of_max: Option<usize> = nets
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index);

    index_of_max.expect("failed to find index_min")
}

fn trace_wire_path(wire_instructions: String) -> Vec<HashMap<[i32; 2], i32>> {

    // value of type `std::collections::HashMap<char, (u32, u32)>` cannot be built from `std::iter::Iterator<Item=(&char, &(u32, u32))>`
    // let direction_keys: [char; 4] = ['U', 'D', 'R', 'L', ];
    // let cartesian_increments: [(u32, u32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0), ];
    // let direction_translation: HashMap<char, (i32, i32)> = direction_keys.iter().zip(cartesian_increments.iter()).collect();

    // // struggled to use convenience hashmap macro
    // "typical" way of initialising a HashMap, then use .insert(key, value)
    let mut direction_translation: HashMap<char, (i32, i32)> = HashMap::new();
    direction_translation.insert('U', (0, 1));
    direction_translation.insert('D', (0, -1));
    direction_translation.insert('R', (1, 0));
    direction_translation.insert('L', (-1, 0));
    println!("direction_translation HashMap {:?}", direction_translation);

    // trace wire path as manhattan grid points
    let mut out_vec = Vec::new();

    // https://stackoverflow.com/questions/29859892/mutating-an-item-inside-of-nested-loops
    'wire: for each_wire in wire_instructions.lines() {
        let mut current_xy_pos = [0, 0];  // x, y
        let mut loop_map = HashMap::new();
        let mut cumulative_wire_path = 0;

        'instructions: for instruction in each_wire.split(",") {
            // parsing the instructions: separate direction and distance
            // instructions of format "U32", "D240",
            let heading_key: char = instruction.chars().nth(0).expect("failed to extract first char from instruction");

            let heading = direction_translation.get(&heading_key).expect("unexpected key supplied to direction map");
            let distance = instruction[1..].parse::<i32>().expect("failed to parse");


            // have to track ALL positions that the wire "visits"
            for _ in 1..=distance {
                current_xy_pos[0] += heading.0;
                current_xy_pos[1] += heading.1;
                cumulative_wire_path += 1;

                // syntax for conditional hashmap gets
                // https://doc.rust-lang.org/stable/rust-by-example/std/hash/alt_key_types.html
                if !loop_map.contains_key(&current_xy_pos) {
                    loop_map.insert(current_xy_pos, cumulative_wire_path);
                };
            }
        }
        out_vec.push(loop_map);
    }
    out_vec
}

fn intersection_set<'a>(inp1: &'a HashSet<[i32; 2]>, inp2: &'a HashSet<[i32; 2]>) -> HashSet<&'a [i32; 2]> {
    // UNUSED in current answer because had to change from HashSet to HashMap in part2
    // issues with typing inside the HashSet so isolated this functionality
    // TODO - solved issues with explicit HashSet<_>
    let intersection_points: HashSet<_> = inp1.intersection(&inp2).collect();
    println!("debug, intersection_points: {:?}", intersection_points);
    intersection_points
}

fn intersection_loop(inp1: HashMap<[i32; 2], i32>, inp2: HashMap<[i32; 2], i32>) -> HashMap<[i32; 2], i32> {
    // re-working intersection function for part2, needs to work with Maps
    let mut intersection_points: HashMap<[i32; 2], i32> = HashMap::new();
    for wire_point in inp1.keys() {
        if inp2.contains_key(wire_point) {
            let w1_steps = *inp1.get(wire_point).expect("");
            let w2_steps = *inp2.get(wire_point).expect("");
            let combined_wire_length = w1_steps + w2_steps;
            println!("debug, combined wire steps at intersection {:?}, are {}, w1 {}, w2 {}", wire_point, combined_wire_length, w1_steps, w2_steps);
            intersection_points.insert(*wire_point, combined_wire_length);
        }
    }
    println!("debug, intersection_points: {:?}", intersection_points);
    intersection_points
}

fn find_intersection_points(wire_path_1: HashMap<[i32; 2], i32>, wire_path_2:  HashMap<[i32; 2], i32>) -> Vec<(i32, i32)> {
    // hashset intersection between the wire paths
    // find the minimum combined abs sum of their coordinates
    let intersection_points = intersection_loop(wire_path_1, wire_path_2);

    let total_distance: Vec<(i32, i32)> = intersection_points.iter().map(|x| (x.0[0].abs() + x.0[1].abs(), *x.1)).collect();
    total_distance
}

fn part_1(inp: String) -> i32 {

    let wire_paths = trace_wire_path(inp);
    let wire_path_1 = wire_paths[0].clone();
    let wire_path_2 = wire_paths[1].clone();
    // println!("debug, wire_paths: {:?}", wire_paths);

    let intersection_points = find_intersection_points(wire_path_1, wire_path_2);

    // let min_idx = index_min(&total_distance);
    // println!("closest intersection point {:?}", total_distance[min_idx]);
    let out = *intersection_points.iter().min_by_key(|x| x.0).expect("no minimum found");
    println!("closest intersection point is {:?} units from the central port.", out);
    out.0
}

fn part_2(inp: String) -> i32 {

    let wire_paths = trace_wire_path(inp);
    let wire_path_1 = wire_paths[0].clone();
    let wire_path_2 = wire_paths[1].clone();
    // println!("debug, wire_paths: {:?}", wire_paths);

    let intersection_points = find_intersection_points(wire_path_1, wire_path_2);

    // let min_idx = index_min(&total_distance);
    // println!("closest intersection point {:?}", total_distance[min_idx]);
    let out = *intersection_points.iter().min_by_key(|x| x.1).expect("no minimum found");
    println!("closest intersection point is {:?} units from the central port.", out);
    out.1
}


pub fn answer() {
    let inp: String = utils::input_reader::input_reader_raw_string("inp_day3.txt");
    // part_1(inp);
    part_2(inp);
}


#[test]
fn test_day3_part1_example() {
    let example = "R8,U5,L5,D3\n\
        U7,R6,D4,L4".to_string();
    let sample_output = part_1(example);
    assert_eq!(sample_output, 6 as i32)
}

#[test]
fn test_day3_part1_sample1() {
    let sample1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
        U62,R66,U55,R34,D71,R55,D58,R83".to_string();
    let sample_output = part_1(sample1);
    assert_eq!(sample_output, 159 as i32)
}

#[test]
fn test_day3_part1_sample2() {
    let sample2 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string();

    assert_eq!(part_1(sample2), 135)
}

#[test]
fn test_day3_part2_example() {
    let example = "R8,U5,L5,D3\n\
        U7,R6,D4,L4".to_string();
    let sample_output = part_2(example);
    assert_eq!(sample_output, 30 as i32)
}

#[test]
fn test_day3_part2_sample1() {
    let sample1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
        U62,R66,U55,R34,D71,R55,D58,R83".to_string();
    let sample_output = part_2(sample1);
    assert_eq!(sample_output, 610 as i32)
}

#[test]
fn test_day3_part2_sample2() {
    let sample2 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string();

    assert_eq!(part_2(sample2), 410)
}

#[test]
fn test_day3_part1() {
    // validate that haven't broken anything in trying to solve part 2 or later parts
    let inp: String = utils::input_reader::input_reader_raw_string("inp_day3.txt");
    assert_eq!(part_1(inp), 3247)
}

#[test]
fn test_day3_part2() {
    // validate that haven't broken anything in trying to solve part 2 or later parts
    let inp: String = utils::input_reader::input_reader_raw_string("inp_day3.txt");
    assert_eq!(part_2(inp), 48054)
}

/*LEARNINGS
- Lifetimes' names start with a single quote, ':
// named lifetimes:
fn print<'a>(x: &'a i32) {}
This allows returning references whose lifetime depend on the lifetime of the arguments.......
***LISTENING TO THE COMPLILER THIS WAS CAUSE OF DOWNFALL

- overusing vectors and not using Arrays (heterogeneous types, ) or Tuples (homogeneous types, immutable)

- handling lines() and Lines types better
https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines


// ------------------------------------------------------------------------------------
let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();

// Print 2, 3 in arbitrary order.
for x in a.intersection(&b) {
    println!("{}", x);
}

let intersection_points: HashSet<_> = a.intersection(&b).collect();
println!("got here! {:?}", intersection_points);
// ------------------------------------------------------------------------------------

- multiline string syntax, was preventing from inputting own test fixtures
If you want linebreaks in the string you can add them before the \:

let string = "multiple\n\
              lines\n\
              with\n\
              indentation";





*/