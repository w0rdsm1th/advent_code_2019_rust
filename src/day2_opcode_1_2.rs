/*
Opcode 1 adds together numbers read from two positions and stores the result in a third position.
The three integers immediately after the opcode tell you these three positions -
the first two indicate the positions from which you should read the input values,
and the third indicates the position at which the output should be stored.

Opcode 2 works exactly like opcode 1, except it multiplies the two inputs
instead of adding them. Again, the three integers after the opcode
indicate where the inputs and outputs are, not their values.
*/
use crate::utils;

fn opcode_1(a: usize, b: usize, pos: usize, mut opcode: Vec<i32>) -> Vec<i32> {
    opcode[pos] = opcode[a] + opcode[b];
    opcode
}

fn opcode_2(a: usize, b: usize, pos: usize, mut opcode: Vec<i32>) -> Vec<i32> {
    opcode[pos] = opcode[a] * opcode[b];
    opcode
}

pub fn opcode_computer(inp: &Vec<i32>) -> Vec<i32>{
    let mut out = inp.clone();

    // slice 4 at a time
    let mut instruction_pointer: usize = 0;
    loop {
        let opcode = out[instruction_pointer];
        if opcode == 99 {
            break
        }
        let arg_idx_1 = out[instruction_pointer + 1] as usize;
        let arg_idx_2 = out[instruction_pointer + 2] as usize;
        let out_pos= out[instruction_pointer + 3] as usize;

        // match opcode into different functions to process
        match opcode {
            1 => out = opcode_1(arg_idx_1, arg_idx_2, out_pos, out),
            2 => out = opcode_2(arg_idx_1, arg_idx_2, out_pos, out),
            // 99 => break,
            _ => panic!("unexpected opcode")
        }
        instruction_pointer += 4;
    }
    out
}

pub fn answer() {
    let inp = utils::input_reader::input_reader_commas("inp_day2.txt");

    // before running the program, replace
    //  position 1 with the value 12 and
    //  replace position 2 with the value 2
    // inp[1] = 12;
    // inp[2] = 2;

    // let output = opcode_computer(inp);
    // println!("position 0 value: {}", output[0]);
    // --------------------------------------------------------------------------------
    // part 2
    let part_2_target = 19690720;
    // TODO LEARNING POINT -  named nested loops with syntax 'name: for...
    'noun: for noun in (1..100) {
        let mut iter_inp = inp.clone();
        iter_inp[1] = noun;
        'verb: for verb in (1..100) {
            iter_inp[2] = verb;
            let result = opcode_computer(&iter_inp);
            if result[0]  == part_2_target {
                println!("found for noun={}, verb={}", noun, verb);
                println!("100 * noun + verb={}", 100 * noun + verb);
                break 'noun;
            }
        }
    }


}

// --------------------------------------------------------------------------------
// paramterised tests
macro_rules! testit {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, opcode_computer(input));
        }
    )*
    }
}

testit! {
    testit1: (vec![1,0,0,0,99], vec![2,0,0,0,99]),
    // (vec![2,3,0,3,99], vec![2,3,0,6,99]),
    // (vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801]),
    // (vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99]),
}

#[test]
fn test_opcode_computer() {
    let cases = vec![
        // (vec![1,0,0,0,99], vec![2,0,0,0,99]),
        (vec![2,3,0,3,99], vec![2,3,0,6,99]),
        (vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801]),
        (vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99]),
    ];

    for (initial, after) in cases {
        assert_eq!(opcode_computer(initial), after)
    }
    // assert_eq!(true, false)
}
/*
- step by and enumerate
for (i, x) in (1..10).enumerate().step_by(2) {
        println!("i: {}, x: {}", i, x);
    }

- final is a "reserved keyword". not in use yet but reserved for future use
https://doc.rust-lang.org/reference/keywords.html#reserved-keywords

- borrowed (e.g. &Vec) function argument types
https://stackoverflow.com/questions/40006219/why-is-it-discouraged-to-accept-a-reference-to-a-string-string-vec-vec-o

- borrows occur as iterate
    for (idx, x) in out.iter().enumerate().step_by(4) {

*/