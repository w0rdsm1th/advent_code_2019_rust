


fn validate_candidate_pw_part_1(inp: u32) -> bool {
    // The value is within the range given in your puzzle input.
    // It is a six-digit number.

    // Two adjacent digits are the same (like 22 in 122345).
    // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    let mut two_same_adjacent = false;
    let mut never_decrease = true;


    let mut iter_inp = inp.clone();
    let mut prev_inp = inp / 10;

    while  iter_inp > 10 {
        let left_dig = prev_inp % 10;
        let right_dig = iter_inp % 10;

        if left_dig == right_dig {
            two_same_adjacent = true;
        }
        if left_dig > right_dig {
            never_decrease = false;
        }

        iter_inp = iter_inp / 10;
        prev_inp = prev_inp / 10;
    }

    // dont need string conversion, complicates things
    // let zipped_num = inp.to_string().chars().zip(inp.to_string().chars());
    // for (prev_digit, digit) in zipped_num {
    //     if prev_digit == digit {
    //         two_same_adjacent = true;
    //     }
    //     if prev_digit < prev_digit {
    //         never_decrease = false;
    //     }
    // }

    two_same_adjacent & never_decrease
}

fn validate_candidate_pw_part_2(inp: u32) -> bool {
    // The value is within the range given in your puzzle input.
    // It is a six-digit number.

    // Two adjacent digits are the same (like 22 in 122345).
    // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    let mut two_same_adjacent = false;
    let mut never_decrease = true;
    let mut iter_inp = inp.clone();
    let mut prev_inp = inp / 10;

    while  iter_inp > 10 {
        let left_dig = prev_inp % 10;
        let right_dig = iter_inp % 10;

        if left_dig > right_dig {
            never_decrease = false;
        }
        iter_inp = iter_inp / 10;
        prev_inp = prev_inp / 10;
    };

    // string convert for this extra logic because easier to then get pointer to single char
    let stringified = inp.to_string();
    // .collect() is slow, but can't think of easy alternative for this pattern
    // alternatives discussed below
    // https://stackoverflow.com/questions/38947231/iterating-through-a-window-of-a-string-without-collect
    for (idx, digit) in stringified.chars().collect::<Vec<char>>().windows(4).enumerate() {
        /* approved repeated scenarios:
        - right hand 2 digits (so index == 3)
        - left hand 2 digits (so index == 0)
        - else 'middle 2' digits in a window of 4 digits
        */

        // left hand 2 digits (so index == 0)
        if idx == 0 && (digit[0] == digit[1]) && (digit[1] != digit[2]) {
            two_same_adjacent = true;
        }
        // - right hand 2 digits (so index == 2)
        if idx == 2 && (digit[1] != digit[2]) && (digit[2] == digit[3]) {
            two_same_adjacent = true;
        }
        // - else 'middle 2' digits in a window of 4 digits
        else if (digit[1] == digit[2]) && (digit[0] != digit[1]) && (digit[2] != digit[3]) {
            two_same_adjacent = true;
        }
    }
    two_same_adjacent & never_decrease
}

fn part1() -> u32 {
    // not an inclusive range bc numbers themselves are not valid

    /*
    https://www.reddit.com/r/rust/comments/3rz4gu/for_match/cwsq4ex?utm_source=share&utm_medium=web2x

    Often flat_map, filter or take_while are good alternatives to a loop with inner if or match.
    */

    // easy Parallelize execution of independent tasks!!!
    // https://www.programming-idioms.org/cheatsheet/Rust
    // let mut out_count: u32 = search_space.into_par_iter().for_each(validate_candidate_pw);
    // let search_space = (206938..679128);
    // let mut out_count: u32 = (206938..679128).for_each(|x| validate_candidate_pw(x));
    let mut out_count = 0;
    for candidate in 206938..679128 {
        if validate_candidate_pw_part_1(candidate) {
            out_count += 1;
        }
    }
    println!("part 1 number of valid passwords in range {}", out_count);
    out_count
}

fn part2() -> u32 {
    // not an inclusive range bc numbers themselves are not valid
    let mut out_count = 0;
    for candidate in 206938..679128 {
        if validate_candidate_pw_part_2(candidate) {
            out_count += 1;
        }
    }
    println!("part 2 number of valid passwords in range {}", out_count);
    out_count
}

pub fn answer() {
    part1();
    part2();
}


#[test]
fn test_part_1_erroneous(){
    assert_eq!(validate_candidate_pw_part_1(207777), false)
}


#[test]
fn test_part_1_example(){
    assert_eq!(validate_candidate_pw_part_1(122345), true)
}

#[test]
fn test_part_1_sample_1(){
    // meets criteria
    assert_eq!(validate_candidate_pw_part_1(111111), true)
}

#[test]
fn test_part_1_sample_2(){
    // decreasing pair of digits
    assert_eq!(validate_candidate_pw_part_1(223450), false)
}

#[test]
fn test_part_1_sample_3(){
    // no double
    assert_eq!(validate_candidate_pw_part_1(123789), false)
}

#[test]
fn test_part_1(){
    assert_eq!(part1(), 1653)
}


#[test]
fn test_part_2_sample_1(){
    // meets criteria
    assert_eq!(validate_candidate_pw_part_2(112233), true)
}

#[test]
fn test_part_2_sample_2(){
    // no longer meets the criteria (the repeated 44 is part of a larger group of 444).
    assert_eq!(validate_candidate_pw_part_2(123444), false)
}

#[test]
fn test_part_2_sample_3(){
    // meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
    assert_eq!(validate_candidate_pw_part_2(111122), true)
}


/*LEARNINGS


*/
