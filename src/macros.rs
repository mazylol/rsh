#[macro_export]
macro_rules! alias_hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = std::collections::HashMap::new();
        $( map.insert(String::from($key), String::from($val)); )*
        map
    }}
}

#[macro_export]
macro_rules! cmd_hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map: std::collections::HashMap<&'static str, fn(&Vec<&str>)> = HashMap::new();
        $( map.insert($key, $val); )*
        map
    }}
}