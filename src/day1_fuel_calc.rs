/*

Specifically, to find the fuel required for a module, take its mass,
divide by three, round down, and subtract 2.
*/

use math::round;
use crate::utils;


fn fuel_calculator(mass: &i32) -> i32 {
    let mass_div3 = (mass / 3) as f64;  // round inputs are f64
    round::floor(mass_div3, 0) as i32 - 2
}

fn inclusive_fuel_calculator(mass: &i32) -> i32 {
    // considers the weight of additional fuel in the total fuel calculations
    let mut total_fuel = fuel_calculator(mass);
    let mut additional_fuel: i32 = total_fuel.clone();

    loop {
        additional_fuel = fuel_calculator(&additional_fuel);
        if additional_fuel <= 0 {
            break;
        }
        total_fuel += additional_fuel;
    }
    total_fuel
}

pub fn answer() {
    println!("started day 1");
    let inp = utils::input_reader::input_reader_lines_convert_int("inp_day1.txt");
    println!("length of inp is {}", inp.len());
    let part_1_fuel_calcs: Vec<i32> = inp.iter().map(|x: &i32| fuel_calculator(x)).collect();
    let part_1_sum: i32 = part_1_fuel_calcs.iter().sum();
    println!("finished day 1, part 1 answer {}", part_1_sum);


    /*-----------------------------------------------*/
    let part_2_fuel_calcs: Vec<i32> = inp.iter().map(|x: &i32| inclusive_fuel_calculator(x)).collect();
    let part_2_sum: i32 = part_2_fuel_calcs.iter().sum();
    println!("finished day 1, part 2 answer {}", part_2_sum);

}


#[test]
fn test_fuel_calc() {
    let test_val_weight = 100756;
    let test_val_fuel = 33583;

    println!("fuel calcs test for weight '{}', fuel needed '{}'", test_val_weight, fuel_calculator(&test_val_weight));
    assert_eq!(test_val_fuel, fuel_calculator(&test_val_weight))
}

#[test]
fn test_inclusive_fuel_calc() {
    let cases = vec![(14, 2),
                     (1969, 966),
                     (100756, 50346),
                     // (,),
    ];

    for (test_val_weight, test_val_fuel) in cases {
        println!("inclusive fuel calcs test for weight '{}', fuel needed '{}'", test_val_weight, inclusive_fuel_calculator(&test_val_weight));
        assert_eq!(test_val_fuel, inclusive_fuel_calculator(&test_val_weight))
    }
}

/* LEARNINGS
- str.parse()
https://doc.rust-lang.org/std/primitive.str.html#method.parse
Parses this string slice into another type.
Because parse is so general, it can cause problems with type inference. As such, parse is one of the few times you'll see the syntax affectionately known as the 'turbofish': ::<>. This helps the inference algorithm understand specifically which type you're trying to parse into.

- .clone() method

- was using u32 for part 1, but then part 2 introduced negative return values
thread 'main' panicked at 'attempt to subtract with overflow', src\day1 - fuel_calc:40:51

*/