
// TODO - can share macro with main.rs but not with other modules, e.g. dayX.rs

// there is no single one-liner hashmap creation
// https://stackoverflow.com/a/28392068/3596968
// let counts = hashmap! {"U" => (0, 1),
//             "D" => (0, -1),
//             "R" => (1, 0),
//             "L" => (-1, 0)
//             };


#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

// https://doc.rust-lang.org/std/collections/struct.HashMap.html#examples
// let timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
// .iter().cloned().collect();
